# data-import-sql
##  在 macOS 上使用 CentOS 容器编译
docker run --rm -v "$(pwd)":/home/rust/src messense/rust-musl-cross:x86_64-musl cargo build --release

docker run --rm -v "$(pwd)":/home/rust/src messense/rust-musl-cross:x86_64-musl \
  cargo build --release --target x86_64-unknown-linux-musl


适用于 SSH 场景的 Rust 终端程序：
- 从 `config.ini` 读取多个数据源（PostgreSQL / MySQL）
- 执行 `sql_*` 查询并导出 CSV
- 将执行结果通过 TCP/UDP 上报（JSON 一行一条）
- 单二进制运行，不需要 GUI

## 为什么不用 Tauri

Tauri 是桌面 GUI 框架，适合有图形界面的客户端。  
你的 `110` 机器仅能 SSH 终端访问，Tauri 不能发挥价值，且包体和运行复杂度更高。  
本项目改用纯 Rust CLI，更符合“终端可运行、体积小、免额外依赖”的目标。

## 快速开始

macOS 本机测试：
```bash
cargo build --release
cp config.example.ini config.ini
./target/release/data-import-sql --config ./config.ini --check-only
```

1. 准备配置文件：
```bash
cp config.example.ini config.ini
```
2. 修改 `config.ini` 的数据库地址、账号、密码和 SQL。
3. 编译并运行：
```bash
cargo build --release
./target/release/data-import-sql --config ./config.ini --output ./output
```

先只做连通性测试（不执行业务 SQL）：
```bash
./target/release/data-import-sql --config ./config.ini --check-only
```

本地启动 TCP/UDP 测试服务端（控制台打印收到的数据）：
```bash
./target/release/data-import-sql --serve --tcp-listen 0.0.0.0:8071 --udp-listen 0.0.0.0:8072
```
或直接按 `config.ini` 里的 `tcpserver/udpserver` 启动：
```bash
./target/release/data-import-sql --config ./config.ini --serve
```
默认会把接收内容落盘到 `server_output/` 下：
- `server_output/tcp_received.ndjson`
- `server_output/udp_received.ndjson`
- 当收到 `kind=query_json` 包时，会自动拆分保存到：
  - `server_output/query_results/<source>_<query_key>_<timestamp>.json`
  - `server_output/query_results/<source>_<query_key>_<timestamp>.csv`

向配置中的 `tcpserver/udpserver` 发送测试包：
```bash
./target/release/data-import-sql --config ./config.ini --send-test-packet
```

将每条查询结果（JSON文件内容）也转发到 `tcpserver/udpserver`：
```bash
./target/release/data-import-sql --config ./config.ini --output ./output --send-query-json
```

批量导入 CSV 到配置中的目标库（PG/MySQL）：
```bash

```
导入规则：
- 仅处理 `enable=1` 的 `data-*` 段
- 对每个 `sql_*`，从 SQL 解析目标表
- 在导入目录中选择同 `query_key` 且表头字段能匹配目标表的 CSV（自动选最新时间戳文件）
- 导入按行执行并容错：单行失败会 `skipped`，其余行继续；`<unsupported:...>` 会按 `NULL` 处理

插入四张测试表模拟数据（PG+MySQL）：
```bash
# PostgreSQL
psql -h 127.0.0.1 -p 5432 -U postgres -d powerforecast -f scripts/mock/pg_hxgf_mock.sql

# MySQL
mysql -h 127.0.0.1 -P 3306 -u root -p < scripts/mock/mysql_dwd_mock.sql
```

也可以使用脚本（CentOS）：
```bash
bash scripts/build-centos.sh
cd dist
./run.sh
```

## 配置说明

- `tcpserver` / `udpserver`
  - `enable=1` 表示启用上报
  - `host` + `port` 为上报目标
  - 可用 `--send-test-packet` 验证传输是否可达
- `paraset`
  - `chunkSize`：UDP 分片单片数据大小（字节）
  - 可选范围 `64~1024`，默认 `512`
  - 建议在隔离链路使用 `256~512`，避免出现 `udp bad length` 或分片丢包
- `data-*` 段
  - `enable=1` 启用该数据源
  - `type=pg` 或 `type=mysql`
  - `database` 可选（MySQL 可留空）
  - `sql_*` 任意数量，程序会全部执行并分别导出 CSV

## 输出

- CSV 文件：`output/<section>_<sql_key>_<timestamp>.csv`
- JSON 文件：`output/<section>_<sql_key>_<timestamp>.json`
- 终端摘要：每个 SQL 的成功/失败和行数
- 启动状态检查：程序启动时会打印 `tcp/udp` 和所有 `enable=1` 的数据库连接状态
- TCP/UDP 上报内容示例：
```json
{
  "timestamp":"2026-03-03 09:30:00",
  "source":"data-huailin",
  "db_type":"postgres",
  "query_key":"sql_short_term",
  "row_count":128,
  "csv_file":"output/data-huailin_sql_short_term_20260303_093000.csv",
  "success":true,
  "error":null
}
```
./target/release/data-import-sql --config ./config.ini --import-csv-dir ./output/output-new