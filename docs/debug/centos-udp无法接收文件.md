
你这个现象里有两个关键点：

--check-only 的 UDP status=OK 不能证明目标服务收到了包
当前实现只是本机 send() 成功就算 OK，不是应用层确认（见 src/main.rs:1950）。
你如果用 dist/run.sh 跑，默认不带 --send-query-json
我已改成可配置：SEND_QUERY_JSON=1 ./run.sh（见 scripts/run.sh）。
另外我加了发送端日志，执行 --send-query-json 时会打印：

[query_json] source=... key=... bytes=...（见 src/main.rs:2032）
先按这组命令在 CentOS 核验：

# 接收端（192.168.0.113）
pkill -f data-import-sql || true
./data-import-sql --config ./config.ini --serve --udp-listen 0.0.0.0:8075 --server-save-dir ./server_output
新开一个终端看监听和防火墙：

ss -lunp | grep 8075
firewall-cmd --query-port=8075/udp
# 若未放行：
firewall-cmd --add-port=8075/udp --permanent && firewall-cmd --reload
发送端执行：

./data-import-sql --config ./config.ini --send-test-packet
./data-import-sql --config ./config.ini --output ./output --send-query-json
接收端查看：

tail -n 30 server_output/udp_received.ndjson
tail -n 30 server_output/udp_chunk_metrics.ndjson
ls -lh server_output/query_results/*super_short_term*
如果你是用脚本跑发送端，记得：

SEND_QUERY_JSON=1 ./run.sh
如果还不行，把这三段贴我：

发送端 [query_json] ... bytes=... 的日志
udp_chunk_metrics.ndjson 最后 30 行
ss -lunp | grep 8075 输出



## 调试1
elnet 113 8075 不推荐，用它测的是 TCP，对 UDP 8075 基本没有意义。

建议这样测，最可靠：

113 端先明确用 IPv4 监听并抓包
./data-import-sql --config ./config.ini --serve --udp-listen 0.0.0.0:8075 --server-save-dir ./server_output
ss -lunp | grep 8075
sudo tcpdump -ni any udp port 8075
110 端发最小 UDP 探测包
echo -n 'udp_probe_20260305' > /dev/udp/192.168.0.113/8075
# 或
printf 'udp_probe_20260305' | nc -u -w1 192.168.0.113 8075
113 端同时看应用日志
tail -f server_output/udp_received.ndjson
结果判断：

tcpdump 都看不到包：二区->三区 UDP 策略/ACL/路由问题。
tcpdump 有包但应用无日志：服务绑定/实例不对（重点确认就是这个进程在收 8075）。
小包能收，大 query_json 不能收：就是分片/限速丢包（链路对大量 UDP 分片不友好）。
你现在这个现象里，最关键是先用上面第 1~3 步把“包是否到 113 网卡”定死。

## 调试2
是的，这个 udp: bad length 1672 > 1472 很可能就是主因。
你不需要“增大包”测试了，证据已经说明包太大触发了 IP 分片。

1472 是 MTU1500 下单个 UDP 报文可承载上限（1500-20-8）。
1672 > 1472 说明该 UDP 报文超过链路上限，被 IP 分片。
正向隔离/防火墙设备常见会丢 UDP 分片或高突发分片，导致应用层无法完整重组。
另外，这不等于“所有包都没收到”：

你的小包 udp_probe、--send-test-packet 能到，说明链路是通的。
--send-query-json 里只有部分分片丢，就会导致整条大消息失败（看起来像“收不到文件”）。
建议马上做两件事：

在 113 看 udp_chunk_metrics.ndjson 是否有 stale_drop/invalid_chunk（这就是分片丢失证据）。
把发送分片尺寸显著调小（例如 1200 -> 400/512），并增加节流，再测。
如果你同意，我可以直接帮你改代码为“按实际序列化后字节数限长分片 + 可配置 chunk 大小”，专门适配你这条二区->三区链路。

## 解决
以这样解决，我已经先在代码里做了稳妥修复：

已修改发送分片策略（本地代码）
把 UDP 分片数据块从 1200 降到 512，避免 JSON 转义后报文超过 MTU。
把发送节流从“每 64 包暂停 2ms”改成“每 8 包暂停 10ms”，降低隔离链路突发丢包。
修改位置：src/main.rs (line 144)
我已本地编译通过（cargo build -q）。

你现在要做的
在 110 重新编译并替换二进制（发送端必须更新）。
113 启动服务时用绝对目录，避免“看错目录”：
./data-import-sql --serve --udp-listen 0.0.0.0:8075 --server-save-dir /tmp/diq_server_output
110 发送 --send-query-json 后，在 113 同时看：
tail -f /tmp/diq_server_output/udp_received.ndjson
tail -f /tmp/diq_server_output/udp_chunk_metrics.ndjson
sudo tcpdump -ni any udp port 8075
判定标准
若 udp_received.ndjson 和 udp_chunk_metrics.ndjson 开始有记录，问题就解决了。
若仍有 stale_drop，继续把分片再降（如 256）或改走 TCP（最稳）。


## 根本解决
若本机可收、110 发不过来：查防火墙/策略
sudo firewall-cmd --query-port=8075/udp
sudo firewall-cmd --add-port=8075/udp --permanent && sudo firewall-cmd --reload
若你必须后台跑，改成行缓冲日志
nohup stdbuf -oL -eL ./data-import-sql --serve --udp-listen 0.0.0.0:8075 --server-save-dir /tmp/diq_server_output >/tmp/diq_server.log 2>&1 &
你现在最关键的是第 2 步：127.0.0.1 -> 8075 自发包是否能触发日志。这个结果能直接定性。


# 查看当前是否已放通
sudo firewall-cmd --query-port=8071/tcp

# 永久放通并重载
sudo firewall-cmd --add-port=8071/tcp --permanent
sudo firewall-cmd --reload

# 再确认
sudo firewall-cmd --query-port=8071/tcp



sudo tcpdump -ni any udp port 8075

sudo tcpdump -ni any tcp port 8071
