#!/usr/bin/env python3
"""Update data.json prompts: cargo check->cargo test, add hidden bugs/constraints"""
import json, os

DATA_PATH = r"C:\Users\wt.home\AppData\Roaming\com.solo-coder.fill-tool\data.json"

with open(DATA_PATH, "r", encoding="utf-8") as f:
    data = json.load(f)

qs = {q["id"]: q for q in data["questions"]}

# ============================================================
# q6: workflow-state-machine - already cargo test, add hidden bug
# ============================================================
qs["q6"]["prompt"] = (
    "这是一个 Rust CLI 实现的简化工作流状态机引擎。当前所有逻辑都在单文件 main.rs 中。"
    "状态机用于驱动审批流程，包含 Draft、Submitted、UnderReview、Approved、Rejected、Completed 等状态。\n\n"
    "现在有3个bug需要修复：\n"
    "1. transition 函数没有检测已访问状态，Rejected->Submitted->UnderReview->Rejected 会无限循环\n"
    "2. cleanup_stale_workflows 中 TTL 检查反转（< 应为 >），导致新鲜的被清理、过期的保留\n"
    "3. cleanup_stale_workflows 用 updated_at.len() 代替真实时间差比较，语义完全错误，应改为基于时间戳的计算\n\n"
    "请完成以下任务：\n"
    "1. 将 main.rs 拆分为 state.rs（状态定义和转换逻辑）、engine.rs（工作流执行引擎，含 cleanup）、main.rs（入口）\n"
    "2. 修复上述3个bug\n"
    "3. 运行 cargo test 确认所有测试通过（项目里已有3个会失败的测试，修好后应全部通过）"
)

# ============================================================
# q7: approval-routing - cargo check -> cargo test, add hidden bug
# ============================================================
qs["q7"]["prompt"] = (
    "这是一个 Rust CLI 实现的简化审批路由引擎。当前所有逻辑都在单文件 main.rs 中。"
    "审批路由用于根据业务条件决定下一审批节点。\n\n"
    "现在有2个问题需要修复：\n"
    "1. 只支持线性串行路由，不支持条件分支（如金额>100万需总经理审批）\n"
    "2. advance() 方法不校验 next_node_id 是否与实际节点匹配，跳转可能指向不存在的节点\n\n"
    "请完成以下任务：\n"
    "1. 将 main.rs 拆分为 router.rs（路由逻辑）、condition.rs（条件表达式解析和求值）、main.rs（入口）\n"
    "2. 添加条件分支支持：条件表达式支持 >、<、==、>=、<=、!= 和 and/or 组合\n"
    "3. 支持根据上下文变量（amount、dept、urgent 等）动态计算条件\n"
    "4. 修复 advance() 的节点校验 bug：跳转前检查目标节点是否存在\n"
    "5. 运行 cargo test 确认所有测试通过（项目里已有2个会失败的测试）"
)

# ============================================================
# q8: query-def-safety - cargo check -> cargo test, add hidden bugs
# ============================================================
qs["q8"]["prompt"] = (
    "这是一个 Rust CLI 实现的简化 SQL 查询模板引擎。当前所有逻辑都在单文件 main.rs 中。"
    "系统允许用户通过模板定义查询，如 SELECT * FROM contracts WHERE dept = {dept}。\n\n"
    "现在有3个安全漏洞需要修复：\n"
    "1. parse_template 直接把用户输入拼接到 SQL 中，存在 SQL 注入风险\n"
    "2. validate_param_value 完全没有实现，任何输入都通过校验\n"
    "3. build_parameterized_query 未实现，无法生成安全的参数化查询（$1, $2 占位符）\n\n"
    "请完成以下任务：\n"
    "1. 将 main.rs 拆分为 parser.rs（模板解析）、executor.rs（查询执行，含参数化查询构建）、main.rs（入口）\n"
    "2. 修复 SQL 注入漏洞：所有变量必须使用占位符（$1, $2）而不是字符串拼接\n"
    "3. 实现输入校验：禁止在变量值中出现 SQL 关键字（DROP、DELETE、INSERT、UPDATE），并处理 Unicode 绕过（如 ＤＲＯＰ 全角字符）\n"
    "4. 实现 build_parameterized_query：返回 (sql_with_placeholders, param_values) 元组\n"
    "5. 运行 cargo test 确认所有测试通过（项目里已有3个会失败的测试）"
)

