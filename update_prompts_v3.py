#!/usr/bin/env python3
"""
V3 prompt update: symptom-based prompts that don't reveal bug locations.
Projects are already multi-file, so prompts describe symptoms only.
"""
import json, os

DATA_JSON = r"C:\Users\wt.home\AppData\Roaming\com.solo-coder.fill-tool\data.json"
TEMP_JSON = r"D:\work\github\solo-coder\temp_data.json"

prompt_updates = {
    "q6": (
        "这是一个 Rust 实现的工作流状态机引擎，用于驱动审批流程的状态转换。"
        "项目已拆分为多模块结构（models, engine, store, history）。\n\n"
        "运行 cargo test 后有 2 个测试失败：\n"
        "- test_workflow_scenario_a\n"
        "- test_store_maintenance\n\n"
        "请排查失败原因并修复，让所有测试通过。不要修改测试代码。"
    ),
    "q7": (
        "这是一个 Rust 实现的审批路由引擎，支持条件分支路由。"
        "项目已拆分为多模块结构（models, router, condition）。\n\n"
        "运行 cargo test 后有 2 个测试失败：\n"
        "- test_conditional_branch\n"
        "- test_node_not_found\n\n"
        "请排查失败原因并修复，让所有测试通过。不要修改测试代码。"
    ),
    "q8": (
        "这是一个 Rust 实现的 SQL 查询模板引擎，用于安全地生成参数化 SQL。"
        "项目已拆分为多模块结构（models, template, validator, builder）。\n\n"
        "运行 cargo test 后有 3 个测试失败：\n"
        "- test_sql_injection_blocked\n"
        "- test_parameterized_output\n"
        "- test_unicode_bypass\n\n"
        "请排查失败原因并修复，让所有测试通过。不要修改测试代码。"
    ),
    "q9": (
        "这是一个 Rust 实现的报表校验规则引擎，支持单条规则和规则链（And/Or 模式）。"
        "项目已拆分为多模块结构（models, evaluator, chain, reporter）。\n\n"
        "运行 cargo test 后有 3 个测试失败：\n"
        "- test_boundary_gte\n"
        "- test_boundary_lte\n"
        "- test_chain_and_mode\n\n"
        "请排查失败原因并修复，让所有测试通过。不要修改测试代码。"
    ),
    "q10": (
        "这是一个 Rust 实现的 DealContent JSON 解析器，用于解析工作流配置中的嵌套 JSON 结构。"
        "项目已拆分为多模块结构（models, parser, formatter）。\n\n"
        "运行 cargo test 后有多个测试失败，部分测试导致 stack overflow 崩溃：\n"
        "- test_missing_required_fields — FAILED\n"
        "- test_missing_children_array — FAILED\n"
        "- test_circular_reference — stack overflow\n"
        "- test_deep_nesting — stack overflow\n\n"
        "请排查失败原因并修复，让所有测试通过。不要修改测试代码。"
    ),
    "q11": (
        "这是一个 Rust 实现的审批委托管理系统，支持委托链解析和循环检测。"
        "项目已拆分为多模块结构（models, store, resolver）。\n\n"
        "运行 cargo test 后有测试失败。"
        "请排查失败原因并修复，让所有测试通过。不要修改测试代码。"
    ),
    "q12": (
        "这是一个 Rust 实现的审批审计日志系统，支持字段级变更追踪。"
        "项目已拆分为多模块结构（models, store, differ）。\n\n"
        "运行 cargo test 后有 2 个测试失败：\n"
        "- test_array_diff\n"
        "- test_nested_object_diff\n\n"
        "请排查失败原因并修复，让所有测试通过。不要修改测试代码。"
    ),
    "q13": (
        "这是一个 Rust 实现的并发审批控制器，使用乐观锁防止并发冲突。"
        "项目已拆分为多模块结构（models, store, service）。\n\n"
        "运行 cargo test 后有 2 个测试失败：\n"
        "- test_version_conflict\n"
        "- test_concurrent_race\n\n"
        "请排查失败原因并修复，让所有测试通过。不要修改测试代码。"
    ),
    "q14": (
        "这是一个 Vue 3 + TypeScript 实现的合同审批表单。"
        "项目已拆分为多组件结构（App.vue, ContractForm.vue, ApproverSelect.vue, composables/useFormValidation.ts）。\n\n"
        "用户反馈以下问题：\n"
        "1. 表单可以提交空数据（合同名称为空、金额为0都能提交）\n"
        "2. 金额超过100万时，审批人下拉框应该只显示总经理级别的选项，但实际显示了所有人\n"
        "3. 运行 npm run build 时有 TypeScript 类型错误\n\n"
        "请排查并修复所有问题，确保 npm run build 通过且功能正确。不要修改测试代码。"
    ),
    "q15": (
        "这是一个 Vue 3 + TypeScript 实现的工作流可视化设计器。"
        "项目已拆分为多组件结构（App.vue, DesignerCanvas.vue, WorkflowNode.vue, ConnectionLine.vue, composables/useDesigner.ts）。\n\n"
        "用户反馈以下问题：\n"
        "1. 节点可以拖拽到画布上，但连线功能不工作（点击输出端口后再点击输入端口没有反应）\n"
        "2. 选中节点后按 Delete 键，节点消失但相关连线还留在画布上\n"
        "3. 运行 npm run build 时有 TypeScript 类型错误\n\n"
        "请排查并修复所有问题，确保 npm run build 通过且功能正确。不要修改测试代码。"
    ),
    "q16": (
        "这是一个 Vue 3 + TypeScript 实现的合同主子表页面。"
        "项目已拆分为多组件结构（App.vue, ContractList.vue, PaymentDetail.vue, composables/useContractSync.ts）。\n\n"
        "用户反馈以下问题：\n"
        "1. 点击不同的合同行时，右侧付款明细偶尔不刷新，还是显示上一个合同的数据\n"
        "2. 快速连续点击不同合同时，明细数据会错乱（显示的不是当前选中合同的明细）\n"
        "3. 运行 npm run build 时有 TypeScript 类型错误\n\n"
        "请排查并修复所有问题，确保 npm run build 通过且功能正确。不要修改测试代码。"
    ),
    "q17": (
        "这是一个 Vue 3 + TypeScript 实现的审批系统权限管理。"
        "项目已拆分为多组件结构（App.vue, PermissionGuard.vue, NavMenu.vue, composables/usePermission.ts）。\n\n"
        "用户反馈以下问题：\n"
        "1. 所有角色都能看到所有菜单和按钮，权限控制没有生效\n"
        "2. 普通员工能看到「删除」和「审批」按钮\n"
        "3. 运行 npm run build 时有 TypeScript 类型错误\n\n"
        "请排查并修复所有问题，确保 npm run build 通过且功能正确。不要修改测试代码。"
    ),
    "q18": (
        "这是一个 Vue 3 + TypeScript 实现的消息通知组件。"
        "项目已拆分为多组件结构（App.vue, NotificationBell.vue, NotificationList.vue, composables/useNotifications.ts）。\n\n"
        "用户反馈以下问题：\n"
        "1. 点击通知标记已读后，铃铛上的红点数字没有减少\n"
        "2. 「全部已读」按钮点击后红点数字变成 NaN\n"
        "3. 运行 npm run build 时有 TypeScript 类型错误\n\n"
        "请排查并修复所有问题，确保 npm run build 通过且功能正确。不要修改测试代码。"
    ),
    "q19": (
        "这是一个 Vue 3 + TypeScript 实现的离线审批队列客户端。"
        "项目已拆分为多组件结构（App.vue, ApprovalForm.vue, QueueStatus.vue, composables/useOfflineQueue.ts）。\n\n"
        "用户反馈以下问题：\n"
        "1. 断网时提交审批直接报错，没有缓存到本地队列\n"
        "2. 网络恢复后队列中的请求没有自动发送\n"
        "3. 运行 npm run build 时有 TypeScript 类型错误\n\n"
        "请排查并修复所有问题，确保 npm run build 通过且功能正确。不要修改测试代码。"
    ),
    "q20": (
        "这是一个全栈审批系统项目（Rust 后端 + Vue 3 前端），用于提交合同审批。"
        "后端在 backend/ 目录，前端在 frontend/ 目录。\n\n"
        "运行后端 cargo test 后有 3 个测试失败：\n"
        "- test_field_mapping\n"
        "- test_iso_datetime_parsing\n"
        "- test_boolean_handling\n\n"
        "前端 npm run build 能通过，但实际调用后端 API 时请求被拒绝，"
        "返回 400 错误，提示字段解析失败。\n\n"
        "请排查前后端集成问题并修复，让后端所有测试通过。不要修改测试代码。"
    ),
    "q21": (
        "这是一个全栈审批系统项目（Rust 后端 + Vue 3 前端），实现审批超时自动升级功能。"
        "后端在 backend/ 目录，前端在 frontend/ 目录。\n\n"
        "运行后端 cargo test 后有 3 个测试失败：\n"
        "- test_escalation_triggered\n"
        "- test_escalation_chain\n"
        "- test_no_escalation_needed\n\n"
        "前端 npm run build 能通过，但升级状态显示不正确。\n\n"
        "请排查后端升级逻辑问题并修复，让后端所有测试通过。不要修改测试代码。"
    ),
    "q22": (
        "这是一个全栈审批系统项目（Rust 后端 + Vue 3 前端），实现大数据分页导出和断点续传。"
        "后端在 backend/ 目录，前端在 frontend/ 目录。\n\n"
        "运行后端 cargo test 后有 3 个测试失败：\n"
        "- test_cursor_pagination_consistency\n"
        "- test_concurrent_insert_stability\n"
        "- test_resume_from_failure\n\n"
        "前端 npm run build 能通过，但导出中断后点击「继续」会从头开始而不是断点续传。\n\n"
        "请排查分页和续传逻辑问题并修复，让后端所有测试通过。不要修改测试代码。"
    ),
    "q23": (
        "这是一个 Rust 实现的审批数据 CSV/JSON 导出工具，支持按条件过滤和排序。"
        "项目已拆分为多模块结构（models, filter, exporter, sorter）。\n\n"
        "运行 cargo test 后有 4 个测试失败：\n"
        "- test_filter_by_status\n"
        "- test_filter_by_date_range\n"
        "- test_csv_special_chars\n"
        "- test_sort_by_date\n\n"
        "请排查失败原因并修复，让所有测试通过。不要修改测试代码。"
    ),
    "q24": (
        "这是一个 Rust 实现的工作流配置校验器，检查节点定义和转换规则的合法性。"
        "项目已拆分为多模块结构（models, parser, validator）。\n\n"
        "运行 cargo test 后有 2 个测试失败：\n"
        "- test_transition_target_ref\n"
        "- test_node_reachability\n\n"
        "请排查失败原因并修复，让所有测试通过。不要修改测试代码。"
    ),
    "q25": (
        "这是一个 Rust 实现的审批统计指标计算器，计算通过率、平均耗时和瓶颈环节。"
        "项目已拆分为多模块结构（models, calculator, analyzer）。\n\n"
        "运行 cargo test 后有 3 个测试失败：\n"
        "- test_approval_rate\n"
        "- test_avg_duration\n"
        "- test_bottleneck_detection\n\n"
        "请排查失败原因并修复，让所有测试通过。不要修改测试代码。"
    ),
}

# Update temp_data.json
with open(TEMP_JSON, "r", encoding="utf-8") as f:
    temp_data = json.load(f)

updated = 0
for q in temp_data["questions"]:
    if q["id"] in prompt_updates:
        q["prompt"] = prompt_updates[q["id"]]
        updated += 1

with open(TEMP_JSON, "w", encoding="utf-8") as f:
    json.dump(temp_data, f, ensure_ascii=False, indent=2)

print(f"Updated {updated} prompts in temp_data.json")

# Also update the live data.json if it exists
if os.path.exists(DATA_JSON):
    with open(DATA_JSON, "r", encoding="utf-8") as f:
        data = json.load(f)

    updated2 = 0
    for q in data["questions"]:
        if q["id"] in prompt_updates:
            q["prompt"] = prompt_updates[q["id"]]
            updated2 += 1

    with open(DATA_JSON, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, indent=2)

    print(f"Updated {updated2} prompts in data.json")
else:
    print(f"data.json not found at {DATA_JSON}, skipping")
