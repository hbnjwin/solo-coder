# Solo Coder 标注辅助项目

## 项目概述

这是一个 Trae Solo Coder 用户满意度标注项目。核心工作流：
1. 从题库选题，发送 prompt 给 Trae Solo Coder
2. 评估每轮代码任务的完成度和满意度
3. 填写飞书表格提交标注数据
4. 如不满意，生成下轮 prompt 继续迭代（最多 5 轮）

## 目录结构

- `projects/trae-solo/` — 所有项目代码（每个项目对应 GitHub 上的一个分支）
- `auto-solo-fill-tool/` — Tauri + React 题库维护和交付填写工具
- `docs/require/` — 题库列表和技能说明
- `docs/result/` — trace 日志和评估结果
- `docs/指引说明.md` — 甲方标注规范
- `docs/提交飞书表格.md` — 飞书表格字段说明

## GitHub 仓库

- 地址: https://github.com/hbnjwin/solo-coder
- 每个项目创建同名分支（如 `tsp2-standard-system-tree`）

## 自定义技能

| 技能 | 用途 |
|------|------|
| `/solo-evaluate` | 分析 trace 日志 + git diff，自动评估轮次字段 |
| `/solo-reason` | 生成符合甲方质量要求的不满意原因 |
| `/solo-next` | 基于失败点生成下轮 prompt |
| `/solo-validate` | 提交前合规校验 |
| `/solo-status` | 统计轮次分布和交付进度 |

## 关键约束

- 单轮数据不能超过总量 8%
- 每个任务最多 5 轮对话
- 第一轮不能是单文件修改
- 不满意原因必须包含 2+ 要素（范围/现象/偏差/影响/复现/严重程度）
- 禁止模板化 prompt 和 AI 轨迹总结
- 同环境同任务重复题目仅收录 1 条