# ============================================================
# q9: report-validator - cargo check -> cargo test, add hidden bugs
# ============================================================
qs["q9"]["prompt"] = (
    "这是一个 Rust CLI 实现的简化报表校验引擎。当前所有逻辑都在单文件 main.rs 中。"
    "报表校验用于检查业务数据的合规性。\n\n"
    "现在有3个问题需要修复：\n"
    "1. 只支持单条独立规则，不支持规则链（and/or 组合）\n"
    "2. evaluate_rule 对 gte/lte 操作符的边界值处理有 off-by-one 错误（等于阈值时返回 false）\n"
    "3. 没有短路求值机制，即使 and 链中第一条就 false 也会执行所有规则\n\n"
    "请完成以下任务：\n"
    "1. 将 main.rs 拆分为 rule.rs（规则定义和单条规则求值）、engine.rs（规则引擎、规则链和短路求值）、logger.rs（执行日志）、main.rs（入口）\n"
    "2. 修复 gte/lte 边界 bug：等于阈值时应返回 true\n"
    "3. 添加规则链支持：RuleChain 包含多个 Rule，支持 and/or 组合\n"
    "4. 添加短路求值：and 链中第一条 false 就停止，or 链中第一条 true 就停止\n"
    "5. 运行 cargo test 确认所有测试通过（项目里已有2个会失败的测试）"
)

# ============================================================
# q10: deal-content-parser - already cargo test, add hidden bug
# ============================================================
qs["q10"]["prompt"] = (
    "这是一个 Rust CLI 实现的简化 DealContent JSON 解析器，用于解析工作流配置中的 dealcontent_json 字段。"
    "当前所有逻辑都在单文件 main.rs 中。dealcontent_json 是复杂的嵌套 JSON。\n\n"
    "现在有3个bug需要修复：\n"
    "1. 访问数组越界时 panic（用索引直接访问而非 get()）\n"
    "2. unwrap None 时 panic（所有 Option 都直接 unwrap）\n"
    "3. 递归解析子节点时没有检测循环引用，如果 children 中出现循环（如 A 的 child 引用回 A），会导致栈溢出\n\n"
    "请完成以下任务：\n"
    "1. 将 main.rs 拆分为 parser.rs（JSON 解析逻辑）、types.rs（数据结构定义）、main.rs（入口）\n"
    "2. 修复越界 bug：使用 get() 代替索引访问，越界时返回错误\n"
    "3. 修复空值 bug：所有 unwrap 改为 ok_or/with_context\n"
    "4. 添加循环引用检测：递归解析时维护已访问节点 ID 集合，重复访问时返回错误\n"
    "5. 运行 cargo test 确认所有测试通过（项目里已有3个会失败的测试）"
)

# ============================================================
# q11: approval-delegation - cargo check -> cargo test, add hidden bugs
# ============================================================
qs["q11"]["prompt"] = (
    "这是一个 Rust CLI 实现的简化审批委托管理系统。当前所有逻辑都在单文件 main.rs 中。"
    "审批委托用于在审批人请假时将权限临时转移给代理人。\n\n"
    "现在有3个问题需要修复：\n"
    "1. 没有委托机制，审批人 unavailable 时流程卡住\n"
    "2. detect_circular_delegation 完全没实现，A->B->A 循环委托不会报错\n"
    "3. resolve_approver 不支持委托链（A->B->C），只返回直接委托人 B 而非最终审批人 C\n\n"
    "额外约束：委托链中如果中间环节已过期（如 A->B 有效但 B->C 已过期），应跳过 B->C 继续使用 B 作为审批人，而不是报错。\n\n"
    "请完成以下任务：\n"
    "1. 将 main.rs 拆分为 delegation.rs（委托规则管理）、resolver.rs（审批人解析逻辑，含链式查找和过期跳过）、main.rs（入口）\n"
    "2. 实现委托规则：支持按时间段委托\n"
    "3. 实现委托链：A 委托给 B，B 委托给 C，解析器自动找到最终审批人\n"
    "4. 实现循环委托检测：A->B->A 报错\n"
    "5. 实现过期跳过：链中过期环节自动跳过\n"
    "6. 运行 cargo test 确认所有测试通过（项目里已有3个会失败的测试）"
)

