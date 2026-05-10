# 110 机器部署指南（CentOS 7，SSH-only）

本文针对你描述的 `192.168.0.110` 终端环境，目标是部署单二进制程序 `data-import-sql`。

## 1. 在 110 上准备 Rust（仅首次）

```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"
rustc -V
cargo -V
```

如果公司网络限制外网，可在可联网机器先编译后上传 `dist` 目录。

## 2. 上传代码

把本仓库同步到 110 机器，例如：
```bash
scp -r data-import-sql windos@192.168.0.110:/home/windos/
```

## 3. 在 110 机器构建

```bash
cd /home/windos/data-import-sql
bash scripts/build-centos.sh
```

构建完成后得到：
- `dist/data-import-sql`（主程序）
- `dist/config.ini`（配置模板）
- `dist/run.sh`（启动脚本）

## 4. 修改配置

编辑 `dist/config.ini`：
- 每个 `data-*` 段填写目标库连接信息
- 填写 `sql_*` 查询语句
- 如果要上报执行结果，设置 `tcpserver/udpserver`

建议把敏感密码改成环境变量替换流程，不要长期明文保存在仓库。

## 5. 执行

```bash
cd dist
./data-import-sql --config ./config.ini --check-only
./run.sh
```

或手工执行：
```bash
./data-import-sql --config ./config.ini --output ./output
```

## 6. 验证结果

```bash
ls -lh output
```

预期：
- 生成 CSV 文件
- 终端打印每条 SQL 的行数和状态
- 若启用了 TCP/UDP，上报目标能收到 JSON 行

## 7. TCP/UDP 收包联调（可选）

在接收端启动调试服务（前台打印 payload）：
```bash
./data-import-sql --serve --tcp-listen 0.0.0.0:8071 --udp-listen 0.0.0.0:8072
```
也可以直接读取 `config.ini` 的端口：
```bash
./data-import-sql --config ./config.ini --serve
```

在发送端触发测试包：
```bash
./data-import-sql --config ./config.ini --send-test-packet
```

## 8. 做成定时任务（可选）

```bash
crontab -e
```

示例（每天 01:10 执行一次）：
```cron
10 1 * * * cd /home/windos/data-import-sql/dist && ./run.sh >> job.log 2>&1
```
