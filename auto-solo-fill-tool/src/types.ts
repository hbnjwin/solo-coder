export interface Question {
  id: string;
  title: string;
  abbr: string;
  description: string;
  prompt: string;
  tech_stack: string;
  repo_url: string;
  github_repo_name: string;
  github_owner: string;
  github_ssh_url: string;
  default_branch: string;
}

export interface TaskRecord {
  id: string;
  question_id: string;
  round_number: number;
  session_id: string;
  user_prompt: string;
  task_type: string;
  business_domain: string;
  modify_scope: string;
  difficulty: string;
  is_completed: string;
  is_satisfied: string;
  unsatisfied_reason: string;
  github_url: string;
  screenshot_paths: string;
  log_trace: string;
  ai_quality_check_result: string;
  process_analysis_result: string;
  created_at: string;
}

export interface AppData {
  questions: Question[];
  records: TaskRecord[];
}

export interface LlmConfig {
  provider: 'openai_compat' | 'azure' | 'anthropic';
  base_url: string;
  api_key: string;
  model: string;
  azure_deployment: string;
  azure_api_version: string;
  github_token: string;
  github_username: string;
}

export const TASK_TYPE_OPTIONS = [
  'Bug 修复',
  '0-1 代码生成',
  'Feature 迭代',
  '代码理解',
  '代码重构',
  '工程化',
  '代码测试',
];

export const BUSINESS_DOMAIN_OPTIONS = [
  '大前端与服务端类',
  '纯后端 API 服务',
  'Web 前端',
  '全栈 Web 应用',
  '垂直业务',
  '游戏开发',
  '数据分析与可视化',
  '3D/交互可视化',
  'AI/ML 应用',
  '科学计算',
  '命令行工具',
  '桌面应用（含GUI）',
  '自动化与工具脚本',
];

export const MODIFY_SCOPE_OPTIONS = [
  '无需修改',
  '单文件',
  '模块内多文件',
  '跨模块多文件',
  '跨系统多模块',
];

export const DIFFICULTY_OPTIONS = [
  '一般',
  '较难',
  '困难',
];

export const COMPLETION_OPTIONS = [
  '完成了任务',
  '未完成任务',
];

export const SATISFACTION_OPTIONS = [
  '满意',
  '不满意',
];