# ============================================================
# q12: audit-trail - cargo check -> cargo test, add hidden bugs
# ============================================================
qs["q12"]["prompt"] = (
    "这是一个 Rust CLI 实现的简化审批审计日志系统。当前所有逻辑都在单文件 main.rs 中。"
    "审计日志用于记录审批流程中的每一次操作。\n\n"
    "现在有3个问题需要修复：\n"
    "1. 没有字段级变更追踪，只有简单操作记录\n"
    "2. compute_field_diff 完全没实现，无法对比修改前后的值\n"
    "3. compute_field_diff 需要支持嵌套对象和数组的变更检测（如 approver 从 {name:\"张三\"} 变为 {name:\"李四\"}，应记录 approver.name: 张三->李四）\n\n"
    "请完成以下任务：\n"
    "1. 将 main.rs 拆分为 audit.rs（审计记录结构）、diff.rs（字段级变更对比，含嵌套对象和数组）、storage.rs（日志存储和查询）、main.rs（入口）\n"
    "2. 实现字段级变更追踪：记录修改前后的具体值\n"
    "3. 实现嵌套对象变更检测：用点号路径表示嵌套字段（如 approver.name）\n"
    "4. 实现数组变更检测：记录新增、删除、修改的元素\n"
    "5. 实现全链路追踪：通过 trace_id 查询一次审批的所有操作\n"
    "6. 运行 cargo test 确认所有测试通过（项目里已有1个会失败的测试）"
)

# ============================================================
# q13: concurrent-approval - already cargo test, add hidden bug
# ============================================================
qs["q13"]["prompt"] = (
    "这是一个 Rust CLI 实现的简化并发审批控制器。当前所有逻辑都在单文件 main.rs 中。"
    "系统支持多人同时审批，使用乐观锁（version 字段）防止并发冲突。\n\n"
    "现在有3个bug需要修复：\n"
    "1. 读取和更新 version 之间没有原子性检查，两个审批人同时操作时数据被覆盖\n"
    "2. 重试机制未实现：检测到并发冲突后直接返回错误，没有自动重试\n"
    "3. 即使实现了重试，重试时没有重新读取最新 version，而是用第一次读取的旧 version 再次比较，导致重试永远失败\n\n"
    "请完成以下任务：\n"
    "1. 将 main.rs 拆分为 lock.rs（乐观锁机制）、store.rs（数据存储和版本控制）、main.rs（入口）\n"
    "2. 修复乐观锁 bug：更新时必须检查 version 是否匹配\n"
    "3. 实现重试机制：并发冲突时自动重试（最多3次，随机退避）\n"
    "4. 修复重试 bug：每次重试前必须重新读取最新 version\n"
    "5. 运行 cargo test 确认所有测试通过（项目里已有1个会失败的测试）"
)

# ============================================================
# q14: approval-form-validation - add hidden bug + vue-tsc
# ============================================================
qs["q14"]["prompt"] = (
    "这是一个 Vue 3 + TypeScript 实现的简化审批表单页面。当前所有代码都在单文件 App.vue 中。"
    "表单包含合同名称、金额、审批人、日期等字段。\n\n"
    "现在有3个问题需要修复：\n"
    "1. 没有任何校验逻辑，空名称和负数金额都可以提交\n"
    "2. 金额和审批人之间有联动关系（金额>100万需总经理），但没有实现\n"
    "3. 金额超过100万时，审批人下拉框中没有\"总经理\"选项（只有\"部门经理\"），导致联动校验无法通过\n\n"
    "请完成以下任务：\n"
    "1. 将 App.vue 拆分为多个组件：FormField.vue（通用表单字段，含校验提示）、ContractForm.vue（合同表单逻辑）、ApprovalSelector.vue（审批人选择器，金额>100万时自动添加总经理选项）、App.vue（入口）\n"
    "2. 添加表单校验：合同名称必填、金额必须大于0、日期不能晚于今天\n"
    "3. 添加联动校验：金额>100万时审批人级别必须>=总经理\n"
    "4. 修复审批人下拉框bug：金额超过100万时自动添加\"总经理\"选项\n"
    "5. 所有组件必须有正确的 TypeScript 类型定义，不能使用 any\n"
    "6. 运行 npm run build（包含 vue-tsc 类型检查）确认构建通过"
)

