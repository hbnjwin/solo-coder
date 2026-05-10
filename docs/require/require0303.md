我想在 centos 服务器上，安装 能运行 rust 的程序，需要能访问 二区的其他服务器的 mysql/pg 数据库，并查询数据 通过tcp/udp传输及本地导出csv文件；
配置文件config.ini

[tcpserver]
enable=1
host=172.23.24.140
port=8071
[udpserver]
enable=1
host=172.23.24.140
port=8072
[data-huailin]
enable=1
type=pg
host=172.17.0.10
port=35432
user=powerforecast
# password=pf@gwV632023
sql_short_term=SELECT * FROM hdrfcstdata_hxgf WHERE fcst_time between '2026-03-01 00:00:00' and '2026-03-03 00:00:00'
sql_super_short_term=SELECT * FROM hdrsfcstdata_hxgf WHERE fcst_time between '2026-03-01 00:00:00' and '2026-03-03 00:00:00'
  

[data-hexu]
enable=1
type=mysql
host=172.17.108.30
port=3306
user=sa 
password=cast1234
sql_short_term=SELECT * FROM dwd.dwd_forecast_wf_short_term_partition WHERE forecast_date between '2026-03-01 00:00:00' and '2026-03-03 00:00:00'
sql_super_short_term=SELECT * FROM dwd.dwd_forecast_wf_ultra_short_term WHERE forecast_date between '2026-03-01 00:00:00' and '2026-03-03 00:00:00'



CREATE TABLE dwd_forecast_wf_short_term_partition (
    id bigint NOT NULL,
    wf_id bigint NOT NULL,
    tier character varying,
    data_type character varying,
    record_date timestamp(6) without time zone,
    forecast_date timestamp(6) without time zone NOT NULL,
    forecast_batch character varying,
    model_id bigint,
    model_version character varying,
    model_source character varying,
    data_value jsonb,
    cumulative_value numeric(20,4),
    is_backup smallint,
    create_time timestamp(6) without time zone DEFAULT now(),
    update_time timestamp(6) without time zone,
    source_batch character varying
)
PARTITION BY RANGE (forecast_date);

CREATE TABLE dwd_forecast_wf_ultra_short_term (
    id bigint NOT NULL,
    wf_id bigint NOT NULL,
    data_type character varying(255),
    "interval" integer,
    record_date timestamp(6) without time zone,
    forecast_date timestamp(6) without time zone NOT NULL,
    forecast_batch character varying,
    forecast_length integer,
    model_id bigint,
    model_version character varying,
    model_source character varying,
    data_value jsonb,
    is_backup smallint DEFAULT 0,
    create_time timestamp(6) without time zone DEFAULT now(),
    update_time timestamp(6) without time zone,
    source_batch character varying
)
PARTITION BY RANGE (forecast_date);



## power相关表结构
##  `dwd.dwd_forecast_wf_short_term_partition`（短期主表，已查有数据）

| 字段 | 类型 | 中文说明 |
|---|---|---|
| `id` | `bigint` | 雪花id |
| `wf_id` | `bigint` | 电场id |
| `tier` | `varchar` | 层（分层数据层级，不分层设0，最高层-1） |
| `data_type` | `varchar` | 数据类型（如预测指标分类） |
| `record_date` | `timestamp` | 记录时间 |
| `forecast_date` | `timestamp` | 预测起始时间 |
| `forecast_batch` | `varchar` | 预测批次 |
| `model_id` | `bigint` | 预测模型id |
| `model_version` | `varchar` | 模型程序版本 |
| `model_source` | `varchar` | 预测来源（forecast/transaction/modify） |
| `data_value` | `jsonb` | 一天的点位序列（通常96点；若含次日00:00端点则为97点） |
| `cumulative_value` | `numeric(20,4)` | 累计值 |
| `is_backup` | `smallint` | 是否备份 |
| `create_time` | `timestamp` | 创建时间 |
| `update_time` | `timestamp` | 更新时间 |
| `source_batch` | `varchar` | 原始批次（如00/12） |

###  `dwd.dwd_multi_forecast_wf_short_term_partition`（短期多源主表，已查有数据）

