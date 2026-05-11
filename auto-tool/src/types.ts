export type Category = string;
export type Model = string;
export type FixCost = 'low' | 'medium' | 'high';
export type GsbWinner = 'A' | 'B' | 'same';

export interface ModelConfig {
  value: string;
  label: string;
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
  models: ModelConfig[];
}

export interface AnalyzeResult {
  rounds?: number;
  ux: number;
  planning: number;
  reasoning: number;
  instruction: number;
  engineering: number;
  issue_types: string[];
  issue_desc: string;
  fix_cost: FixCost | null;
  pros: string;
  analysis: string;
}

export interface Question {
  id: string;
  title: string;
  category: Category;
  difficulty: 1 | 2 | 3;
  prompt: string;
  repo_url: string;
  repo_desc: string;
  tech_stack: string;
  task_direction: string;
  question_direction: string;
  github_repo_name: string;
  github_owner: string;
  github_ssh_url: string;
  default_branch: string;
  branches: string[];
  is_current_focus: boolean;
  requires_long_task_eval: boolean;
  requires_frontend_eval: boolean;
}

export interface Scores {
  ux: number;
  planning: number;
  reasoning: number;
  instruction: number;
  engineering: number;
}

export interface TestRecord {
  id: string;
  question_id: string;
  model: Model;
  branch_name: string;
  session_id: string;
  pr_url: string;
  interaction_rounds: number;
  scores: Scores;
  long_task_score: number | null;
  frontend_3d_score: number | null;
  frontend_aesthetic_score: number | null;
  issue_types: string[];
  issue_desc: string;
  fix_cost: FixCost | null;
  pros: string;
  analysis_summary: string;
  conversation_text: string;
  conversation_file: string;
  created_at: string;
}

export interface GsbRecord {
  id: string;
  question_id: string;
  model_a: Model;
  model_b: Model;
  winner: GsbWinner;
  good: string;
  bad: string;
  note: string;
  reason: string;
  created_at: string;
}

export interface AppData {
  questions: Question[];
  records: TestRecord[];
  gsb_records: GsbRecord[];
}

export interface ValidationIssue {
  level: 'error' | 'warn';
  field: string;
  message: string;
}

export interface DatasetValidation {
  questionIssues: Map<string, ValidationIssue[]>;
  recordIssues: Map<string, ValidationIssue[]>;
  gsbIssues: Map<string, ValidationIssue[]>;
  summary: string;
}

export const CATEGORY_OPTIONS: Category[] = ['SMC', '分布式', '音频', 'VDC'];

export const DEFAULT_MODELS: ModelConfig[] = [
  { value: 'kimi-k2.5', label: 'Kimi K2.5' },
  { value: 'glm-5.0', label: 'GLM 5.0' },
  { value: 'seed-2.0-pro', label: 'seed-2.0-pro-global-minimal' },
];

export const ISSUE_TYPES = [
  '幻觉',
  '上下文丢失',
  '指令遵循失败',
  '死循环',
  '偷懒',
  '代码破坏',
  '代码 Bug',
  '废话过多',
  '输出中断',
  '目标漂移/分心',
  '其他',
];

export const SCORE_LABELS: Record<keyof Scores, string> = {
  ux: '用户体验满意度',
  planning: '规划&执行反馈',
  reasoning: '理解/推理能力',
  instruction: '复杂指令遵循',
  engineering: '工程完备度',
};

export const EXTENDED_SCORE_LABELS = {
  long_task_score: '长程任务',
  frontend_3d_score: '前端 3D 产物',
  frontend_aesthetic_score: '前端产物美观度',
} as const;

export const SCORE_CRITERIA: Record<keyof Scores, { label: string; tips: Record<number, string> }> = {
  ux: {
    label: '用户体验满意度',
    tips: {
      5: '过程高效流畅，几乎无需额外引导，输出可信。',
      4: '整体体验良好，偶有冗余但不影响完成任务。',
      3: '存在重复解释或轻微绕路，需要少量人工跟进。',
      2: '推理拖沓或明显偏题，用户需要频繁纠正。',
      1: '过程失控或陷入循环，无法完成有效交付。',
    },
  },
  planning: {
    label: '规划&执行反馈',
    tips: {
      5: '有清晰计划，状态反馈及时，执行路径合理。',
      4: '计划较完整，阶段反馈基本到位，偶有遗漏。',
      3: '计划和执行都不够稳定，需要用户追问。',
      2: '规划模糊，反馈断裂，行动顺序混乱。',
      1: '几乎没有计划，也没有过程反馈。',
    },
  },
  reasoning: {
    label: '理解/推理能力',
    tips: {
      5: '完整理解上下文和约束，能一次性抓住问题核心。',
      4: '整体理解准确，仅有轻微冗余或小偏差。',
      3: '能完成主线任务，但会遗漏部分细节或约束。',
      2: '误解较多，只能在反复纠正后完成部分任务。',
      1: '持续误解需求，无法有效推进任务。',
    },
  },
  instruction: {
    label: '复杂指令遵循',
    tips: {
      5: '完整遵循多重约束，后续修复也不破坏前提。',
      4: '少量遗漏，提醒后能快速补齐。',
      3: '关键约束执行不稳，需要多次纠偏。',
      2: '多次纠偏后仍无法同时满足多个要求。',
      1: '基本无视约束或陷入错误循环。',
    },
  },
  engineering: {
    label: '工程完备度',
    tips: {
      5: '主动补充验证和工程收尾，交付完整。',
      4: '能覆盖大部分工程环节，但仍有收尾缺口。',
      3: '需要提醒才会补验证，覆盖面有限。',
      2: '验证和收尾都明显不足，工程质量不稳。',
      1: '没有工程意识，产出不可直接使用。',
    },
  },
};

export const DIFFICULTY_OPTIONS = [
  { value: 1, label: '简单' },
  { value: 2, label: '中等' },
  { value: 3, label: '困难' },
] as const;

export const FIX_COST_LABELS: Record<FixCost, string> = {
  low: '低',
  medium: '中',
  high: '高',
};

export let MODELS: ModelConfig[] = [...DEFAULT_MODELS];

export function setRuntimeModels(models: ModelConfig[]) {
  MODELS = models.length > 0 ? models : [...DEFAULT_MODELS];
}

export function getModelLabel(model: string) {
  return MODELS.find((item) => item.value === model)?.label ?? model;
}

export function getQuestionModelOptions(question?: Question | null) {
  if (!question) {
    return MODELS;
  }
  const selected = MODELS.filter((item) => question.branches.includes(item.value));
  return selected.length > 0 ? selected : MODELS;
}

export function buildGsbReason(good: string, bad: string, note: string) {
  return [good, bad, note].filter(Boolean).join('\n\n');
}