# ============================================================
# q15: workflow-designer - add constraint + hidden bug
# ============================================================
qs["q15"]["prompt"] = (
    "这是一个 Vue 3 + TypeScript 实现的简化工作流设计器。当前所有代码都在单文件 App.vue 中。"
    "设计器用于可视化配置审批流程。\n\n"
    "现在只有静态展示，不支持交互。需要添加以下功能：\n"
    "1. 节点拖拽：从工具栏拖拽节点到画布\n"
    "2. 连线功能：点击节点输出端点再点击另一个输入端点创建连线\n"
    "3. 节点删除：选中节点按 Delete 删除，同时删除相关连线\n\n"
    "额外约束：\n"
    "- 连线不能形成环（A→B→C→A），检测到环路时提示用户\"不允许创建循环连线\"\n"
    "- 删除节点后如果留下悬空连线（一端没有连接节点），应自动清理这些悬空连线\n\n"
    "请完成以下任务：\n"
    "1. 将 App.vue 拆分为：DesignerCanvas.vue（画布）、WorkflowNode.vue（节点组件）、ConnectionLine.vue（连线组件）、Toolbar.vue（工具栏）、App.vue（入口）\n"
    "2. 实现节点拖拽、连线、删除功能\n"
    "3. 实现环路检测：创建连线前检查是否会形成环\n"
    "4. 实现悬空连线清理：删除节点时自动删除相关连线\n"
    "5. 所有组件必须有正确的 TypeScript 类型定义\n"
    "6. 运行 npm run build（包含 vue-tsc 类型检查）确认构建通过"
)

# ============================================================
# q16: master-detail-sync - add hidden bug
# ============================================================
qs["q16"]["prompt"] = (
    "这是一个 Vue 3 + TypeScript 实现的简化主子表页面。当前所有代码都在单文件 App.vue 中。"
    "页面左侧是合同主表列表，右侧是选中合同的付款明细子表。\n\n"
    "现在有3个bug需要修复：\n"
    "1. 选中不同合同行时子表偶尔不刷新，还是显示上一个合同的明细\n"
    "2. 快速连续点击不同主表行时，子表会出现闪烁或短暂显示错误数据（竞态条件）\n"
    "3. 没有加载状态和空状态提示，用户体验差\n\n"
    "请完成以下任务：\n"
    "1. 将 App.vue 拆分为：MasterTable.vue（主表）、DetailTable.vue（子表）、SyncIndicator.vue（同步状态指示器）、App.vue（入口）\n"
    "2. 修复子表刷新 bug：确保主表选中行变化时子表一定重新加载\n"
    "3. 修复快速点击闪烁 bug：添加防抖或取消机制，确保只显示最终选中行的数据\n"
    "4. 添加加载状态：子表加载时显示 loading 动画\n"
    "5. 添加空状态：没有选中合同或子表为空时显示友好提示\n"
    "6. 所有组件必须有正确的 TypeScript 类型定义\n"
    "7. 运行 npm run build（包含 vue-tsc 类型检查）确认构建通过"
)

# ============================================================
# q17: permission-guard - add constraint + hidden bug
# ============================================================
qs["q17"]["prompt"] = (
    "这是一个 Vue 3 + TypeScript 实现的简化审批系统前端。当前所有代码都在单文件 App.vue 中。"
    "系统有多个角色：普通员工、部门经理、总经理、系统管理员。\n\n"
    "现在没有任何权限控制。需要添加以下功能：\n"
    "1. 路由级权限：不同角色看到不同页面\n"
    "2. 按钮级权限：不同角色看到不同操作按钮\n\n"
    "额外约束：\n"
    "- 管理员不能删除自己的账号，需要添加自删除保护\n"
    "- 切换角色后页面必须立即更新，不能残留上一个角色的可见内容\n\n"
    "请完成以下任务：\n"
    "1. 将 App.vue 拆分为：PermissionGuard.vue（权限守卫组件）、NavMenu.vue（根据角色动态显示的导航）、RoleBadge.vue（角色标识）、App.vue（入口）\n"
    "2. 实现路由级权限：员工只能列表和提交，经理可审批，总经理看报表，管理员可配置\n"
    "3. 实现按钮级权限：删除按钮仅管理员可见，审批按钮仅经理和总经理可见\n"
    "4. 实现自删除保护：管理员删除操作不能作用于自己\n"
    "5. 实现角色切换即时更新：切换角色后所有受权限控制的元素立即响应\n"
    "6. 所有组件必须有正确的 TypeScript 类型定义\n"
    "7. 运行 npm run build（包含 vue-tsc 类型检查）确认构建通过"
)

