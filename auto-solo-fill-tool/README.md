# Solo Coder 交付工具

> 用于【中强智联】Solo Coder 用户满意度标注项目的本地桌面辅助工具，帮助标注员管理题库、记录交付数据、AI 辅助质检，并一键导出 XLSX。

---

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri v2（Rust 后端 + WebView 前端） |
| 前端 | React 19 + TypeScript + Tailwind CSS v4 + Vite 7 |
| 后端（Rust） | reqwest（HTTP）、serde_json（数据）、rust_xlsxwriter（Excel 导出） |
| 包管理 | pnpm |

---

## 项目结构

```
auto-solo-fill-tool/
├── index.html                    # 应用入口（lang: zh-CN）
├── package.json                  # 前端依赖与脚本
├── vite.config.ts                # Vite 配置（端口 1527）
├── tsconfig.json                 # TypeScript 严格模式配置
├── src/
│   ├── main.tsx                  # React 根挂载
│   ├── App.tsx                   # 根组件，Tab 导航
│   ├── types.ts                  # 所有 TypeScript 接口与选项常量
│   ├── store.ts                  # 所有 Tauri invoke() 调用（前端 API 层）
│   ├── index.css                 # 最小 CSS（Tailwind 处理其余样式）
│   └── pages/
│       ├── QuestionBank.tsx      # 题库管理页
│       ├── TaskDelivery.tsx      # 交付记录创建与列表页
│       ├── Export.tsx            # XLSX 导出页
│       └── Settings.tsx          # LLM 与 GitHub 配置页
└── src-tauri/
    ├── tauri.conf.json           # 应用配置（窗口 1000×700）
    ├── Cargo.toml                # Rust 依赖
    └── src/
        ├── main.rs               # 入口
        ├── lib.rs                # Tauri Builder，注册所有命令
        └── commands.rs           # 全部后端逻辑（~940 行）
```

---

## 数据模型

所有数据以 JSON 形式存储在 Tauri 应用数据目录（`%APPDATA%\com.solo-coder.fill-tool\`）。

### Question（题目）

| 字段 | 类型 | 说明 |
|------|------|------|
| id | string | 唯一 ID |
| title | string | 题目名称 |
| abbr | string | 英文缩写（用于生成仓库名/分支名） |
| description | string | 题目描述 |
| prompt | string | 默认 User Prompt |
| tech_stack | string | 技术栈描述 |
| repo_url | string | GitHub 仓库 HTML 地址 |
| github_repo_name | string | 仓库名 |
| github_owner | string | 仓库所有者 |
| github_ssh_url | string | SSH 克隆地址 |
| default_branch | string | 默认分支（默认 `main`） |

### TaskRecord（交付记录）

| 字段 | 类型 | 是否必填 | 说明 |
|------|------|---------|------|
| id | string | — | 唯一 ID |
| question_id | string | ✅ | 关联题目 ID |
| round_number | u8 | ✅ | 轮次（1–5） |
| session_id | string | ✅ | Trae Session ID |
| user_prompt | string | ✅ | 本轮 User Prompt |
| task_type | string | ✅ | 任务类型 |
| business_domain | string | ✅ | 业务领域 |
| modify_scope | string | ✅ | 修改范围 |
| difficulty | string | ✅ | 任务难度 |
| is_completed | string | ✅ | 任务是否完成 |
| is_satisfied | string | ✅ | 产物及过程是否满意 |
| unsatisfied_reason | string | 条件必填 | 不满意时必填 |
| github_url | string | ✅ | GitHub 地址 |
| screenshot_paths | string | ✅ | 截图路径或描述 |
| log_trace | string | ✅ | 日志轨迹 |
| ai_quality_check_result | string | — | AI 质检结果 |
| process_analysis_result | string | — | 过程分析结果 |
| created_at | string | — | 创建时间（ISO 格式） |

---

## 功能模块

### 题库（QuestionBank）

管理题目池，支持：

- 新建 / 编辑 / 删除题目
- 按名称、缩写、技术栈搜索
- 一键调用 GitHub API 创建仓库（命名规则：`solo-coder-{abbr}`）
- 一键创建 feature 分支（命名规则：`feat/{abbr}`）
- 查看题目的 User Prompt 原文

### 交付记录（TaskDelivery）

核心工作流页面，支持：

- 选择题目（自动填充 Prompt 和 GitHub 地址）
- 从 Trae CN 本地日志自动读取最近 Session ID（读取路径：`%APPDATA%\Trae CN\logs\`）
- 填写所有标注字段（任务类型、业务领域、修改范围、难度、完成度、满意度）
- 从文件加载日志轨迹（支持 `.txt`、`.md`、`.log`）
- AI 质检：调用 LLM 检查标注数据质量
- 过程分析：调用 LLM 分析模型执行过程是否满意
- AI 生成不满意原因（不满意时可用）
- 按题目筛选记录列表

### 导出（Export）

一键将所有交付记录导出为 `.xlsx` 文件，包含 18 列：

| 列 | 字段 |
|----|------|
| A | 题目名称 |
| B | 英文缩写 |
| C | 轮次 |
| D | Session ID |
| E | User Prompt |
| F | 任务类型 |
| G | 业务领域 |
| H | 修改范围 |
| I | 任务难度 |
| J | 任务是否完成 |
| K | 产物及过程是否满意 |
| L | 不满意原因 |
| M | GitHub 地址 |
| N | 截图 |
| O | 日志轨迹 |
| P | AI 质检结果 |
| Q | 过程分析结果 |
| R | 创建时间 |

导出文件名格式：`solo-coder-delivery-YYYY-MM-DD.xlsx`，首行冻结，列宽自适应。

### 设置（Settings）

配置两类外部服务：

**LLM 配置**（用于 AI 质检、过程分析、生成不满意原因）

| 接口类型 | 说明 |
|---------|------|
| Anthropic | 调用 `/v1/messages`，支持 claude-sonnet-4-6 等模型 |
| OpenAI 兼容 | 调用 `/v1/chat/completions`，Bearer Token 鉴权 |
| Azure OpenAI | 调用 Azure Deployment 端点，api-key 鉴权 |

**GitHub 配置**（用于自动创建仓库和分支）

- 填写 Personal Access Token（需要 `repo` 权限）
- 验证后自动获取并保存 GitHub 用户名

---

## 选项定义

### 任务类型

| 选项 | 说明 |
|------|------|
| Bug 修复 | 逻辑修复、报错排查、安全漏洞修补 |
| 0-1 代码生成 | 基于 PRD 或自然语言描述的新项目/完整模块构建 |
| Feature 迭代 | 在已有功能基础上的平滑扩展 |
| 代码理解 | 解释逻辑、寻找潜在风险、回答技术细节 |
| 代码重构 | 性能优化、可读性提升、解耦 |
| 工程化 | 环境配置、依赖管理、CI/CD 相关 |
| 代码测试 | 单测、集成测试等 |

### 业务领域

| 分类 | 选项 |
|------|------|
| 大前端与服务端类 | 纯后端 API 服务、Web 前端、全栈 Web 应用、垂直业务 |
| 前沿技术 | 游戏开发、数据分析与可视化、3D/交互可视化、AI/ML 应用、科学计算 |
| 端侧与基础工具 | 命令行工具、桌面应用（含GUI）、自动化与工具脚本 |

### 修改范围

| 选项 | 说明 |
|------|------|
| 无需修改 | 代码理解等无需代码生成的任务；所有不产生 diff 的 session |
| 单文件 | 局部逻辑修改 |
| 模块内多文件 | 涉及同一模块内的接口定义与实现协同 |
| 跨模块多文件 | 涉及多个逻辑单元的调用链修改 |
| 跨系统多模块 | 涉及前后端联调、跨服务调用、或数据库-后端的垂直改动 |

### 任务难度

`一般` / `较难` / `困难`

---

## 开发与构建

```bash
# 安装依赖
pnpm install