| 字段 | 类型 | 中文说明 |
|---|---|---|
| `id` | `bigint` | 雪花id |
| `wf_id` | `bigint` | 电场id |
| `tier` | `varchar` | 层（分层数据） |
| `data_type` | `varchar` | 数据类型 |
| `record_date` | `timestamp` | 记录时间 |
| `forecast_date` | `timestamp` | 预测起始时间 |
| `forecast_batch` | `varchar` | 预测批次 |
| `forecast_model_config_id` | `bigint` | 预测模型ID |
| `data_value` | `jsonb` | 点位数据 |
| `cumulative_value` | `numeric(20,4)` | 累计值 |
| `is_backup` | `smallint` | 是否备份 |
| `create_time` | `timestamp` | 创建时间 |
| `update_time` | `timestamp` | 更新时间 |
| `source_batch` | `varchar` | 原始批次（如00/12） |
| `model_source` | `varchar` | 预测来源优先级（forecast < transaction < modify） |

###  `dwd.dwd_forecast_wf_ultra_short_term`（超短期主表，已查有数据）

| 字段 | 类型 | 中文说明 |
|---|---|---|
| `id` | `bigint` | 雪花id |
| `wf_id` | `bigint` | 电场id |
| `data_type` | `varchar(255)` | 数据类型 |
| `interval` | `integer` | 数据间隔（分钟，15表示逐15分钟） |
| `record_date` | `timestamp` | 记录时间 |
| `forecast_date` | `timestamp` | 预测时间 |
| `forecast_batch` | `varchar` | 预测批次 |
| `forecast_length` | `integer` | 预测时长（小时） |
| `model_id` | `bigint` | 模型id |
| `model_version` | `varchar` | 模型程序版本（非预测版本） |
| `model_source` | `varchar` | 模型来源 |
| `data_value` | `jsonb` | 点位序列数据 |
| `is_backup` | `smallint` | 是否备份 |
| `create_time` | `timestamp` | 创建时间 |
| `update_time` | `timestamp` | 更新时间 |
| `source_batch` | `varchar` | 原始批次（如00/12） |

###  `dwd.dwd_multi_forecast_wf_ultra_short_term`（超短期多源主表，已查有数据）

| 字段 | 类型 | 中文说明 |
|---|---|---|
| `id` | `bigint` | 雪花id |
| `wf_id` | `bigint` | 电场id |
| `data_type` | `varchar` | 数据类型 |
| `interval` | `integer` | 间隔（15/60/360） |
| `record_date` | `timestamp` | 记录时间 |
| `forecast_date` | `timestamp` | 预测起始时间 |
| `forecast_batch` | `varchar` | 预测批次 |
| `forecast_length` | `integer` | 预测时长（天） |
| `forecast_model_config_id` | `bigint` | 预测模型id |
| `data_value` | `jsonb` | 点位序列数据 |
| `is_backup` | `smallint` | 是否备份 |
| `create_time` | `timestamp` | 创建时间 |
| `update_time` | `timestamp` | 更新时间 |
| `source_batch` | `varchar` | 原始批次（如00/12） |
| `model_source` | `varchar` | 预测来源优先级（forecast < transaction < modify） |


## davinci 相关表结构

CREATE TABLE `hdrfcstdata_hxgf` (
  `id` int NOT NULL,
  `record_id` int DEFAULT NULL,
  `fcst_time` datetime DEFAULT NULL,
  `fcst_nodetype` smallint DEFAULT NULL,
  `limit_upper` double(38,6) DEFAULT NULL,
  `limit_lower` double(38,6) DEFAULT NULL,
  `p` double(38,6) DEFAULT NULL,
  `daytype` smallint DEFAULT NULL,
  `is_modify` smallint DEFAULT NULL,
  `is_main` int DEFAULT NULL,
  `powertype` int DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `hdrfcstdata_HXGF_fcsttime_powertype_daytype_ismain_fcstnodetype` (`fcst_time`,`powertype`,`daytype`,`is_main`,`fcst_nodetype`)
) ENGINE=InnoDB DEFAULT CHARSET=gbk;


CREATE TABLE `hdrsfcstdata_hxgf` (
  `id` int NOT NULL,
  `record_id` int DEFAULT NULL,
  `fcst_time` datetime DEFAULT NULL,
  `fcst_nodetype` smallint DEFAULT NULL,
  `limit_upper` double(38,6) DEFAULT NULL,
  `limit_lower` double(38,6) DEFAULT NULL,
  `p` double(38,6) DEFAULT NULL,
  `pointtype` smallint DEFAULT NULL,
  `is_modify` smallint DEFAULT NULL,
  `powertype` int DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `record_id` (`record_id`,`fcst_time`)
) ENGINE=InnoDB DEFAULT CHARSET=gbk;