# ============================================================
# q18: notification-badge - add constraint + hidden bug
# ============================================================
qs["q18"]["prompt"] = (
    "这是一个 Vue 3 + TypeScript 实现的简化消息通知组件。当前所有代码都在单文件 App.vue 中。"
    "系统需要显示用户的待审批数量（红点），点击后展示通知列表。\n\n"
    "现在有2个bug和1个缺失功能：\n"
    "1. 清空所有通知后红点数字没有重置为0（badge count 仍显示旧值）\n"
    "2. 没有实时推送模拟，所有通知都是静态的\n"
    "3. 点击通知标记已读后，如果同一条通知被再次推送，应该重新标记为未读\n\n"
    "请完成以下任务：\n"
    "1. 将 App.vue 拆分为：NotificationBell.vue（铃铛+红点角标）、NotificationList.vue（下拉列表）、NotificationItem.vue（单条通知）、App.vue（入口）\n"
    "2. 修复清空后红点不重置 bug\n"
    "3. 添加实时推送模拟：用 setInterval 每3秒自动添加一条新通知，红点数字实时更新\n"
    "4. 实现已读标记：点击通知后标记已读，红点减少\n"
    "5. 实现重复推送处理：已读通知被再次推送时重新标记为未读\n"
    "6. 所有组件必须有正确的 TypeScript 类型定义\n"
    "7. 运行 npm run build（包含 vue-tsc 类型检查）确认构建通过"
)

# ============================================================
# q19: offline-queue - add constraint + hidden bug
# ============================================================
qs["q19"]["prompt"] = (
    "这是一个 Tauri + Vue 3 + TypeScript 实现的简化审批客户端。当前所有代码都在单文件 App.vue 中。"
    "用户在断网环境下需要能继续提交审批，等网络恢复后自动同步。\n\n"
    "现在有3个问题需要修复：\n"
    "1. 断网时所有操作直接报错，没有本地缓存\n"
    "2. 同步重试时没有保持请求顺序（先提交的可能后发送），导致业务逻辑错乱\n"
    "3. 没有队列上限，大量离线操作可能撑爆 localStorage\n\n"
    "请完成以下任务：\n"
    "1. 将 App.vue 拆分为：OfflineStatusBar.vue（网络状态指示）、PendingQueue.vue（待同步队列展示）、SyncManager.vue（同步逻辑）、App.vue（入口）\n"
    "2. 添加网络检测：实时监测网络状态，断网时显示离线提示\n"
    "3. 添加本地队列：断网时审批请求缓存到 localStorage\n"
    "4. 修复同步顺序 bug：恢复网络后按提交时间顺序发送，保证先提交先处理\n"
    "5. 添加队列上限：最多50条，超出时提示\"队列已满，请恢复网络后重试\"\n"
    "6. 添加重试机制：同步失败时自动重试（最多3次）\n"
    "7. 所有组件必须有正确的 TypeScript 类型定义\n"
    "8. 运行 npm run build（包含 vue-tsc 类型检查）确认构建通过"
)