# 开发模式（启动 Vite + Tauri）
pnpm tauri dev

# 生产构建
pnpm tauri build
```

> 注意：本工具为 Windows 专用。文件对话框通过 PowerShell 调用，Session ID 读取路径硬编码为 `%APPDATA%\Trae CN\logs\`。

---

## 后端命令一览

| 命令 | 说明 |
|------|------|
| `read_data` / `write_data` | 读写 `data.json`（题库 + 记录） |
| `get_llm_config` / `save_llm_config` | 读写 `llm_config.json` |
| `open_file_dialog` | 调用 PowerShell 打开文件选择对话框 |
| `read_text_file` | 读取指定路径的文本文件 |
| `get_recent_sessions` | 解析 Trae CN 日志，提取最近 10 条 Session ID |
| `call_llm`（内部） | 统一 LLM 调用，支持 Anthropic / Azure / OpenAI 兼容 |
| `ai_quality_check` | 调用 LLM 检查标注数据质量 |
| `process_analysis` | 调用 LLM 分析模型执行过程 |
| `generate_unsatisfied_reason` | 调用 LLM 生成不满意原因文本 |
| `test_connection` | 验证 LLM 配置连通性 |
| `github_get_username` | 获取 GitHub 用户名 |
| `github_create_repo` | 创建 GitHub 仓库（`auto_init: true`，私有） |
| `github_create_branch` | 基于指定分支创建新分支 |
| `export_xlsx` | 构建 XLSX 并返回字节流 |

---

## 注意事项

- **数据全本地**：所有数据存储在 Tauri 应用数据目录，无云同步，无服务器。
- **AI 生成内容需人工审核**：AI 质检、过程分析、不满意原因均为辅助参考，提交前必须自行判断，不要直接复制粘贴。
- **Session ID 读取**：工具会自动从 Trae CN 日志中提取最近 10 条 Session ID，也可手动粘贴。
- **LLM 提示词均为中文**：要求模型输出自然口语化文本，不使用 markdown 格式和 emoji。
