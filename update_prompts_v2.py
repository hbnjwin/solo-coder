#!/usr/bin/env python3
"""
Apply the "hidden bug" strategy to all 20 projects:
- Prompt only exposes 1-2 obvious bugs (round 1 fixes)
- Source code contains 1 hidden bug (round 2 discovers via cargo test)
- Test cases cover both obvious and hidden bugs
"""
import json, os

DATA_JSON = r"C:\Users\wt.home\AppData\Roaming\com.solo-coder.fill-tool\data.json"

with open(DATA_JSON, "r", encoding="utf-8") as f:
    data = json.load(f)

# Map of question id -> new prompt (only expose obvious bugs, hide the deeper one)
prompt_updates = {
    "q6": (
        "这是一个 Rust CLI 实现的简化工作流状态机引擎。当前所有逻辑都在单文件 main.rs 中。"
        "状态机用于驱动审批流程，包含 Draft、Submitted、UnderReview、Approved、Rejected、Completed 等状态。\n\n"
        "现在有2个bug需要修复：\n"
        "1. cleanup_stale_workflows 中 TTL 检查反转（< 应为 >），导致新鲜的被清理、过期的保留\n"
        "2. cleanup_stale_workflows 用 updated_at.len() 代替真实时间差比较，语义完全错误，应改为基于时间戳的计算\n\n"
        "请完成以下任务：\n"
        "1. 将 main.rs 拆分为 state.rs（状态定义和转换逻辑）、engine.rs（工作流执行引擎，含 cleanup）、main.rs（入口）\n"
        "2. 修复上述2个bug\n"
        "3. 运行 cargo test 确认所有测试通过（项目里已有测试用例，修好后应全部通过）"
    ),
    "q7": (
        "这是一个 Rust CLI 实现的简化审批路由引擎。当前所有逻辑都在单文件 main.rs 中。"
        "审批路由用于根据业务条件决定下一审批节点。\n\n"
        "现在有1个问题需要修复：\n"
        "1. 只支持线性串行路由，不支持条件分支（如金额>100万需总经理审批）\n\n"
        "请完成以下任务：\n"
        "1. 将 main.rs 拆分为 router.rs（路由逻辑）、condition.rs（条件表达式解析和求值）、main.rs（入口）\n"
        "2. 添加条件分支支持：条件表达式支持 >、<、==、>=、<=、!= 和 and/or 组合\n"
        "3. 支持根据上下文变量（amount、dept、urgent 等）动态计算条件\n"
        "4. 运行 cargo test 确认所有测试通过（项目里已有测试用例，修好后应全部通过）"
    ),
    "q8": (
        "这是一个 Rust CLI 实现的简化 SQL 查询模板引擎。当前所有逻辑都在单文件 main.rs 中。"
        "查询模板用于安全地生成 SQL 语句。\n\n"
        "现在有2个问题需要修复：\n"
        "1. parse_template 直接拼接用户输入到 SQL 中，存在 SQL 注入风险\n"
        "2. validate_param_value 完全没有实现，任何输入都通过\n\n"
        "请完成以下任务：\n"
        "1. 将 main.rs 拆分为 template.rs（模板解析逻辑）、validator.rs（参数校验逻辑）、main.rs（入口）\n"
        "2. 实现 validate_param_value：拦截 SQL 关键字（SELECT、DROP、DELETE、INSERT、UPDATE、--、;）\n"
        "3. 实现 build_parameterized_query：使用 $1、$2 占位符代替字符串拼接\n"
        "4. 运行 cargo test 确认所有测试通过（项目里已有测试用例，修好后应全部通过）"
    ),
    "q9": (
        "这是一个 Rust CLI 实现的简化报表校验引擎。当前所有逻辑都在单文件 main.rs 中。"
        "报表校验用于检查业务数据的合规性。\n\n"
        "现在有1个问题需要修复：\n"
        "1. 只支持单条独立规则，不支持规则链（and/or 组合）\n\n"
        "请完成以下任务：\n"
        "1. 将 main.rs 拆分为 rule.rs（规则定义和单条规则求值）、engine.rs（规则引擎、规则链和短路求值）、main.rs（入口）\n"
        "2. 添加规则链支持：RuleChain 包含多个 Rule，支持 and/or 组合\n"
        "3. 添加短路求值：and 链中第一条 false 就停止，or 链中第一条 true 就停止\n"
        "4. 运行 cargo test 确认所有测试通过（项目里已有测试用例，修好后应全部通过）"
    ),
    "q10": (
        "这是一个 Rust CLI 实现的简化 DealContent JSON 解析器，用于解析工作流配置中的 dealcontent_json 字段。"
        "当前所有逻辑都在单文件 main.rs 中。dealcontent_json 是复杂的嵌套 JSON。\n\n"
        "现在有2个bug需要修复：\n"
        "1. 访问数组越界时 panic（用索引直接访问而非 get()）\n"
        "2. unwrap None 时 panic（所有 Option 都直接 unwrap）\n\n"
        "请完成以下任务：\n"
        "1. 将 main.rs 拆分为 parser.rs（JSON 解析逻辑）、types.rs（数据结构定义）、main.rs（入口）\n"
        "2. 修复越界 bug：使用 get() 代替索引访问，越界时返回错误\n"
        "3. 修复空值 bug：所有 unwrap 改为 ok_or/with_context\n"
        "4. 运行 cargo test 确认所有测试通过（项目里已有测试用例，修好后应全部通过）"
    ),
    "q11": (
        "这是一个 Rust CLI 实现的简化审批委托管理系统。当前所有逻辑都在单文件 main.rs 中。"
        "审批委托用于在审批人请假时将权限临时转移给代理人。\n\n"
        "现在有2个问题需要修复：\n"
        "1. 没有委托机制，审批人 unavailable 时流程卡住\n"
        "2. detect_circular_delegation 完全没实现，A->B->A 循环委托不会报错\n\n"
        "请完成以下任务：\n"
        "1. 将 main.rs 拆分为 delegation.rs（委托规则管理）、resolver.rs（审批人解析逻辑）、main.rs（入口）\n"
        "2. 实现委托规则：支持按时间段委托\n"
        "3. 实现循环委托检测：A->B->A 报错\n"
        "4. 运行 cargo test 确认所有测试通过（项目里已有测试用例，修好后应全部通过）"
    ),
    "q12": (
        "这是一个 Rust CLI 实现的简化审批审计日志系统。当前所有逻辑都在单文件 main.rs 中。"
        "审计日志用于记录审批操作的全链路追踪。\n\n"
        "现在有1个问题需要修复：\n"
        "1. compute_field_diff 完全没实现，无法对比审批前后的字段变更\n\n"
        "请完成以下任务：\n"
        "1. 将 main.rs 拆分为 store.rs（审计日志存储和查询）、diff.rs（字段变更对比）、main.rs（入口）\n"
        "2. 实现 compute_field_diff：支持顶层字段对比（如 amount 从 10000 变为 15000）\n"
        "3. 运行 cargo test 确认所有测试通过（项目里已有测试用例，修好后应全部通过）"
    ),
    "q13": (
        "这是一个 Rust CLI 实现的简化并发审批控制器。当前所有逻辑都在单文件 main.rs 中。"
        "并发审批用于处理多人同时审批同一记录的场景。\n\n"
        "现在有1个问题需要修复：\n"
        "1. approve_record 的 read-then-write 不是原子操作，并发时后写覆盖前写\n\n"
        "请完成以下任务：\n"
        "1. 将 main.rs 拆分为 store.rs（存储层，含乐观锁版本检查）、handler.rs（审批处理逻辑）、main.rs（入口）\n"
        "2. 实现乐观锁：update 时检查 version 是否匹配，不匹配则拒绝\n"
        "3. 运行 cargo test 确认所有测试通过（项目里已有测试用例，修好后应全部通过）"
    ),
    "q14": (
        "这是一个 Vue 3 + TypeScript 的合同审批表单组件。当前所有代码都在单文件 App.vue 中。"
        "表单用于提交合同审批申请。\n\n"
        "现在有1个问题需要修复：\n"
        "1. 表单没有任何验证逻辑，空数据也能提交\n\n"
        "请完成以下任务：\n"
        "1. 将 App.vue 拆分为 ContractForm.vue（表单组件）、FormValidator.ts（验证逻辑）、App.vue（入口）\n"
        "2. 添加表单验证：合同名称不能为空、金额必须大于0、审批人必选、日期不能为空\n"
        "3. 运行 npm run build（包含 vue-tsc 类型检查）确认编译通过，不能使用 any 类型"
    ),
    "q15": (
        "这是一个 Vue 3 + TypeScript 的审批流程设计器组件。当前所有代码都在单文件 App.vue 中。"
        "设计器用于可视化配置审批流程节点和连线。\n\n"
        "现在有1个问题需要修复：\n"
        "1. 只能添加节点，不能删除节点和连线\n\n"
        "请完成以下任务：\n"
        "1. 将 App.vue 拆分为 DesignerCanvas.vue（画布组件）、NodePanel.vue（节点面板）、FlowEngine.ts（流程引擎逻辑）、App.vue（入口）\n"
        "2. 添加删除节点和连线功能\n"
        "3. 运行 npm run build（包含 vue-tsc 类型检查）确认编译通过，不能使用 any 类型"
    ),
    "q16": (
        "这是一个 Vue 3 + TypeScript 的合同主从表组件。当前所有代码都在单文件 App.vue 中。"
        "主从表用于展示合同列表和对应的付款明细。\n\n"
        "现在有1个问题需要修复：\n"
        "1. 点击合同后付款明细不总是刷新（computed 依赖不完整）\n\n"
        "请完成以下任务：\n"
        "1. 将 App.vue 拆分为 ContractList.vue（合同列表组件）、PaymentDetail.vue（付款明细组件）、useContractData.ts（数据管理 composable）、App.vue（入口）\n"
        "2. 修复明细不刷新的 bug：确保 selectedId 变化时 detail 正确响应\n"
        "3. 添加加载状态和空状态展示\n"
        "4. 运行 npm run build（包含 vue-tsc 类型检查）确认编译通过，不能使用 any 类型"
    ),
    "q17": (
        "这是一个 Vue 3 + TypeScript 的审批权限管理组件。当前所有代码都在单文件 App.vue 中。"
        "权限管理用于控制不同角色对审批功能的访问。\n\n"
        "现在有1个问题需要修复：\n"
        "1. 没有任何权限控制逻辑，所有按钮对所有用户都可见\n\n"
        "请完成以下任务：\n"
        "1. 将 App.vue 拆分为 PermissionGuard.vue（权限守卫组件）、RoleManager.vue（角色管理组件）、usePermission.ts（权限判断 composable）、App.vue（入口）\n"
        "2. 实现权限守卫：根据角色显示/隐藏按钮和菜单\n"
        "3. 运行 npm run build（包含 vue-tsc 类型检查）确认编译通过，不能使用 any 类型"
    ),
    "q18": (
        "这是一个 Vue 3 + TypeScript 的消息通知组件。当前所有代码都在单文件 App.vue 中。"
        "通知组件用于展示审批相关的消息提醒。\n\n"
        "现在有1个问题需要修复：\n"
        "1. 点击消息后标记已读，但红点数字不更新（badge count 不响应）\n\n"
        "请完成以下任务：\n"
        "1. 将 App.vue 拆分为 NotificationBell.vue（铃铛和红点组件）、NotificationList.vue（消息列表组件）、useNotification.ts（通知数据管理 composable）、App.vue（入口）\n"
        "2. 修复红点不更新的 bug\n"
        "3. 运行 npm run build（包含 vue-tsc 类型检查）确认编译通过，不能使用 any 类型"
    ),
    "q19": (
        "这是一个 Vue 3 + TypeScript 的离线审批客户端组件。当前所有代码都在单文件 App.vue 中。"
        "离线审批用于在网络不可用时暂存审批操作，网络恢复后自动同步。\n\n"
        "现在有1个问题需要修复：\n"
        "1. 离线时提交审批直接失败，没有队列暂存机制\n\n"
        "请完成以下任务：\n"
        "1. 将 App.vue 拆分为 ApprovalForm.vue（审批表单组件）、OfflineQueue.vue（离线队列组件）、useOfflineSync.ts（离线同步 composable）、App.vue（入口）\n"
        "2. 实现离线队列：离线时操作存入队列，上线后按时间顺序同步\n"
        "3. 运行 npm run build（包含 vue-tsc 类型检查）确认编译通过，不能使用 any 类型"
    ),
    "q20": (
        "这是一个全栈项目：Vue 3 前端 + Rust 后端，用于合同审批 API。"
        "前端发送审批请求，后端处理并返回结果。\n\n"
        "现在有1个问题需要修复：\n"
        "1. 前端发送的字段名是 camelCase（contractName、sheetAmount），但后端期望 snake_case（contract_name、sheet_amount），导致反序列化失败\n\n"
        "请完成以下任务：\n"
        "1. 后端：将 main.rs 拆分为 api.rs（API 处理）、model.rs（数据模型）、main.rs（入口）\n"
        "2. 后端：给数据模型加上 #[serde(rename_all = \"camelCase\")] 或手动 rename，兼容前端 camelCase\n"
        "3. 前端：拆分 App.vue 为 ApprovalForm.vue、ApiService.ts、App.vue\n"
        "4. 后端运行 cargo test 确认测试通过，前端运行 npm run build（含 vue-tsc）确认编译通过"
    ),
    "q21": (
        "这是一个全栈项目：Vue 3 前端 + Rust 后端，用于审批超时自动升级。"
        "审批超过指定时间未处理时，自动升级到上级审批人。\n\n"
        "现在有1个问题需要修复：\n"
        "1. check_escalation 完全没实现，超时审批不会被升级\n\n"
        "请完成以下任务：\n"
        "1. 后端：将 main.rs 拆分为 escalation.rs（升级逻辑）、timer.rs（超时检测）、main.rs（入口）\n"
        "2. 后端：实现 check_escalation：检查 submitted_at + timeout_hours > now 的任务\n"
        "3. 前端：拆分 App.vue 为 EscalationPanel.vue、useEscalation.ts、App.vue\n"
        "4. 后端运行 cargo test 确认测试通过，前端运行 npm run build（含 vue-tsc）确认编译通过"
    ),
    "q22": (
        "这是一个全栈项目：Vue 3 前端 + Rust 后端，用于审批数据导出。"
        "支持分页导出大量审批数据，并支持失败重试。\n\n"
        "现在有1个问题需要修复：\n"
        "1. export_page 使用页码分页，当导出过程中有新数据插入时，后续页会漏数据或重复数据\n\n"
        "请完成以下任务：\n"
        "1. 后端：将 main.rs 拆分为 pagination.rs（分页逻辑）、export.rs（导出逻辑）、main.rs（入口）\n"
        "2. 后端：实现游标分页（cursor-based）：用最后一条记录的 ID 作为 next_token，避免数据漂移\n"
        "3. 前端：拆分 App.vue 为 ExportPanel.vue、useExport.ts、App.vue\n"
        "4. 后端运行 cargo test 确认测试通过，前端运行 npm run build（含 vue-tsc）确认编译通过"
    ),
    "q23": (
        "这是一个 Rust CLI 实现的审批数据导出工具。当前所有逻辑都在单文件 main.rs 中。"
        "支持导出审批数据到 CSV 和 JSON 格式。\n\n"
        "现在有1个问题需要修复：\n"
        "1. filter_records 完全没实现，所有数据都被导出，无法按状态/日期/审批人筛选\n\n"
        "请完成以下任务：\n"
        "1. 将 main.rs 拆分为 filter.rs（筛选逻辑）、export.rs（导出逻辑，含 CSV 和 JSON）、main.rs（入口）\n"
        "2. 实现 filter_records：按 status、日期范围、approver 筛选\n"
        "3. 实现 sort_records：按指定字段和方向排序\n"
        "4. 运行 cargo test 确认所有测试通过（项目里已有测试用例，修好后应全部通过）"
    ),
    "q24": (
        "这是一个 Rust CLI 实现的工作流配置校验器。当前所有逻辑都在单文件 main.rs 中。"
        "配置校验用于检查工作流定义的完整性和正确性。\n\n"
        "现在有1个问题需要修复：\n"
        "1. validate_structure 只检查 id 和 nodes 是否为空，不检查节点 ID 重复\n\n"
        "请完成以下任务：\n"
        "1. 将 main.rs 拆分为 structure.rs（结构校验）、logic.rs（逻辑校验）、main.rs（入口）\n"
        "2. 添加重复节点 ID 检测\n"
        "3. 运行 cargo test 确认所有测试通过（项目里已有测试用例，修好后应全部通过）"
    ),
    "q25": (
        "这是一个 Rust CLI 实现的审批指标计算器。当前所有逻辑都在单文件 main.rs 中。"
        "指标计算用于统计审批通过率、平均耗时、瓶颈环节等。\n\n"
        "现在有2个问题需要修复：\n"
        "1. calculate_approval_rate 把 pending 状态的记录也算进分母，导致通过率偏低\n"
        "2. calculate_avg_duration 完全没实现，返回 0.0\n\n"
        "请完成以下任务：\n"
        "1. 将 main.rs 拆分为 rate.rs（通过率计算）、duration.rs（耗时计算）、bottleneck.rs（瓶颈分析）、main.rs（入口）\n"
        "2. 修复 approval_rate：排除 pending 记录\n"
        "3. 实现 avg_duration：计算 submit_time 到 complete_time 的平均耗时\n"
        "4. 运行 cargo test 确认所有测试通过（项目里已有测试用例，修好后应全部通过）"
    ),
}

updated = 0
for q in data["questions"]:
    if q["id"] in prompt_updates:
        q["prompt"] = prompt_updates[q["id"]]
        updated += 1

with open(DATA_JSON, "w", encoding="utf-8") as f:
    json.dump(data, f, ensure_ascii=False, indent=2)

print(f"Updated {updated} prompts in data.json")