# ============================================================
# q20: api-contract - cargo check->cargo test, add hidden bugs
# ============================================================
qs["q20"]["prompt"] = (
    "这是一个包含前后端分离的简化审批系统。后端是 Rust CLI，前端是 Vue 3。"
    "当前后端所有代码在 main.rs，前端所有代码在 App.vue。\n\n"
    "前后端 API 合同有4个不一致问题：\n"
    "1. 前端用 camelCase（sheetAmount），后端期望 snake_case（sheet_amount）\n"
    "2. 前端日期格式是 ISO 字符串（2026-05-11T10:00:00Z），后端只接受 YYYY-MM-DD\n"
    "3. 前端布尔值用字符串 \"true\"，后端期望 JSON true\n"
    "4. 后端 ApprovalRequest 的 urgent 字段是 String 类型，但业务逻辑中用 == \"true\" 判断，应改为 bool 类型\n\n"
    "请完成以下任务：\n"
    "1. 将后端 main.rs 拆分为 handler.rs（请求处理）、dto.rs（数据传输对象定义，统一字段命名和类型）、main.rs（入口）\n"
    "2. 将前端 App.vue 拆分为 api.ts（API 调用层，含 camelCase/snake_case 转换）、types.ts（类型定义）、ContractForm.vue（表单组件）、App.vue（入口）\n"
    "3. 统一 API 合同：前后端统一使用 snake_case、ISO 8601 日期、JSON 布尔值\n"
    "4. 修复后端 urgent 字段类型：String 改为 bool\n"
    "5. 添加请求/响应转换层\n"
    "6. 运行 cargo test（后端）和 npm run build（前端，含 vue-tsc）确认通过"
)

# ============================================================
# q21: workflow-escalation - cargo check->cargo test, add constraints
# ============================================================
qs["q21"]["prompt"] = (
    "这是一个包含前后端分离的简化审批系统。后端是 Rust CLI，前端是 Vue 3。"
    "当前后端所有代码在 main.rs，前端所有代码在 App.vue。\n\n"
    "审批流程中如果审批人长时间不处理，流程会永远卡住。需要添加超时自动升级机制。\n\n"
    "现在有3个问题：\n"
    "1. check_escalation 完全没实现，超时的审批不会被升级\n"
    "2. 升级后原审批人不会收到通知，不知道自己的任务已被转交\n"
    "3. 升级链没有上限，如果系统管理员也不处理，会无限升级\n\n"
    "请完成以下任务：\n"
    "1. 将后端 main.rs 拆分为 escalation.rs（升级逻辑和定时检查）、notification.rs（通知发送）、main.rs（入口）\n"
    "2. 实现超时检测：检查所有 Pending 状态的审批，超过可配置时间触发升级\n"
    "3. 实现升级链：员工→经理→总经理→系统管理员，到达最高级后不再升级而是标记为\"超时未处理\"\n"
    "4. 实现升级通知：升级后原审批人收到通知（写入通知列表）\n"
    "5. 前端添加升级标识：已升级的审批卡片显示【已升级】标签和升级原因\n"
    "6. 运行 cargo test（后端）和 npm run build（前端，含 vue-tsc）确认通过"
)

# ============================================================
# q22: export-retry - cargo check->cargo test, add constraints
# ============================================================
qs["q22"]["prompt"] = (
    "这是一个包含前后端分离的简化审批系统。后端是 Rust CLI，前端是 Vue 3。"
    "当前后端所有代码在 main.rs，前端所有代码在 App.vue。\n\n"
    "用户需要导出大量审批记录（可能几万条），现在一次性返回所有数据，超时或网络中断就失败。\n\n"
    "现在有2个bug和1个缺失功能：\n"
    "1. export_page 用页码分页，但导出期间如果有新数据插入，会导致某些记录被重复导出或遗漏（数据漂移）\n"
    "2. 前端没有断点续传功能，网络中断后只能从头开始\n"
    "3. 导出过程中如果有新数据写入，需要确保不重复导出也不遗漏\n\n"
    "请完成以下任务：\n"
    "1. 将后端 main.rs 拆分为 export.rs（基于游标的分块导出）、pagination.rs（游标分页，用 ID 而非页码避免数据漂移）、main.rs（入口）\n"
    "2. 后端改为游标分页：用最后一条记录的 ID 作为 page_token，避免数据漂移\n"
    "3. 前端添加进度条：显示导出进度和预计剩余时间\n"
    "4. 前端添加断点续传：网络中断后点击【继续】从断点恢复\n"
    "5. 添加取消功能：导出过程中可以取消\n"
    "6. 运行 cargo test（后端）和 npm run build（前端，含 vue-tsc）确认通过"
)

