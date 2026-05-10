# 国能日新 `davinci` 数据库短期/超短期功率预测分析汇报

## 1. 汇报结论

1. `davinci` 库中，短期与超短期预测均存在“三套表”模式：基础表、`_hxgf` 数值表、`_jm_hxgf` 二进制密文表。  
2. 当前现场查询结论显示：  
   - 短期基础表 `hdrfcstdata` 为空；实际业务数据在 `hdrfcstdata_hxgf` / `hdrfcstdata_jm_hxgf`。  
   - 超短期基础表 `hdrsfcstdata` 为空；实际业务数据在 `hdrsfcstdata_hxgf` / `hdrsfcstdata_jm_hxgf`。  
3. `*_jm_hxgf` 表中的 `p/limit_upper/limit_lower` 为 `binary(24)`，属于密文或二进制编码数据；统计分析应优先使用 `*_hxgf` 数值表。  
4. 查询分析时必须带齐业务口径（如 `powertype`、`is_main`、`fcst_nodetype`、`daytype/pointtype`），避免将多维版本混算导致结果偏差。  

## 2. 数据来源与依据

- 结构依据：`docs/davinci.sql`
- 现场查询依据：你提供的 MySQL 截图与会话结果（空表、63条、11条等）
- 时间背景：2026-02-26 的现场查询结果

## 3. 关键表清单（短期/超短期）

| 业务类型 | 表名 | 结构位置 | 关键字段类型 | 当前数据状态（现场） | 说明 |
|---|---|---|---|---|---|
| 短期 | `hdrfcstdata` | `davinci.sql:25248` | `p float(12,3)` | 空 | 历史基础表 |
| 短期 | `hdrfcstdata_hxgf` | `davinci.sql:25271` | `p double(38,6)` | 有数据 | 短期主分析表（数值） |
| 短期 | `hdrfcstdata_jm_hxgf` | `davinci.sql:25295` | `p binary(24)` | 有数据 | 短期加密/二进制表 |
| 超短期 | `hdrsfcstdata` | `davinci.sql:26059` | `p float(12,3)` | 空 | 历史基础表 |
| 超短期 | `hdrsfcstdata_hxgf` | `davinci.sql:26081` | `p double(38,6)` | 有数据 | 超短期主分析表（数值） |
| 超短期 | `hdrsfcstdata_jm_hxgf` | `davinci.sql:26104` | `p binary(24)` | 有数据 | 超短期加密/二进制表 |
| 超短期当前 | `hdrsfcstcurrentdata` | `davinci.sql:26035` | `p binary(24)` | 需现场确认 | 注释为“超短期当前预测数据表” |

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

## 4. 结构证据（用于汇报可追溯）

### 4.1 短期归属证据

- `hdrfcstdata_modify_backup` 注释为“短期预测数据修改备份表”：`davinci.sql:25319`
- `hdrfcstdata_preset_coeffient` 注释为“短期预测系数预设置表”：`davinci.sql:25340`

### 4.2 超短期归属证据

- `hdrsfcstcurrentdata` 注释为“超短期当前预测数据表”：`davinci.sql:26035`
- `hdrsfcstdata_modify` 注释为“超短期预测数据预设置表”：`davinci.sql:26127`
- `hdrsfcstdata_modify_backup` 注释为“超短期预测数据修改备份表”：`davinci.sql:26143`

### 4.3 记录主表（用于解释 `record_id`）

- 短期记录表：`hdrfcstrecord`（含 `record_time/fcst_time/is_main/powertype`）：`davinci.sql:25381`
- 超短期记录表：`hdrsfcstrecord`（含 `record_time/fcst_time/is_main/powertype`）：`davinci.sql:26176`

## 5. 字段与口径差异说明

1. 短期表使用 `daytype` 作为点位/分组维度；超短期表使用 `pointtype`。  
2. 短期 `hdrfcstdata_hxgf` 有组合索引：  
   `fcst_time,powertype,daytype,is_main,fcst_nodetype`（`davinci.sql:25284`）。  
