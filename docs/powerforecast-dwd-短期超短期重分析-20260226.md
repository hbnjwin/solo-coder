# 金风 `powerforecast`（`dwd`）短期/超短期重分析（2026-02-26）

## 1. 涉及表总览（放在最前）

| 类别 | 表名 | `power.sql`位置 | 结构定位 | 本次查询现状（来自 `power-tables.md`） |
|---|---|---|---|---|
| 短期基础 | `dwd.dwd_forecast_wf_short_term` | `25985` | 短期基础分区表 | 时间范围计数为 `0` |
| 短期主表（新） | `dwd.dwd_forecast_wf_short_term_partition` | `26267` / `26294` | 短期主结果（新） | 时间范围计数 `9386`；`forecast_date='2026-02-27 00:00:00'` 计数 `1196` |
| 短期多源主表（新） | `dwd.dwd_multi_forecast_wf_short_term_partition` | `32202` / `32228` | 多源短期主结果（新） | 文档记录：单时刻（例 `2026-02-27 00:00:00`）约 `230` 条 |
| 超短期主表 | `dwd.dwd_forecast_wf_ultra_short_term` | `30127` / `30203` | 超短期主结果 | 时间范围计数 `70` |
| 超短期快照 | `dwd.dwd_forecast_wf_ultra_short_term_snapshot` | `31904` | 超短期快照留痕 | 本次未给出计数 |
| 超短期多源主表 | `dwd.dwd_multi_forecast_wf_ultra_short_term` | `35699` | 多源超短期主结果 | 时间范围计数 `35` |

补充：
- `fore_wf_short_power` / `fore_wf_ultra_power` 在 `power.sql` 中是 `data_type` 取值，不是独立表名（`11805`、`19813`）。
- 你查询报错的 `dwd.dwd_forecast_wf_ultra_short_term_partition` 在当前库中确实不存在独立同名父表；应查询 `dwd.dwd_forecast_wf_ultra_short_term`（其分区子表在 `power.sql` 中是 `_partition_YYYY_MM` 命名）。

## 2. 短期与超短期结构说明（核心字段）

## 2.1 短期主线

### `dwd.dwd_forecast_wf_short_term`（基础）
- 关键字段：`wf_id`, `data_type`, `interval`, `record_date`, `forecast_date`, `forecast_end_date`, `forecast_batch`, `forecast_length`, `data_value(jsonb)`。
- 含义：按预测发布批次保存短期数据，`data_value` 为点位序列（JSON）。

### `dwd.dwd_forecast_wf_short_term_partition`（主表/新）
- 比基础表更偏“结果汇总输出”，保留 `data_value` 并新增 `cumulative_value`（累计值）。
- 典型用于业务查询与展示汇总。

### `dwd.dwd_multi_forecast_wf_short_term_partition`（多源主表/新）
- 在短期主表基础上增加多源维度：`forecast_model_config_id`, `model_source`。
- 适合看不同模型/来源并存结果。

## 2.2 超短期主线

### `dwd.dwd_forecast_wf_ultra_short_term`（主表）
- 关键字段：`wf_id`, `data_type`, `interval`, `record_date`, `forecast_date`, `forecast_batch`, `forecast_length`, `data_value(jsonb)`。
- 字段注释明确：`forecast_length` 为“预测时长（小时）”。

### `dwd.dwd_forecast_wf_ultra_short_term_snapshot`（快照）
- 关键字段：`data_value`, `data_source`, `remark`, `forecast_date`, `forecast_length`。
- 用于留痕（正常预测/校正/人工干预等快照）。

### `dwd.dwd_multi_forecast_wf_ultra_short_term`（多源主表）
- 超短期主表的多源版本，新增 `forecast_model_config_id`, `model_source`。

## 3. 对你本次查询结果的解读

1. 之前“未发现短期/超短期预测表”的结论应修正为：  
   在 `dwd` schema 已发现并查询到短期/超短期预测相关主表数据。
2. 短期基础表 `dwd_forecast_wf_short_term` 在给定时间范围为 `0`，但短期主表（新）`dwd_forecast_wf_short_term_partition` 有大量数据（`9386`）。
3. 超短期主表 `dwd_forecast_wf_ultra_short_term` 有数据（`70`）；多源超短期 `dwd_multi_forecast_wf_ultra_short_term` 也有数据（`35`）。
4. 单个 `forecast_date` 出现大量行（如 `1196`、`230`）是合理的，通常由多维组合导致：`wf_id + data_type + forecast_batch + model/source + tier` 等。
5. 你的样例查询显示 `data_value` 为数组/JSON 序列，这与表结构设计一致，说明该库以“序列值”而非单点行方式存储预测曲线。

## 4. 结论（汇报可直接使用）

1. 金风 `powerforecast` 库中，短期/超短期预测表是存在的，核心位于 `dwd` schema。  
2. 业务上应优先关注：  
   - 短期：`dwd_forecast_wf_short_term_partition`（及多源表）  
   - 超短期：`dwd_forecast_wf_ultra_short_term`（及多源表）  
3. `fore_wf_short_power` / `fore_wf_ultra_power` 是 `data_type` 分类值，不是物理表。  
4. 当前数据现状显示：短期“基础表空、主表有数据”的迁移特征明显；超短期主表与多源表均已有数据。

## 5. 建议补充核查SQL（用于进一步锁定“功率预测”）

```sql
-- 短期主表：按 data_type 统计
SELECT data_type, COUNT(*) AS cnt
FROM dwd.dwd_forecast_wf_short_term_partition
WHERE forecast_date BETWEEN '2026-02-26 08:00:00' AND '2026-03-13 18:00:00'
GROUP BY data_type
ORDER BY cnt DESC;
```

```sql
-- 超短期主表：按 data_type 统计
SELECT data_type, COUNT(*) AS cnt
FROM dwd.dwd_forecast_wf_ultra_short_term
WHERE forecast_date BETWEEN '2026-02-26 08:00:00' AND '2026-03-13 18:00:00'
GROUP BY data_type
ORDER BY cnt DESC;
```

```sql
-- 单时刻为何多行：看维度拆分
SELECT forecast_date, wf_id, data_type, forecast_batch, model_source, COUNT(*) AS cnt
FROM dwd.dwd_forecast_wf_short_term_partition
WHERE forecast_date='2026-02-27 00:00:00'
GROUP BY forecast_date, wf_id, data_type, forecast_batch, model_source
ORDER BY cnt DESC;
```

## 6. 附录：有数据表结构字段中文说明

说明：以下字段中文释义优先采用 `power.sql` 中 `COMMENT ON COLUMN`，类型来自 `CREATE TABLE`。

### 6.1 `dwd.dwd_forecast_wf_short_term_partition`（短期主表，已查有数据）

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

### 6.2 `dwd.dwd_multi_forecast_wf_short_term_partition`（短期多源主表，已查有数据）

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

### 6.3 `dwd.dwd_forecast_wf_ultra_short_term`（超短期主表，已查有数据）

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

### 6.4 `dwd.dwd_multi_forecast_wf_ultra_short_term`（超短期多源主表，已查有数据）

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