# ============================================================
# q23: approval-csv-export - cargo check->cargo test, add hidden bug
# ============================================================
qs["q23"]["prompt"] = (
    "这是一个 Rust CLI 项目，当前所有代码都在单文件 main.rs 中。"
    "工具用于将审批记录导出为 CSV 和 JSON 格式。\n\n"
    "现在有3个问题需要修复：\n"
    "1. filter_records 完全没实现，所有过滤条件被忽略，始终返回全部记录\n"
    "2. sort_records 完全没实现，排序参数无效\n"
    "3. CSV 导出没有转义含逗号、引号、换行的字段值，导致生成的 CSV 格式错误\n\n"
    "请完成以下任务：\n"
    "1. 将 main.rs 拆分为 export.rs（导出逻辑，含 CSV 转义）、filter.rs（过滤条件解析和评估）、main.rs（入口）\n"
    "2. 实现 filter_records：支持按 status、start_date/end_date、approver 过滤\n"
    "3. 实现 sort_records：支持按任意字段升序/降序排序\n"
    "4. 修复 CSV 转义 bug：含逗号、引号、换行的字段值必须用双引号包裹并转义内部引号\n"
    "5. 运行 cargo test 确认所有测试通过（项目里已有测试用例验证 CSV 转义和过滤逻辑）"
)

# ============================================================
# q24: config-validator - cargo check->cargo test, add hidden bug
# ============================================================
qs["q24"]["prompt"] = (
    "这是一个 Rust CLI 项目，当前所有代码都在单文件 main.rs 中。"
    "工具用于校验工作流配置文件（YAML/JSON）的合法性。\n\n"
    "现在有3个问题需要修复：\n"
    "1. validate_structure 只检查了 id 和 nodes 是否为空，缺少更多结构校验\n"
    "2. validate_logic 完全没实现，不检查节点可达性、孤立节点等逻辑问题\n"
    "3. 没有检测重复节点 ID，配置中如果有两个同 ID 的节点不会报错\n\n"
    "请完成以下任务：\n"
    "1. 将 main.rs 拆分为 validator.rs（校验逻辑）、rules.rs（校验规则定义）、reporter.rs（错误报告格式化）、main.rs（入口）\n"
    "2. 完善结构校验：必填字段缺失、字段类型错误、重复节点 ID 检测\n"
    "3. 实现逻辑校验：开始节点唯一、结束节点可达、无孤立节点、条件表达式语法正确\n"
    "4. 实现错误报告：按严重程度分类，显示错误描述和修复建议\n"
    "5. 运行 cargo test 确认所有测试通过（项目里已有测试用例验证重复ID和孤立节点检测）"
)

# ============================================================
# q25: approval-metrics - cargo check->cargo test, add hidden bug
# ============================================================
qs["q25"]["prompt"] = (
    "这是一个 Rust CLI 项目，当前所有代码都在单文件 main.rs 中。"
    "工具用于计算审批流程的各类统计指标。\n\n"
    "现在有4个问题需要修复：\n"
    "1. calculate_avg_duration 完全没实现，始终返回 0.0\n"
    "2. identify_bottlenecks 完全没实现，始终返回空列表\n"
    "3. group_by 完全没实现，始终返回空 HashMap\n"
    "4. calculate_approval_rate 把 status==\"approved\" 的算作通过，但 status==\"rejected\" 的也算进了分母，而 status==\"pending\"（尚未完成）的也被算进去了，导致通过率偏低\n\n"
    "请完成以下任务：\n"
    "1. 将 main.rs 拆分为 metrics.rs（指标计算逻辑）、aggregator.rs（数据聚合和分组）、report.rs（报告生成）、main.rs（入口）\n"
    "2. 修复通过率计算 bug：pending 状态的记录不应计入分母（只统计已完成的）\n"
    "3. 实现平均审批时长：基于 submit_time 和 complete_time 计算\n"
    "4. 实现瓶颈节点识别：找出停留时间最长的审批节点\n"
    "5. 实现分组统计：按部门、审批类型、月份分组\n"
    "6. 运行 cargo test 确认所有测试通过（项目里已有测试用例验证通过率计算）"
)

# Write back
with open(DATA_PATH, "w", encoding="utf-8") as f:
    json.dump(data, f, ensure_ascii=False, indent=2)

print(f"Updated {len(qs)} prompts in data.json")
