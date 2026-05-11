import {
  AppData,
  DatasetValidation,
  GsbRecord,
  Question,
  SCORE_LABELS,
  Scores,
  TestRecord,
  ValidationIssue,
  buildGsbReason,
} from '../types';

const SCORE_KEYS: (keyof Scores)[] = ['ux', 'planning', 'reasoning', 'instruction', 'engineering'];

function pushIssue(target: Map<string, ValidationIssue[]>, key: string, issue: ValidationIssue) {
  const current = target.get(key) ?? [];
  current.push(issue);
  target.set(key, current);
}

function hasMarkdown(text: string) {
  return [
    /```/,
    /^\s{0,3}#{1,6}\s/m,
    /^\s{0,3}>\s/m,
    /^\s{0,3}[-*_]{3,}\s*$/m,
    /^\s*\|.+\|\s*$/m,
    /!\[[^\]]*]\([^)]+\)/,
    /\[[^\]]+]\([^)]+\)/,
    /(^|[^\\])(\*\*|__)[^*_]+(\*\*|__)/,
    /(^|[^\\])(\*|_)[^*_]+(\*|_)/,
  ].some((pattern) => pattern.test(text));
}

function findQuestion(data: AppData, questionId: string) {
  return data.questions.find((item) => item.id === questionId) ?? null;
}

function totalScore(record: TestRecord) {
  return SCORE_KEYS.reduce((sum, key) => sum + record.scores[key], 0);
}

function normalizedPairKey(questionId: string, modelA: string, modelB: string) {
  return [questionId, ...[modelA, modelB].sort()].join('::');
}

export function validateQuestion(question: Question): ValidationIssue[] {
  const issues: ValidationIssue[] = [];

  if (!question.title.trim()) {
    issues.push({ level: 'error', field: 'title', message: '题目标题不能为空。' });
  }

  if (!question.prompt.trim()) {
    issues.push({ level: 'error', field: 'prompt', message: 'Prompt 不能为空。' });
  } else if (hasMarkdown(question.prompt)) {
    issues.push({ level: 'error', field: 'prompt', message: 'Prompt 不能直接使用 Markdown 格式。' });
  }

  if (!question.tech_stack.trim()) {
    issues.push({ level: 'warn', field: 'tech_stack', message: '建议补充技术栈，方便质检核对 prompt 是否匹配。' });
  }

  if (question.branches.length < 2 || question.branches.length > 3) {
    issues.push({ level: 'error', field: 'branches', message: '每个题目必须选择 2-3 个模型分支。' });
  }

  return issues;
}

export function validateRecord(record: TestRecord, question: Question | null): ValidationIssue[] {
  const issues: ValidationIssue[] = [];

  if (!question) {
    issues.push({ level: 'error', field: 'question_id', message: '测试记录缺少对应题目。' });
    return issues;
  }

  if (!question.branches.includes(record.model)) {
    issues.push({ level: 'error', field: 'model', message: '测试记录模型必须属于题目已选择的模型分支。' });
  }

  if (!record.session_id.trim()) {
    issues.push({ level: 'error', field: 'session_id', message: '必须填写 Session ID。' });
  }

  if (!record.pr_url.trim()) {
    issues.push({ level: 'warn', field: 'pr_url', message: '建议填写 PR 链接。' });
  }

  if (record.scores.ux === 5) {
    if (!record.pros.trim()) {
      issues.push({ level: 'error', field: 'pros', message: '用户体验满意度为 5 分时，必须填写模型优点。' });
    } else {
      const missing = SCORE_KEYS.filter((key) => !record.pros.includes(SCORE_LABELS[key]));
      if (missing.length > 0) {
        issues.push({
          level: 'warn',
          field: 'pros',
          message: `模型优点建议覆盖全部维度，当前缺少：${missing.map((key) => SCORE_LABELS[key]).join('、')}。`,
        });
      }
    }
  } else {
    if (record.issue_types.length === 0) {
      issues.push({ level: 'error', field: 'issue_types', message: '非满分记录必须选择问题类型。' });
    }
    if (!record.issue_desc.trim()) {
      issues.push({ level: 'error', field: 'issue_desc', message: '非满分记录必须填写问题描述。' });
    }
    if (!record.fix_cost) {
      issues.push({ level: 'error', field: 'fix_cost', message: '非满分记录必须填写修复成本。' });
    }

    const missing = SCORE_KEYS
      .filter((key) => record.scores[key] < 5)
      .filter((key) => !record.issue_desc.includes(SCORE_LABELS[key]));
    if (missing.length > 0) {
      issues.push({
        level: 'warn',
        field: 'issue_desc',
        message: `问题描述建议覆盖全部非满分维度，当前缺少：${missing.map((key) => SCORE_LABELS[key]).join('、')}。`,
      });
    }
  }

  if (record.interaction_rounds > 1 && record.scores.reasoning > 3) {
    const hasException = /工程完备度|第二轮|二轮/i.test(record.issue_desc);
    if (!hasException) {
      issues.push({
        level: 'warn',
        field: 'reasoning',
        message: '多轮对话下理解/推理能力超过 3 分，需要在问题描述里说明第二轮是工程完备度修复。',
      });
    }
  }

  if (record.interaction_rounds >= 3 && record.fix_cost !== 'high') {
    issues.push({
      level: 'warn',
      field: 'fix_cost',
      message: '三轮及以上仍未完成修复，修复成本通常应为高。',
    });
  }

  if (question.requires_long_task_eval && record.long_task_score == null) {
    issues.push({ level: 'error', field: 'long_task_score', message: '该题目要求填写长程任务分数。' });
  }

  if (question.requires_frontend_eval) {
    if (record.frontend_3d_score == null) {
      issues.push({ level: 'error', field: 'frontend_3d_score', message: '前端题目必须填写前端 3D 产物分数。' });
    }
    if (record.frontend_aesthetic_score == null) {
      issues.push({ level: 'error', field: 'frontend_aesthetic_score', message: '前端题目必须填写前端产物美观度分数。' });
    }
  }

  return issues;
}

export function validateGsb(gsb: GsbRecord, question: Question | null): ValidationIssue[] {
  const issues: ValidationIssue[] = [];

  if (!question) {
    issues.push({ level: 'error', field: 'question_id', message: 'GSB 缺少对应题目。' });
    return issues;
  }

  if (gsb.model_a === gsb.model_b) {
    issues.push({ level: 'error', field: 'model_pair', message: 'GSB 对比必须选择两个不同模型。' });
  }

  if (!question.branches.includes(gsb.model_a) || !question.branches.includes(gsb.model_b)) {
    issues.push({ level: 'error', field: 'model_pair', message: 'GSB 对比模型必须属于题目已选模型。' });
  }

  if (gsb.winner !== 'same' && !(gsb.good.trim() || gsb.bad.trim() || gsb.reason.trim())) {
    issues.push({ level: 'error', field: 'reason', message: '非 same 的 GSB 必须填写好坏模型分析。' });
  }

  if (gsb.winner === 'same' && !(gsb.note.trim() || gsb.reason.trim())) {
    issues.push({ level: 'warn', field: 'note', message: 'same 结果建议填写备注说明。' });
  }

  return issues;
}

export function validateDataset(data: AppData): DatasetValidation {
  const questionIssues = new Map<string, ValidationIssue[]>();
  const recordIssues = new Map<string, ValidationIssue[]>();
  const gsbIssues = new Map<string, ValidationIssue[]>();

  for (const question of data.questions) {
    const issues = validateQuestion(question);
    if (issues.length > 0) {
      questionIssues.set(question.id, issues);
    }
  }

  for (const record of data.records) {
    const issues = validateRecord(record, findQuestion(data, record.question_id));
    if (issues.length > 0) {
      recordIssues.set(record.id, issues);
    }
  }

  for (const gsb of data.gsb_records) {
    const patched = {
      ...gsb,
      reason: gsb.reason || buildGsbReason(gsb.good, gsb.bad, gsb.note),
    };
    const issues = validateGsb(patched, findQuestion(data, gsb.question_id));
    if (issues.length > 0) {
      gsbIssues.set(gsb.id, issues);
    }
  }

  for (const question of data.questions) {
    const questionRecords = data.records.filter((item) => item.question_id === question.id);
    const questionGsbs = data.gsb_records.filter((item) => item.question_id === question.id);

    for (const branch of question.branches) {
      const matches = questionRecords.filter((item) => item.model === branch);
      if (matches.length === 0) {
        pushIssue(
          questionIssues,
          question.id,
          { level: 'warn', field: 'records', message: `模型 ${branch} 还没有测试记录。` }
        );
      }
      if (matches.length > 1) {
        for (const item of matches) {
          pushIssue(
            recordIssues,
            item.id,
            { level: 'error', field: 'duplicate', message: '同一题目和模型只能保留一条测试记录。' }
          );
        }
      }
    }

    const totals = questionRecords.map(totalScore);
    if (questionRecords.length >= 2 && new Set(totals).size === 1) {
      for (const item of questionRecords) {
        pushIssue(
          recordIssues,
          item.id,
          { level: 'error', field: 'scores', message: '同一题目下所有模型总分完全相同，会被判定为无效数据。' }
        );
      }
    }

    const pairMap = new Map<string, GsbRecord[]>();
    for (const item of questionGsbs) {
      const key = normalizedPairKey(item.question_id, item.model_a, item.model_b);
      const current = pairMap.get(key) ?? [];
      current.push(item);
      pairMap.set(key, current);
    }

    for (const items of pairMap.values()) {
      if (items.length > 1) {
        for (const item of items) {
          pushIssue(
            gsbIssues,
            item.id,
            { level: 'error', field: 'duplicate', message: '同一题目和模型 pair 只能保留一条 GSB 记录。' }
          );
        }
      }
    }

    const sameCount = questionGsbs.filter((item) => item.winner === 'same').length;
    if (sameCount >= 2) {
      for (const item of questionGsbs.filter((entry) => entry.winner === 'same')) {
        pushIssue(
          gsbIssues,
          item.id,
          { level: 'error', field: 'winner', message: '同一题目不能出现两次及以上 same，对比结果会被判无效。' }
        );
      }
    }

    if (question.branches.length === 3 && questionGsbs.length >= 3 && questionGsbs.every((item) => item.winner === 'same')) {
      for (const item of questionGsbs) {
        pushIssue(
          gsbIssues,
          item.id,
          { level: 'error', field: 'winner', message: '三个模型的 GSB 不能全部是 same。' }
        );
      }
    }

    for (const item of questionGsbs) {
      const recordA = questionRecords.find((entry) => entry.model === item.model_a);
      const recordB = questionRecords.find((entry) => entry.model === item.model_b);
      if (!recordA || !recordB) {
        pushIssue(
          gsbIssues,
          item.id,
          { level: 'error', field: 'records', message: 'GSB 对比前需要两侧模型都存在测试记录。' }
        );
        continue;
      }

      const diff = totalScore(recordA) - totalScore(recordB);
      if ((diff > 0 && item.winner === 'B') || (diff < 0 && item.winner === 'A')) {
        pushIssue(
          gsbIssues,
          item.id,
          { level: 'warn', field: 'winner', message: 'GSB 结果与已有分数高低相反，请确认描述与结论一致。' }
        );
      }
    }
  }

  const allIssues = [
    ...questionIssues.values(),
    ...recordIssues.values(),
    ...gsbIssues.values(),
  ].flat();

  const errorCount = allIssues.filter((item) => item.level === 'error').length;
  const warnCount = allIssues.filter((item) => item.level === 'warn').length;
  const summary = errorCount > 0
    ? `${errorCount} 个错误，${warnCount} 个警告`
    : warnCount > 0
      ? `质检通过，但还有 ${warnCount} 个建议`
      : '全部通过';

  return { questionIssues, recordIssues, gsbIssues, summary };
}