3. 超短期 `hdrsfcstdata_hxgf` 的唯一键是：  
   `record_id,fcst_time`（`davinci.sql:26093`）。  
4. `*_jm_hxgf` 的 `binary(24)` 字段通常以 `0x...` 或不可读串显示，不适合直接数值聚合。  

## 6. 已观测数据现象（现场）

1. 在 `fcst_time='2026-02-26 09:00:00'` 条件下，短期表 `hdrfcstdata_hxgf` 出现 63 条记录。  
2. 维度分布示例：`daytype=11`、`is_main=2`、`powertype=3`、`fcst_nodetype=1`、`record_id` 基本逐条不同。  
3. 若直接 `avg(p)`，会把多版本、多口径数据混合，得到非展示口径的均值（如 15.42）。  
4. 增加 `is_main=1 and fcst_nodetype=1 and powertype=0` 后，仍有 11 条，说明仍是多个 `daytype` 点，不是单点值。  

## 7. 查询建议（生产与汇报推荐）

### 7.1 先确认是否“空基础表 + 有业务表”

```sql
SELECT 'hdrfcstdata' AS t, COUNT(*) AS c FROM hdrfcstdata
UNION ALL
SELECT 'hdrfcstdata_hxgf', COUNT(*) FROM hdrfcstdata_hxgf
UNION ALL
SELECT 'hdrfcstdata_jm_hxgf', COUNT(*) FROM hdrfcstdata_jm_hxgf
UNION ALL
SELECT 'hdrsfcstdata', COUNT(*) FROM hdrsfcstdata
UNION ALL
SELECT 'hdrsfcstdata_hxgf', COUNT(*) FROM hdrsfcstdata_hxgf
UNION ALL
SELECT 'hdrsfcstdata_jm_hxgf', COUNT(*) FROM hdrsfcstdata_jm_hxgf;
```

### 7.2 短期：按业务口径取数（不要直接平均全部63条）

```sql
SELECT
  fcst_time, daytype, p, limit_lower, limit_upper, record_id
FROM hdrfcstdata_hxgf
WHERE fcst_time = '2026-02-26 09:00:00'
  AND fcst_nodetype = 1
  AND is_main = 1
  AND powertype = 0
ORDER BY daytype, record_id DESC;
```

### 7.3 超短期：使用 `pointtype` 口径取数

```sql
SELECT
  fcst_time, pointtype, p, limit_lower, limit_upper, record_id
FROM hdrsfcstdata_hxgf
WHERE fcst_time = '2026-02-26 09:00:00'
  AND fcst_nodetype = 1
  AND powertype = 0
ORDER BY pointtype, record_id DESC;
```

### 7.4 验证明文表与密文表行级对应关系

```sql
SELECT COUNT(*) AS join_cnt
FROM hdrfcstdata_hxgf a
JOIN hdrfcstdata_jm_hxgf b ON a.id = b.id
WHERE a.fcst_time = '2026-02-26 09:00:00';
```

## 8. 风险与后续建议

1. 风险：将多维版本数据直接 `avg`，会导致与网页展示值不一致。  
2. 风险：误用 `*_jm_hxgf` 参与数值统计，会引入二进制解析误差或不可读结果。  
3. 建议：在接口或报表中固定统一口径（至少固定 `powertype/is_main/fcst_nodetype/daytype或pointtype`）。  
4. 建议：若需与网页逐点对齐，补充抓取网页接口参数，核实是否存在平滑、插值或时间偏移策略。  

## 9. 结论（可直接用于汇报发言）

`davinci` 库的短期/超短期预测数据主存储已切换到 `*_hxgf`（数值）与 `*_jm_hxgf`（密文）两套并行表，传统基础表 `hdrfcstdata`、`hdrsfcstdata` 当前为空。  
后续分析和对外报表应统一使用 `*_hxgf` 并严格限定业务口径，避免“同一时刻多维版本混算”造成结论偏差。
