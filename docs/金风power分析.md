# 金风 `powerforecast` 数据库表对照分析（基于 `power.sql`）

## 1. 结论

结论：**在当前提供的金风 `powerforecast` 实际表清单中，未发现短期/超短期功率预测主表。**

说明：
- `power.sql` 中确实定义了短期/超短期预测表（位于 `dwd` schema）。
- 但当前 `power-tables.md` 中的实际库表清单未出现这些 `dwd` 预测表名。
- 因此本次核对结论为“当前实例未发现对应预测表”（至少在本次导出的清单范围内）。

## 2. `power.sql` 中的核心候选表（理论存在）

| 类别 | 表名 | `power.sql` 位置 | 备注 |
|---|---|---|---|
| 短期主表（新） | `dwd.dwd_forecast_wf_short_term_partition` | `26267`、`26294` | 表注释：短期预测表（新） |
| 超短期主表 | `dwd.dwd_forecast_wf_ultra_short_term` | `30127`、`30203` | 字段注释：预测时长（小时） |
| 短期基础表 | `dwd.dwd_forecast_wf_short_term` | `25985` | 分区表 |
| 超短期快照 | `dwd.dwd_forecast_wf_ultra_short_term_snapshot` | `31904` | 快照表 |
| 多源短期（新） | `dwd.dwd_multi_forecast_wf_short_term_partition` | `32202`、`32228` | 表注释：短期(多源)预测表(新) |
| 多源超短期 | `dwd.dwd_multi_forecast_wf_ultra_short_term` | `35699` | 分区表 |

补充：
- `fore_wf_short_power` / `fore_wf_ultra_power` 在 `power.sql` 中主要作为 `data_type` 取值出现，不是独立表名。  
  参考：`11805`、`19813`。

## 3. 当前实例实际表清单（本文件原始清单）

来源：现场导出的 `powerforecast` 表清单（本文件原始内容）。

```text
auth_about,auth_authentication_channel,auth_dict_data,auth_dict_type,auth_ex_role_group,auth_ex_role_group_detail,auth_group,auth_group_user,auth_i18n,auth_import_export_record,auth_import_mapping,auth_inspection_center_station,auth_login_credential,auth_login_credential_record,auth_login_ip,auth_login_time,auth_menu_info,auth_organization,auth_organization_field,auth_organization_mark,auth_organization_template,auth_organization_template_field,auth_policy,auth_policy_detail,auth_policy_info,auth_policy_role,auth_product,auth_product_authentication_channel,auth_product_info,auth_resource,auth_resource_type,auth_resource_type_action,auth_resource_type_info,auth_resource_type_product,auth_role,auth_role_bundle_policy_info,auth_role_bundle_user_info,auth_role_group,auth_role_info,auth_role_rolebucket,auth_role_user,auth_rolebucket,auth_sync_record,auth_tag,auth_tag_role,auth_tag_user,auth_tenant,auth_tenant_detail,auth_tenant_product,auth_tenant_role,auth_third_application_service,auth_third_application_service_role,auth_user,
auth_user_info,
collector_equipment,
collector_formulate,
collector_gradgroup,
collector_param,
collector_paramgroup,
flyway_schema_history,
gss_compute_cluster,
gss_dictionary,
gss_dictionarytype,
gss_docker_image,
gss_http_address,
gss_job_deploy,
gss_job_instance,
gss_job_plan,
gss_job_plan_tag,
gss_job_plan_tag_relation,
gss_job_slice,
gss_job_slice_result,
gss_jobprogram_common,
gss_jobprogram_http,
gss_jobprogram_k8s,
gss_jobprogram_script,
gss_jobprogram_spark,
gss_k8s_image_config,
gss_memory_cycle_config,
gss_model_template,
gss_package_tag_relation,
gss_prod_info,
gss_program_package,
gss_program_package_tag,
gss_resource_group,
gss_resource_instance,
gss_share_imageconfig,
gss_slice_config,
gss_slice_pre_config,
gss_user_parameter,
gss_user_parameter_tag,
gss_user_parameter_tag_relation,
gss_workflow,
gss_workflow_instance,
gss_workflow_node,
gss_workflow_tag,
gss_workflow_tag_relation,
logging_event,
logging_event_exception,
logging_event_property,
parts_collector_mapping,
scada_pathdescr,
scada_propattrs,
scada_wfinfo,
scada_wtinfo
```

## 4. 对照结果

核对上述“理论候选表”与“实际表清单”：
- 未出现 `dwd_forecast_wf_short_term_partition`
- 未出现 `dwd_forecast_wf_ultra_short_term`
- 未出现 `dwd_forecast_wf_short_term`
- 未出现 `dwd_forecast_wf_ultra_short_term_snapshot`
- 未出现 `dwd_multi_forecast_wf_short_term_partition`
- 未出现 `dwd_multi_forecast_wf_ultra_short_term`

因此当前可汇报结论为：**并未发现短期、超短期的功率预测表。**

## 5. 建议补充核查（避免 schema 视图差异）

若需要最终确认，可在现场库执行：

```sql
SELECT schemaname, tablename
FROM pg_tables
WHERE tablename IN (
  'dwd_forecast_wf_short_term_partition',
  'dwd_forecast_wf_ultra_short_term',
  'dwd_forecast_wf_short_term',
  'dwd_forecast_wf_ultra_short_term_snapshot',
  'dwd_multi_forecast_wf_short_term_partition',
  'dwd_multi_forecast_wf_ultra_short_term'
)
ORDER BY schemaname, tablename;
```

若结果为空，可正式确认该实例未部署/未同步这些预测业务表。

SELECT id,
       forecast_date,
       data_value
FROM dwd.dwd_forecast_wf_ultra_short_term
WHERE forecast_date BETWEEN '2026-02-26 08:00:00' 
                        AND '2026-03-13 18:00:00'
ORDER BY forecast_date DESC
LIMIT 3;

![alt text](image-2.png)



SELECT id,
       forecast_date,
       cumulative_value,
       data_value
FROM dwd.dwd_multi_forecast_wf_short_term_partition
WHERE forecast_date BETWEEN '2026-02-27 08:00:00' 
                        AND '2026-03-13 18:00:00'
ORDER BY forecast_date
LIMIT 3;

多源dwd_multi_forecast_wf_short_term_partition ，一个点，例如：2026-02-27 00:00:00 是 230条

select count(*) from dwd.dwd_forecast_wf_short_term_partition where forecast_date='2026-02-27 00:00:00';
dwd_forecast_wf_short_term_partition，1个点 ：1196 条 ？



id：记录的唯一标识
forecast_date：预测时间
cumulative_value：累计预测值
data_value：数据值（可能是数组或 JSON 格式）

![alt text](image-3.png)



SELECT id,
       forecast_date,
       cumulative_value,
       data_value
FROM dwd.dwd_forecast_wf_short_term_partition
WHERE forecast_date BETWEEN '2026-02-26 08:00:00' 
                        AND '2026-03-13 18:00:00'
ORDER BY forecast_date
LIMIT 3;

![alt text](image-4.png)


![alt text](image.png)


CREATE TABLE dwd.dwd_forecast_wf_ultra_short_term (
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


![alt text](image-1.png)


各个表的统计数量：
![alt text](image-5.png)
