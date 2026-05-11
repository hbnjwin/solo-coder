import { useEffect, useMemo, useState } from 'react';
import ScoreForm from '../components/ScoreForm';
import {
  AppData,
  ModelConfig,
  buildGsbReason,
  CATEGORY_OPTIONS,
  Category,
  DIFFICULTY_OPTIONS,
  FixCost,
  GsbRecord,
  GsbWinner,
  ISSUE_TYPES,
  MODELS,
  Question,
  Question as QuestionType,
  Scores,
  TestRecord,
  getModelLabel,
  setRuntimeModels,
} from '../types';
import {
  SessionInfo,
  analyzeConversation,
  analyzeGsb,
  exportXlsxQuestion,
  generateId,
  getLlmConfig,
  getRecentSessions,
  githubCreateBranch,
  githubCreatePr,
  githubCreateRepo,
  nowIso,
  openFileDialog,
  readTextFile,
  saveLlmConfig,
} from '../store';
import { validateQuestion } from '../utils/validate';

interface Props {
  data: AppData;
  onUpdate: (data: AppData) => void;
}

type WizardStep = 'basic' | 'github';
type QuestionForm = Omit<Question, 'id'>;

const emptyForm = (): QuestionForm => ({
  title: '',
  category: 'SMC',
  difficulty: 1,
  prompt: '',
  repo_url: '',
  repo_desc: '',
  tech_stack: '',
  task_direction: '',
  question_direction: '',
  github_repo_name: '',
  github_owner: '',
  github_ssh_url: '',
  default_branch: 'main',
  branches: [],
  is_current_focus: false,
  requires_long_task_eval: false,
  requires_frontend_eval: false,
});

const defaultScores = (): Scores => ({
  ux: 3,
  planning: 3,
  reasoning: 3,
  instruction: 3,
  engineering: 3,
});

const difficultyColor: Record<number, string> = {
  1: 'text-emerald-600',
  2: 'text-amber-600',
  3: 'text-rose-600',
};

const sessionPathHint = 'APPDATA\\Trae CN\\logs\\<日期>\\window1\\renderer.log 和 Modular\\ai-agent_*_stdout.log';

function buildExpectedPairs(branches: string[]) {
  const pairs: Array<[string, string]> = [];
  for (let i = 0; i < branches.length; i += 1) {
    for (let j = i + 1; j < branches.length; j += 1) {
      pairs.push([branches[i], branches[j]]);
    }
  }
  return pairs;
}

function totalScore(scores: Scores) {
  return Object.values(scores).reduce((sum, value) => sum + value, 0);
}

function pairKey(modelA: string, modelB: string) {
  return [modelA, modelB].sort().join('::');
}

function buildPrTitle(question: QuestionType, model: string) {
  return `[${question.category}] ${question.title} - ${getModelLabel(model)}`;
}

function buildPrBody(question: QuestionType, model: string, sessionId?: string) {
  return [
    `题目：${question.title}`,
    `模型：${getModelLabel(model)} (${model})`,
    `Session ID：${sessionId?.trim() || ''}`,
  ].join('\n');
}

function ExtraScoreSelector({
  label,
  value,
  onChange,
}: {
  label: string;
  value: number | null;
  onChange: (value: number | null) => void;
}) {
  return (
    <div>
      <label className="text-xs text-gray-500 mb-1 block">{label}</label>
      <div className="flex gap-1">
        <button
          onClick={() => onChange(null)}
          className={`px-2 py-1 text-xs rounded border ${value == null ? 'bg-gray-900 text-white border-gray-900' : 'border-gray-200 text-gray-500'}`}
        >
          空
        </button>
        {[1, 2, 3, 4, 5].map((item) => (
          <button
            key={item}
            onClick={() => onChange(item)}
            className={`w-8 h-8 rounded text-xs border ${value === item ? 'bg-blue-600 text-white border-blue-600' : 'border-gray-200 text-gray-600'}`}
          >
            {item}
          </button>
        ))}
      </div>
    </div>
  );
}

export default function Questions({ data, onUpdate }: Props) {
  const [filterCategory, setFilterCategory] = useState<Category | 'all'>('all');
  const [filterDifficulty, setFilterDifficulty] = useState<number | 0>(0);
  const [editing, setEditing] = useState<Question | null>(null);
  const [form, setForm] = useState<QuestionForm>(emptyForm());
  const [showPrompt, setShowPrompt] = useState<string | null>(null);
  const [wizardStep, setWizardStep] = useState<WizardStep>('basic');
  const [selectedModels, setSelectedModels] = useState<string[]>([]);
  const [repoCreating, setRepoCreating] = useState(false);
  const [repoCreated, setRepoCreated] = useState(false);
  const [branchCreating, setBranchCreating] = useState(false);
  const [branchesCreated, setBranchesCreated] = useState<string[]>([]);
  const [isPrivate, setIsPrivate] = useState(false);
  const [githubError, setGithubError] = useState('');
  const [modelConfigs, setModelConfigs] = useState<ModelConfig[]>(MODELS);
  const [dictOpen, setDictOpen] = useState(false);
  const [dictSaving, setDictSaving] = useState(false);
  const [dictError, setDictError] = useState('');

  const [expandedRecords, setExpandedRecords] = useState<Record<string, boolean>>({});
  const [expandedGsbs, setExpandedGsbs] = useState<Record<string, boolean>>({});
  const [xlsxExporting, setXlsxExporting] = useState<Record<string, boolean>>({});

  const [quickRecordQuestion, setQuickRecordQuestion] = useState<QuestionType | null>(null);
  const [quickRecordModel, setQuickRecordModel] = useState('');
  const [quickRecordSessionId, setQuickRecordSessionId] = useState('');
  const [quickRecordPrUrl, setQuickRecordPrUrl] = useState('');
  const [quickRecordRounds, setQuickRecordRounds] = useState(1);
  const [quickRecordScores, setQuickRecordScores] = useState<Scores>(defaultScores());
  const [quickRecordLongTask, setQuickRecordLongTask] = useState<number | null>(null);
  const [quickRecordFrontend3d, setQuickRecordFrontend3d] = useState<number | null>(null);
  const [quickRecordFrontendAesthetic, setQuickRecordFrontendAesthetic] = useState<number | null>(null);
  const [quickRecordIssueTypes, setQuickRecordIssueTypes] = useState<string[]>([]);
  const [quickRecordIssueDesc, setQuickRecordIssueDesc] = useState('');
  const [quickRecordFixCost, setQuickRecordFixCost] = useState<FixCost | null>(null);
  const [quickRecordPros, setQuickRecordPros] = useState('');
  const [quickRecordConversationMode, setQuickRecordConversationMode] = useState<'text' | 'file'>('text');
  const [quickRecordConversationText, setQuickRecordConversationText] = useState('');
  const [quickRecordConversationFile, setQuickRecordConversationFile] = useState('');
  const [quickRecordAnalysisSummary, setQuickRecordAnalysisSummary] = useState('');
  const [quickRecordAnalyzing, setQuickRecordAnalyzing] = useState(false);
  const [quickRecordAnalyzeError, setQuickRecordAnalyzeError] = useState('');
  const [quickRecordFileLoading, setQuickRecordFileLoading] = useState(false);
  const [quickRecordSessions, setQuickRecordSessions] = useState<SessionInfo[]>([]);
  const [quickRecordSessionsLoading, setQuickRecordSessionsLoading] = useState(false);
  const [quickRecordSessionsError, setQuickRecordSessionsError] = useState('');
  const [quickRecordShowSessionPicker, setQuickRecordShowSessionPicker] = useState(false);
  const [quickRecordPrCreating, setQuickRecordPrCreating] = useState(false);
  const [quickRecordPrError, setQuickRecordPrError] = useState('');

  const [quickGsbQuestion, setQuickGsbQuestion] = useState<QuestionType | null>(null);
  const [quickGsbModelA, setQuickGsbModelA] = useState('');
  const [quickGsbModelB, setQuickGsbModelB] = useState('');
  const [quickGsbWinner, setQuickGsbWinner] = useState<GsbWinner>('same');
  const [quickGsbGood, setQuickGsbGood] = useState('');
  const [quickGsbBad, setQuickGsbBad] = useState('');
  const [quickGsbNote, setQuickGsbNote] = useState('');
  const [quickGsbAnalyzing, setQuickGsbAnalyzing] = useState(false);
  const [quickGsbAnalyzeError, setQuickGsbAnalyzeError] = useState('');

  const filtered = useMemo(() => data.questions.filter((question) => (
    (filterCategory === 'all' || question.category === filterCategory) &&
    (filterDifficulty === 0 || question.difficulty === filterDifficulty)
  )), [data.questions, filterCategory, filterDifficulty]);

  useEffect(() => {
    getLlmConfig().then((config) => {
      const nextModels = config.models && config.models.length > 0 ? config.models : MODELS;
      setModelConfigs(nextModels);
      setRuntimeModels(nextModels);
    });
  }, []);

  function resetEditor() {
    setEditing(null);
    setForm(emptyForm());
    setSelectedModels([]);
    setRepoCreated(false);
    setBranchesCreated([]);
    setGithubError('');
    setWizardStep('basic');
    setIsPrivate(false);
  }

  function openNew() {
    resetEditor();
    setEditing({ id: '', ...emptyForm() });
  }

  function openEdit(question: Question) {
    setEditing(question);
    setForm({ ...question });
    setSelectedModels(question.branches);
    setRepoCreated(Boolean(question.github_repo_name));
    setBranchesCreated(question.branches);
    setGithubError('');
    setWizardStep('basic');
  }

  function toggleModel(model: string) {
    setSelectedModels((current) => {
      if (current.includes(model)) {
        return current.filter((item) => item !== model);
      }
      if (current.length >= 3) {
        return current;
      }
      return [...current, model];
    });
  }

  function updateModelConfig(index: number, patch: Partial<ModelConfig>) {
    setModelConfigs((current) => current.map((item, itemIndex) => itemIndex === index ? { ...item, ...patch } : item));
  }

  function addModelConfig() {
    setModelConfigs((current) => [...current, { value: '', label: '' }]);
    setDictOpen(true);
  }

  function removeModelConfig(index: number) {
    const target = modelConfigs[index];
    setModelConfigs((current) => current.filter((_, itemIndex) => itemIndex !== index));
    if (target) {
      setSelectedModels((current) => current.filter((item) => item !== target.value));
    }
  }

  async function saveModelDictionary() {
    const normalized = modelConfigs
      .map((item) => ({ value: item.value.trim(), label: item.label.trim() }))
      .filter((item) => item.value || item.label);

    if (normalized.length === 0) {
      setDictError('至少保留一个模型。');
      return;
    }
    if (normalized.some((item) => !item.value || !item.label)) {
      setDictError('模型字典中的 value 和 label 都必须填写。');
      return;
    }
    if (new Set(normalized.map((item) => item.value)).size !== normalized.length) {
      setDictError('模型 value 不能重复。');
      return;
    }

    setDictSaving(true);
    setDictError('');
    const config = await getLlmConfig();
    await saveLlmConfig({ ...config, models: normalized });
    setRuntimeModels(normalized);
    setModelConfigs(normalized);
    setSelectedModels((current) => current.filter((item) => normalized.some((model) => model.value === item)));
    setDictSaving(false);
    setDictOpen(false);
  }

  async function handleCreateRepo() {
    if (!form.github_repo_name.trim()) {
      return;
    }
    setRepoCreating(true);
    setGithubError('');
    const result = await githubCreateRepo(
      form.github_repo_name.trim(),
      form.repo_desc || form.title,
      isPrivate
    );
    if (result.ok) {
      const owner = result.repo.full_name.split('/')[0] ?? '';
      setForm((current) => ({
        ...current,
        github_owner: owner,
        repo_url: result.repo.html_url,
        github_ssh_url: result.repo.ssh_url,
        default_branch: result.repo.default_branch || 'main',
      }));
      setRepoCreated(true);
    } else {
      setGithubError(result.error);
    }
    setRepoCreating(false);
  }

  async function handleCreateBranches() {
    if (!form.github_owner || !form.github_repo_name || selectedModels.length < 2) {
      return;
    }
    setBranchCreating(true);
    setGithubError('');
    const created: string[] = [];
    for (const branch of selectedModels) {
      const result = await githubCreateBranch(
        form.github_owner,
        form.github_repo_name,
        branch,
        form.default_branch || 'main'
      );
      if (!result.ok) {
        setGithubError(`创建分支 ${branch} 失败：${result.error}`);
        setBranchCreating(false);
        return;
      }
      created.push(branch);
    }
    setBranchesCreated(created);
    setForm((current) => ({ ...current, branches: created }));
    setBranchCreating(false);
  }

  function save() {
    const nextBranches = branchesCreated.length > 0 ? branchesCreated : selectedModels;
    const nextQuestion: Question = {
      id: editing?.id || generateId(),
      ...form,
      branches: nextBranches,
    };

    const issues = validateQuestion(nextQuestion).filter((item) => item.level === 'error');
    if (issues.length > 0) {
      alert(issues.map((item) => item.message).join('\n'));
      return;
    }

    if (editing?.id) {
      onUpdate({
        ...data,
        questions: data.questions.map((item) => item.id === editing.id ? nextQuestion : item),
      });
    } else {
      onUpdate({ ...data, questions: [...data.questions, nextQuestion] });
    }
    resetEditor();
  }

  function remove(questionId: string) {
    if (!confirm('删除题目后会同时删除该题目下的测试记录和 GSB，对吗？')) {
      return;
    }
    onUpdate({
      ...data,
      questions: data.questions.filter((item) => item.id !== questionId),
      records: data.records.filter((item) => item.question_id !== questionId),
      gsb_records: data.gsb_records.filter((item) => item.question_id !== questionId),
    });
  }

  async function handleExportQuestionXlsx(questionId: string, questionTitle: string) {
    setXlsxExporting((current) => ({ ...current, [questionId]: true }));
    try {
      const bytes = await exportXlsxQuestion(questionId);
      const blob = new Blob([new Uint8Array(bytes)], {
        type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
      });
      const url = URL.createObjectURL(blob);
      const anchor = document.createElement('a');
      anchor.href = url;
      anchor.download = `${questionTitle}_${new Date().toISOString().slice(0, 10)}.xlsx`;
      anchor.click();
      URL.revokeObjectURL(url);
    } finally {
      setXlsxExporting((current) => ({ ...current, [questionId]: false }));
    }
  }

  function toggleRecordPanel(questionId: string) {
    setExpandedRecords((current) => ({ ...current, [questionId]: !current[questionId] }));
  }

  function toggleGsbPanel(questionId: string) {
    setExpandedGsbs((current) => ({ ...current, [questionId]: !current[questionId] }));
  }

  function resetQuickRecord() {
    setQuickRecordQuestion(null);
    setQuickRecordModel('');
    setQuickRecordSessionId('');
    setQuickRecordPrUrl('');
    setQuickRecordRounds(1);
    setQuickRecordScores(defaultScores());
    setQuickRecordLongTask(null);
    setQuickRecordFrontend3d(null);
    setQuickRecordFrontendAesthetic(null);
    setQuickRecordIssueTypes([]);
    setQuickRecordIssueDesc('');
    setQuickRecordFixCost(null);
    setQuickRecordPros('');
    setQuickRecordConversationMode('text');
    setQuickRecordConversationText('');
    setQuickRecordConversationFile('');
    setQuickRecordAnalysisSummary('');
    setQuickRecordAnalyzing(false);
    setQuickRecordAnalyzeError('');
    setQuickRecordFileLoading(false);
    setQuickRecordSessions([]);
    setQuickRecordSessionsLoading(false);
    setQuickRecordSessionsError('');
    setQuickRecordShowSessionPicker(false);
    setQuickRecordPrCreating(false);
    setQuickRecordPrError('');
  }

  function openQuickRecord(question: QuestionType, model?: string) {
    const targetModel = model || question.branches[0] || '';
    const existing = data.records.find((item) => item.question_id === question.id && item.model === targetModel);
    setQuickRecordQuestion(question);
    setQuickRecordModel(targetModel);
    setQuickRecordSessionId(existing?.session_id || '');
    setQuickRecordPrUrl(existing?.pr_url || '');
    setQuickRecordRounds(existing?.interaction_rounds || 1);
    setQuickRecordScores(existing?.scores || defaultScores());
    setQuickRecordLongTask(existing?.long_task_score ?? null);
    setQuickRecordFrontend3d(existing?.frontend_3d_score ?? null);
    setQuickRecordFrontendAesthetic(existing?.frontend_aesthetic_score ?? null);
    setQuickRecordIssueTypes(existing?.issue_types || []);
    setQuickRecordIssueDesc(existing?.issue_desc || '');
    setQuickRecordFixCost(existing?.fix_cost ?? null);
    setQuickRecordPros(existing?.pros || '');
    setQuickRecordConversationMode(existing?.conversation_file ? 'file' : 'text');
    setQuickRecordConversationText(existing?.conversation_text || '');
    setQuickRecordConversationFile(existing?.conversation_file || '');
    setQuickRecordAnalysisSummary(existing?.analysis_summary || '');
    setQuickRecordAnalyzeError('');
    setQuickRecordSessions([]);
    setQuickRecordSessionsError('');
    setQuickRecordShowSessionPicker(false);
    setQuickRecordPrError('');
  }

  async function handleQuickFetchSessions() {
    setQuickRecordSessionsLoading(true);
    setQuickRecordShowSessionPicker(true);
    setQuickRecordSessionsError('');
    const result = await getRecentSessions();
    if (result.ok) {
      setQuickRecordSessions(result.data);
    } else {
      setQuickRecordSessions([]);
      setQuickRecordSessionsError(result.error);
    }
    setQuickRecordSessionsLoading(false);
  }

  async function runQuickRecordAnalysis(conversation: string) {
    if (!quickRecordQuestion || !conversation.trim()) {
      return;
    }
    setQuickRecordAnalyzing(true);
    setQuickRecordAnalyzeError('');
    try {
      const result = await analyzeConversation(quickRecordQuestion.prompt, conversation);
      if (result.rounds && result.rounds > 0) {
        setQuickRecordRounds(result.rounds);
      }
      setQuickRecordScores({
        ux: result.ux,
        planning: result.planning,
        reasoning: result.reasoning,
        instruction: result.instruction,
        engineering: result.engineering,
      });
      setQuickRecordIssueTypes(result.issue_types ?? []);
      setQuickRecordIssueDesc(result.issue_desc ?? '');
      setQuickRecordFixCost(result.fix_cost ?? null);
      setQuickRecordPros(result.pros ?? '');
      setQuickRecordAnalysisSummary(result.analysis ?? '');
    } catch (error) {
      setQuickRecordAnalyzeError(String(error));
    } finally {
      setQuickRecordAnalyzing(false);
    }
  }

  async function handleQuickPickFile() {
    setQuickRecordFileLoading(true);
    try {
      const path = await openFileDialog();
      if (!path) {
        return;
      }
      setQuickRecordConversationFile(path);
      const text = await readTextFile(path);
      await runQuickRecordAnalysis(text);
    } catch (error) {
      setQuickRecordAnalyzeError(String(error));
    } finally {
      setQuickRecordFileLoading(false);
    }
  }

  async function handleQuickCreatePr() {
    if (!quickRecordQuestion?.github_owner || !quickRecordQuestion.github_repo_name) {
      return '';
    }
    setQuickRecordPrCreating(true);
    setQuickRecordPrError('');
    const result = await githubCreatePr(
      quickRecordQuestion.github_owner,
      quickRecordQuestion.github_repo_name,
      quickRecordModel,
      buildPrTitle(quickRecordQuestion, quickRecordModel),
      buildPrBody(quickRecordQuestion, quickRecordModel, quickRecordSessionId),
      quickRecordQuestion.default_branch || 'main'
    );
    if (result.ok) {
      setQuickRecordPrUrl(result.url);
      setQuickRecordPrCreating(false);
      return result.url;
    }
    setQuickRecordPrError(result.error);
    setQuickRecordPrCreating(false);
    return '';
  }

  async function handleQuickImportJson() {
    try {
      const path = await openFileDialog();
      if (!path) return;
      const text = await readTextFile(path);
      const parsed = JSON.parse(text);
      if (parsed.model) setQuickRecordModel(parsed.model);
      if (parsed.session_id) setQuickRecordSessionId(parsed.session_id);
      if (parsed.pr_url) setQuickRecordPrUrl(parsed.pr_url);
      if (parsed.interaction_rounds) setQuickRecordRounds(parsed.interaction_rounds);
      if (parsed.scores) setQuickRecordScores({ ...defaultScores(), ...parsed.scores });
      if (parsed.long_task_score != null) setQuickRecordLongTask(parsed.long_task_score);
      if (parsed.frontend_3d_score != null) setQuickRecordFrontend3d(parsed.frontend_3d_score);
      if (parsed.frontend_aesthetic_score != null) setQuickRecordFrontendAesthetic(parsed.frontend_aesthetic_score);
      if (parsed.issue_types) setQuickRecordIssueTypes(parsed.issue_types);
      if (parsed.issue_desc) setQuickRecordIssueDesc(parsed.issue_desc);
      if (parsed.fix_cost) setQuickRecordFixCost(parsed.fix_cost);
      if (parsed.pros) setQuickRecordPros(parsed.pros);
      if (parsed.analysis_summary) setQuickRecordAnalysisSummary(parsed.analysis_summary);
      if (parsed.conversation_text) {
        setQuickRecordConversationText(parsed.conversation_text);
        setQuickRecordConversationMode('text');
      }
      if (parsed.conversation_file) {
        setQuickRecordConversationFile(parsed.conversation_file);
        setQuickRecordConversationMode('file');
      }
    } catch (error) {
      alert(`导入失败：${String(error)}`);
    }
  }

  async function saveQuickRecord() {
    if (!quickRecordQuestion || !quickRecordModel || !quickRecordSessionId.trim()) {
      alert('请至少填写模型和 Session ID。');
      return;
    }
    const existing = data.records.find((item) => item.question_id === quickRecordQuestion.id && item.model === quickRecordModel);
    let resolvedPrUrl = quickRecordPrUrl.trim() || existing?.pr_url?.trim() || '';
    if (!resolvedPrUrl && quickRecordQuestion.github_owner && quickRecordQuestion.github_repo_name) {
      const createdPrUrl = await handleQuickCreatePr();
      if (!createdPrUrl) {
        return;
      }
      resolvedPrUrl = createdPrUrl;
    }
    const uxSatisfied = quickRecordScores.ux === 5;
    const nextRecord: TestRecord = {
      id: generateId(),
      question_id: quickRecordQuestion.id,
      model: quickRecordModel,
      branch_name: quickRecordModel,
      session_id: quickRecordSessionId.trim(),
      pr_url: resolvedPrUrl,
      interaction_rounds: quickRecordRounds,
      scores: quickRecordScores,
      long_task_score: quickRecordQuestion.requires_long_task_eval ? quickRecordLongTask : null,
      frontend_3d_score: quickRecordQuestion.requires_frontend_eval ? quickRecordFrontend3d : null,
      frontend_aesthetic_score: quickRecordQuestion.requires_frontend_eval ? quickRecordFrontendAesthetic : null,
      issue_types: uxSatisfied ? [] : quickRecordIssueTypes,
      issue_desc: uxSatisfied ? '' : quickRecordIssueDesc.trim(),
      fix_cost: uxSatisfied ? null : quickRecordFixCost,
      pros: uxSatisfied ? quickRecordPros.trim() : '',
      analysis_summary: quickRecordAnalysisSummary.trim(),
      conversation_text: quickRecordConversationMode === 'text' ? quickRecordConversationText : '',
      conversation_file: quickRecordConversationMode === 'file' ? quickRecordConversationFile : '',
      created_at: nowIso(),
    };
    const records = existing
      ? data.records.map((item) => item.id === existing.id ? { ...nextRecord, id: existing.id } : item)
      : [...data.records, nextRecord];
    onUpdate({ ...data, records });
    setExpandedRecords((current) => ({ ...current, [quickRecordQuestion.id]: true }));
    resetQuickRecord();
  }

  function removeRecord(recordId: string) {
    if (!confirm('确认删除这条测试记录吗？')) {
      return;
    }
    onUpdate({ ...data, records: data.records.filter((item) => item.id !== recordId) });
  }

  function resetQuickGsb() {
    setQuickGsbQuestion(null);
    setQuickGsbModelA('');
    setQuickGsbModelB('');
    setQuickGsbWinner('same');
    setQuickGsbGood('');
    setQuickGsbBad('');
    setQuickGsbNote('');
    setQuickGsbAnalyzing(false);
    setQuickGsbAnalyzeError('');
  }

  function openQuickGsb(question: QuestionType, pair?: [string, string]) {
    const branches = question.branches;
    setQuickGsbQuestion(question);
    setQuickGsbModelA(pair?.[0] || branches[0] || '');
    setQuickGsbModelB(pair?.[1] || branches[1] || branches[0] || '');
    setQuickGsbWinner('same');
    setQuickGsbGood('');
    setQuickGsbBad('');
    setQuickGsbNote('');
    setQuickGsbAnalyzeError('');
  }

  async function handleQuickGsbAnalyze() {
    if (!quickGsbQuestion) return;
    const recA = data.records.find((r) => r.question_id === quickGsbQuestion.id && r.model === quickGsbModelA);
    const recB = data.records.find((r) => r.question_id === quickGsbQuestion.id && r.model === quickGsbModelB);
    if (!recA || !recB) return;
    setQuickGsbAnalyzing(true);
    setQuickGsbAnalyzeError('');
    try {
      const result = await analyzeGsb(
        quickGsbQuestion.title,
        getModelLabel(quickGsbModelA),
        getModelLabel(quickGsbModelB),
        recA,
        recB
      );
      setQuickGsbWinner(result.winner as GsbWinner);
      setQuickGsbGood(result.good);
      setQuickGsbBad(result.bad);
      setQuickGsbNote(result.note);
    } catch (error) {
      setQuickGsbAnalyzeError(String(error));
    } finally {
      setQuickGsbAnalyzing(false);
    }
  }

  function saveQuickGsb() {
    if (!quickGsbQuestion || !quickGsbModelA || !quickGsbModelB || quickGsbModelA === quickGsbModelB) {
      alert('请选择两个不同模型。');
      return;
    }
    const nextRecord: GsbRecord = {
      id: generateId(),
      question_id: quickGsbQuestion.id,
      model_a: quickGsbModelA,
      model_b: quickGsbModelB,
      winner: quickGsbWinner,
      good: quickGsbGood.trim(),
      bad: quickGsbBad.trim(),
      note: quickGsbNote.trim(),
      reason: buildGsbReason(quickGsbGood.trim(), quickGsbBad.trim(), quickGsbNote.trim()),
      created_at: nowIso(),
    };
    const existing = data.gsb_records.find((item) => item.question_id === quickGsbQuestion.id && pairKey(item.model_a, item.model_b) === pairKey(quickGsbModelA, quickGsbModelB));
    const gsbRecords = existing
      ? data.gsb_records.map((item) => item.id === existing.id ? { ...nextRecord, id: existing.id } : item)
      : [...data.gsb_records, nextRecord];
    onUpdate({ ...data, gsb_records: gsbRecords });
    setExpandedGsbs((current) => ({ ...current, [quickGsbQuestion.id]: true }));
    resetQuickGsb();
  }

  function removeGsb(gsbId: string) {
    if (!confirm('确认删除这条 GSB 对比吗？')) {
      return;
    }
    onUpdate({ ...data, gsb_records: data.gsb_records.filter((item) => item.id !== gsbId) });
  }

  return (
    <div className="p-4 max-w-6xl mx-auto">
      <div className="flex items-center gap-3 mb-4">
        <select
          value={filterCategory}
          onChange={(event) => setFilterCategory(event.target.value as Category | 'all')}
          className="border border-gray-300 rounded px-2 py-1 text-sm"
        >
          <option value="all">全部分类</option>
          {CATEGORY_OPTIONS.map((category) => (
            <option key={category} value={category}>{category}</option>
          ))}
        </select>
        <select
          value={filterDifficulty}
          onChange={(event) => setFilterDifficulty(Number(event.target.value))}
          className="border border-gray-300 rounded px-2 py-1 text-sm"
        >
          <option value={0}>全部难度</option>
          {DIFFICULTY_OPTIONS.map((item) => (
            <option key={item.value} value={item.value}>{item.label}</option>
          ))}
        </select>
        <span className="text-sm text-gray-400">{filtered.length} 个题目</span>
        <button
          onClick={openNew}
          className="ml-auto bg-blue-600 text-white px-3 py-1.5 rounded text-sm hover:bg-blue-700"
        >
          + 新建题目
        </button>
      </div>

      <div className="space-y-3">
        {filtered.map((question) => {
          const questionRecords = data.records.filter((item) => item.question_id === question.id);
          const questionGsbs = data.gsb_records.filter((item) => item.question_id === question.id);
          const expectedPairs = buildExpectedPairs(question.branches);
          const recordExpanded = Boolean(expandedRecords[question.id]);
          const gsbExpanded = Boolean(expandedGsbs[question.id]);

          return (
            <div key={question.id} className="bg-white border border-gray-200 rounded-xl overflow-hidden">
              <div className="p-4 border-b border-gray-100">
                <div className="flex items-start gap-3">
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-2 flex-wrap">
                      <span className="text-xs bg-blue-100 text-blue-700 px-1.5 py-0.5 rounded">{question.category}</span>
                      <span className={`text-xs font-medium ${difficultyColor[question.difficulty]}`}>
                        {DIFFICULTY_OPTIONS.find((item) => item.value === question.difficulty)?.label}
                      </span>
                      {question.is_current_focus && (
                        <span className="text-xs bg-amber-100 text-amber-700 px-1.5 py-0.5 rounded">本期重点</span>
                      )}
                      {question.requires_long_task_eval && (
                        <span className="text-xs bg-slate-100 text-slate-700 px-1.5 py-0.5 rounded">长程任务</span>
                      )}
                      {question.requires_frontend_eval && (
                        <span className="text-xs bg-emerald-100 text-emerald-700 px-1.5 py-0.5 rounded">前端题</span>
                      )}
                      <span className="text-xs text-gray-400">{question.id}</span>
                    </div>
                    <div className="font-semibold text-sm">{question.title}</div>
                    <div className="text-xs text-gray-500 mt-1">{question.tech_stack || '未填写技术栈'}</div>
                    <div className="flex flex-wrap gap-1 mt-2">
                      {question.branches.map((branch) => (
                        <span key={branch} className="text-xs bg-purple-50 text-purple-700 px-1.5 py-0.5 rounded font-mono">
                          {branch}
                        </span>
                      ))}
                    </div>
                    {(question.repo_url || question.github_ssh_url) && (
                      <div className="mt-2 space-y-1">
                        {question.repo_url && (
                          <a
                            href={question.repo_url}
                            target="_blank"
                            rel="noreferrer"
                            className="block text-xs text-blue-600 hover:underline truncate"
                          >
                            Repo: {question.repo_url}
                          </a>
                        )}
                        {question.github_ssh_url && (
                          <div className="text-xs text-gray-500 font-mono break-all">{question.github_ssh_url}</div>
                        )}
                      </div>
                    )}
                  </div>

                  <div className="flex gap-2 shrink-0">
                    <button
                      onClick={() => setShowPrompt(showPrompt === question.id ? null : question.id)}
                      className="text-xs text-gray-500 hover:text-blue-600 px-2 py-1 border border-gray-200 rounded"
                    >
                      {showPrompt === question.id ? '收起 Prompt' : '查看 Prompt'}
                    </button>
                    <button
                      onClick={() => handleExportQuestionXlsx(question.id, question.title)}
                      disabled={xlsxExporting[question.id]}
                      className="text-xs text-gray-500 hover:text-orange-600 px-2 py-1 border border-gray-200 rounded disabled:opacity-50"
                    >
                      {xlsxExporting[question.id] ? '导出中...' : '导出 xlsx'}
                    </button>
                    <button
                      onClick={() => openEdit(question)}
                      className="text-xs text-gray-500 hover:text-blue-600 px-2 py-1 border border-gray-200 rounded"
                    >
                      编辑
                    </button>
                    <button
                      onClick={() => remove(question.id)}
                      className="text-xs text-gray-500 hover:text-red-600 px-2 py-1 border border-gray-200 rounded"
                    >
                      删除
                    </button>
                  </div>
                </div>

                {showPrompt === question.id && (
                  <pre className="mt-3 bg-gray-50 border border-gray-200 rounded p-3 text-xs whitespace-pre-wrap font-mono leading-relaxed">
                    {question.prompt}
                  </pre>
                )}
              </div>

              <div className="grid md:grid-cols-2 gap-4 p-4 bg-gray-50/60">
                <div className="bg-white border border-gray-200 rounded-lg">
                  <div className="flex items-center gap-2 px-3 py-2 border-b border-gray-100">
                    <button
                      onClick={() => toggleRecordPanel(question.id)}
                      className="text-xs font-semibold text-gray-700 hover:text-blue-600"
                    >
                      {recordExpanded ? '收起' : '展开'} 测试记录 {questionRecords.length}/{question.branches.length}
                    </button>
                    <button
                      onClick={() => openQuickRecord(question)}
                      className="ml-auto text-xs bg-blue-600 text-white px-2 py-1 rounded hover:bg-blue-700"
                    >
                      + 新增
                    </button>
                  </div>

                  {recordExpanded && (
                    <div className="p-3 space-y-2">
                      {question.branches.map((branch) => {
                        const record = questionRecords.find((item) => item.model === branch);
                        return (
                          <div key={branch} className="border border-gray-200 rounded-lg p-3">
                            <div className="flex items-center gap-2">
                              <span className="text-xs font-mono bg-purple-50 text-purple-700 px-1.5 py-0.5 rounded">
                                {branch}
                              </span>
                              <span className="text-xs text-gray-500">{getModelLabel(branch)}</span>
                              <span className={`ml-auto text-xs ${record ? 'text-emerald-600' : 'text-amber-600'}`}>
                                {record ? '已完成' : '待补充'}
                              </span>
                            </div>

                            {record ? (
                              <div className="mt-2 text-xs text-gray-500 space-y-1">
                                <div className="break-all">Session: {record.session_id}</div>
                                <div>轮次: {record.interaction_rounds}，总分: {totalScore(record.scores)}</div>
                                {record.pr_url && (
                                  <a href={record.pr_url} target="_blank" rel="noreferrer" className="text-blue-600 hover:underline break-all block">
                                    {record.pr_url}
                                  </a>
                                )}
                                <div className="flex gap-2 pt-1">
                                  <button
                                    onClick={() => openQuickRecord(question, branch)}
                                    className="text-xs text-blue-600 hover:underline"
                                  >
                                    覆盖保存
                                  </button>
                                  <button
                                    onClick={() => removeRecord(record.id)}
                                    className="text-xs text-red-600 hover:underline"
                                  >
                                    删除
                                  </button>
                                </div>
                              </div>
                            ) : (
                              <div className="mt-2 flex items-center justify-between gap-2">
                                <div className="text-xs text-gray-400">这个模型还没有测试记录。</div>
                                <button
                                  onClick={() => openQuickRecord(question, branch)}
                                  className="text-xs text-blue-600 hover:underline"
                                >
                                  立即新增
                                </button>
                              </div>
                            )}
                          </div>
                        );
                      })}
                    </div>
                  )}
                </div>

                <div className="bg-white border border-gray-200 rounded-lg">
                  <div className="flex items-center gap-2 px-3 py-2 border-b border-gray-100">
                    <button
                      onClick={() => toggleGsbPanel(question.id)}
                      className="text-xs font-semibold text-gray-700 hover:text-blue-600"
                    >
                      {gsbExpanded ? '收起' : '展开'} GSB 对比 {questionGsbs.length}/{expectedPairs.length}
                    </button>
                    <button
                      onClick={() => openQuickGsb(question)}
                      className="ml-auto text-xs bg-blue-600 text-white px-2 py-1 rounded hover:bg-blue-700"
                    >
                      + 新增
                    </button>
                  </div>

                  {gsbExpanded && (
                    <div className="p-3 space-y-2">
                      {expectedPairs.map(([modelA, modelB]) => {
                        const gsb = questionGsbs.find((item) => pairKey(item.model_a, item.model_b) === pairKey(modelA, modelB));
                        return (
                          <div key={`${modelA}-${modelB}`} className="border border-gray-200 rounded-lg p-3">
                            <div className="flex items-center gap-2 text-xs">
                              <span className="bg-blue-50 text-blue-700 px-1.5 py-0.5 rounded">{getModelLabel(modelA)}</span>
                              <span className="text-gray-400">vs</span>
                              <span className="bg-purple-50 text-purple-700 px-1.5 py-0.5 rounded">{getModelLabel(modelB)}</span>
                              <span className={`ml-auto ${gsb ? 'text-emerald-600' : 'text-amber-600'}`}>
                                {gsb ? '已完成' : '待补充'}
                              </span>
                            </div>

                            {gsb ? (
                              <div className="mt-2 text-xs text-gray-500 space-y-1">
                                <div>
                                  结果：{gsb.winner === 'A' ? getModelLabel(gsb.model_a) : gsb.winner === 'B' ? getModelLabel(gsb.model_b) : 'same'}
                                </div>
                                <div className="flex gap-2 pt-1">
                                  <button
                                    onClick={() => openQuickGsb(question, [modelA, modelB])}
                                    className="text-xs text-blue-600 hover:underline"
                                  >
                                    覆盖保存
                                  </button>
                                  <button
                                    onClick={() => removeGsb(gsb.id)}
                                    className="text-xs text-red-600 hover:underline"
                                  >
                                    删除
                                  </button>
                                </div>
                              </div>
                            ) : (
                              <div className="mt-2 flex items-center justify-between gap-2">
                                <div className="text-xs text-gray-400">这个模型 pair 还没有 GSB。</div>
                                <button
                                  onClick={() => openQuickGsb(question, [modelA, modelB])}
                                  className="text-xs text-blue-600 hover:underline"
                                >
                                  立即新增
                                </button>
                              </div>
                            )}
                          </div>
                        );
                      })}
                    </div>
                  )}
                </div>
              </div>
            </div>
          );
        })}
      </div>

      {quickRecordQuestion && (
        <div className="fixed inset-0 bg-black/40 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-xl shadow-xl w-full max-w-3xl max-h-[90vh] overflow-y-auto p-6">
            <div className="flex items-center gap-3 mb-5">
              <h2 className="font-semibold text-base flex-1">新增测试记录</h2>
              <div className="text-xs text-gray-400">{quickRecordQuestion.title}</div>
            </div>

            <div className="space-y-4">
              <div className="grid md:grid-cols-[1fr,1.3fr,110px] gap-3">
                <div>
                  <label className="text-xs text-gray-500 mb-1 block">模型</label>
                  <select
                    value={quickRecordModel}
                    onChange={(event) => openQuickRecord(quickRecordQuestion, event.target.value)}
                    className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                  >
                    {quickRecordQuestion.branches.map((branch) => (
                      <option key={branch} value={branch}>{getModelLabel(branch)}</option>
                    ))}
                  </select>
                </div>
                <div>
                  <label className="text-xs text-gray-500 mb-1 block">PR 链接</label>
                  <input
                    value={quickRecordPrUrl}
                    onChange={(event) => setQuickRecordPrUrl(event.target.value)}
                    className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                    placeholder="https://github.com/..."
                  />
                  {quickRecordQuestion.github_owner && quickRecordQuestion.branches.includes(quickRecordModel) && (
                    <button
                      onClick={handleQuickCreatePr}
                      disabled={quickRecordPrCreating}
                      className="mt-2 px-2 py-1.5 text-xs bg-purple-600 text-white rounded hover:bg-purple-700 disabled:opacity-40"
                      >
                      {quickRecordPrCreating ? '创建中...' : '自动 PR'}
                    </button>
                  )}
                  <div className="text-[11px] text-gray-400 mt-1">可留空，点击“自动 PR”或保存记录时自动创建并回填。</div>
                  {quickRecordPrError && <div className="text-xs text-red-600 mt-1 break-all">{quickRecordPrError}</div>}
                </div>
                <div>
                  <label className="text-xs text-gray-500 mb-1 block">轮次</label>
                  <input
                    type="number"
                    min={1}
                    max={20}
                    value={quickRecordRounds}
                    onChange={(event) => setQuickRecordRounds(Math.max(1, Number(event.target.value) || 1))}
                    className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm text-center"
                  />
                </div>
              </div>

              <div>
                <div className="flex items-center justify-between mb-1">
                  <label className="text-xs text-gray-500">Session ID *</label>
                  <button
                    onClick={handleQuickImportJson}
                    className="text-xs text-blue-600 hover:underline"
                  >
                    导入 JSON
                  </button>
                </div>
                <textarea
                  value={quickRecordSessionId}
                  onChange={(event) => setQuickRecordSessionId(event.target.value)}
                  rows={3}
                  className="w-full border border-gray-300 rounded px-2 py-1.5 text-xs font-mono resize-none"
                  placeholder="支持直接粘贴长串 Session ID；也可以从本机 Trae CN 日志中读取最近会话。"
                />
                <div className="flex items-center justify-between mt-1">
                  <div className="text-[11px] text-gray-400">日志来源：{sessionPathHint}</div>
                  <button
                    onClick={handleQuickFetchSessions}
                    disabled={quickRecordSessionsLoading}
                    className="text-xs text-blue-600 hover:underline disabled:opacity-40"
                  >
                    {quickRecordSessionsLoading ? '读取中...' : '读取最近会话'}
                  </button>
                </div>
                {quickRecordShowSessionPicker && (
                  <div className="mt-2 border border-gray-200 rounded-lg overflow-hidden bg-white shadow-sm">
                    {quickRecordSessionsError && <div className="px-3 py-2 text-xs text-red-600 break-all">{quickRecordSessionsError}</div>}
                    {!quickRecordSessionsError && quickRecordSessions.length === 0 && !quickRecordSessionsLoading && (
                      <div className="px-3 py-2 text-xs text-gray-400">没有读取到会话，必要时可以手动粘贴完整长串。</div>
                    )}
                    {quickRecordSessions.map((item) => (
                      <button
                        key={`${item.session_id}-${item.last_seen}`}
                        onClick={() => {
                          setQuickRecordSessionId(item.session_id);
                          setQuickRecordShowSessionPicker(false);
                        }}
                        className="w-full px-3 py-2 text-left hover:bg-blue-50 border-b border-gray-50 last:border-0"
                      >
                        <div className="font-mono text-xs text-blue-700 break-all">{item.session_id}</div>
                        <div className="text-[11px] text-gray-400 mt-1">{item.last_seen}</div>
                      </button>
                    ))}
                  </div>
                )}
              </div>

              <div className="border border-gray-200 rounded-lg overflow-hidden">
                <div className="bg-gray-50 px-3 py-2 border-b border-gray-200 flex items-center gap-3">
                  <span className="text-xs font-medium text-gray-500">对话过程记录</span>
                  {quickRecordAnalyzing && <span className="text-xs text-blue-600">分析中...</span>}
                  {!quickRecordAnalyzing && quickRecordAnalysisSummary && <span className="text-xs text-green-600">已自动分析</span>}
                  <div className="ml-auto flex gap-1">
                    <button
                      onClick={() => setQuickRecordConversationMode('text')}
                      className={`px-2 py-0.5 rounded text-xs ${quickRecordConversationMode === 'text' ? 'bg-blue-600 text-white' : 'text-gray-500 hover:bg-gray-100'}`}
                    >
                      粘贴文本
                    </button>
                    <button
                      onClick={() => setQuickRecordConversationMode('file')}
                      className={`px-2 py-0.5 rounded text-xs ${quickRecordConversationMode === 'file' ? 'bg-blue-600 text-white' : 'text-gray-500 hover:bg-gray-100'}`}
                    >
                      关联文件
                    </button>
                  </div>
                </div>

                {quickRecordConversationMode === 'text' ? (
                  <div className="p-3">
                    <textarea
                      value={quickRecordConversationText}
                      onChange={(event) => setQuickRecordConversationText(event.target.value)}
                      rows={8}
                      className="w-full border border-gray-200 rounded px-2 py-1.5 text-xs font-mono leading-relaxed resize-y"
                      placeholder="粘贴完整对话后，可点击“重新分析”。"
                    />
                    <div className="flex items-center justify-between mt-1">
                      <span className="text-xs text-gray-400">{quickRecordConversationText.length.toLocaleString()} 字符</span>
                      <button
                        onClick={() => runQuickRecordAnalysis(quickRecordConversationText)}
                        disabled={!quickRecordConversationText.trim() || quickRecordAnalyzing}
                        className="text-xs text-blue-600 hover:underline disabled:opacity-40"
                      >
                        重新分析
                      </button>
                    </div>
                  </div>
                ) : (
                  <div className="p-3">
                    <div className="flex gap-2">
                      <input
                        value={quickRecordConversationFile}
                        onChange={(event) => setQuickRecordConversationFile(event.target.value)}
                        className="flex-1 border border-gray-200 rounded px-2 py-1.5 text-xs font-mono"
                        placeholder="选择 .txt / .md 文件后自动读取分析"
                      />
                      <button
                        onClick={handleQuickPickFile}
                        disabled={quickRecordFileLoading || quickRecordAnalyzing}
                        className="px-3 py-1.5 text-xs border border-gray-300 rounded hover:bg-gray-50 disabled:opacity-40"
                      >
                        {quickRecordFileLoading ? '读取中...' : '选择文件'}
                      </button>
                    </div>
                    {quickRecordConversationFile && (
                      <button
                        onClick={async () => {
                          const content = await readTextFile(quickRecordConversationFile);
                          await runQuickRecordAnalysis(content);
                        }}
                        className="mt-2 text-xs text-blue-600 hover:underline"
                      >
                        用当前文件重新分析
                      </button>
                    )}
                  </div>
                )}

                {quickRecordAnalyzeError && (
                  <div className="px-3 py-2 bg-red-50 border-t border-red-100 text-xs text-red-600">{quickRecordAnalyzeError}</div>
                )}
                {quickRecordAnalysisSummary && !quickRecordAnalyzeError && (
                  <div className="px-3 py-2 bg-blue-50 border-t border-blue-100 text-xs text-blue-700">
                    AI 总结：{quickRecordAnalysisSummary}
                  </div>
                )}
              </div>

              <ScoreForm scores={quickRecordScores} onChange={setQuickRecordScores} />

              {(quickRecordQuestion.requires_long_task_eval || quickRecordQuestion.requires_frontend_eval) && (
                <div className="border border-gray-200 rounded-lg p-4 space-y-3">
                  <div className="text-xs font-medium text-gray-500">扩展维度</div>
                  <div className="grid md:grid-cols-3 gap-3">
                    {quickRecordQuestion.requires_long_task_eval && (
                      <ExtraScoreSelector label="长程任务" value={quickRecordLongTask} onChange={setQuickRecordLongTask} />
                    )}
                    {quickRecordQuestion.requires_frontend_eval && (
                      <ExtraScoreSelector label="前端 3D 产物" value={quickRecordFrontend3d} onChange={setQuickRecordFrontend3d} />
                    )}
                    {quickRecordQuestion.requires_frontend_eval && (
                      <ExtraScoreSelector label="前端产物美观度" value={quickRecordFrontendAesthetic} onChange={setQuickRecordFrontendAesthetic} />
                    )}
                  </div>
                </div>
              )}

              {quickRecordScores.ux < 5 ? (
                <div className="space-y-3">
                  <div>
                    <label className="text-xs text-gray-500 mb-1 block">问题类型</label>
                    <div className="flex flex-wrap gap-2">
                      {ISSUE_TYPES.map((item) => (
                        <label key={item} className="flex items-center gap-1 text-sm cursor-pointer">
                          <input
                            type="checkbox"
                            checked={quickRecordIssueTypes.includes(item)}
                            onChange={(event) => setQuickRecordIssueTypes(event.target.checked
                              ? [...quickRecordIssueTypes, item]
                              : quickRecordIssueTypes.filter((value) => value !== item))}
                          />
                          {item}
                        </label>
                      ))}
                    </div>
                  </div>
                  <div>
                    <label className="text-xs text-gray-500 mb-1 block">问题描述</label>
                    <textarea
                      value={quickRecordIssueDesc}
                      onChange={(event) => setQuickRecordIssueDesc(event.target.value)}
                      rows={4}
                      className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                    />
                  </div>
                  <div>
                    <label className="text-xs text-gray-500 mb-1 block">修复成本</label>
                    <div className="flex gap-3">
                      {(['low', 'medium', 'high'] as FixCost[]).map((item) => (
                        <label key={item} className="flex items-center gap-1 text-sm cursor-pointer">
                          <input type="radio" checked={quickRecordFixCost === item} onChange={() => setQuickRecordFixCost(item)} />
                          {item}
                        </label>
                      ))}
                    </div>
                  </div>
                </div>
              ) : (
                <div>
                  <label className="text-xs text-gray-500 mb-1 block">模型优点</label>
                  <textarea
                    value={quickRecordPros}
                    onChange={(event) => setQuickRecordPros(event.target.value)}
                    rows={4}
                    className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                  />
                </div>
              )}

              <div className="flex justify-end gap-2">
                <button
                  onClick={resetQuickRecord}
                  className="px-4 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50"
                >
                  取消
                </button>
                <button
                  onClick={saveQuickRecord}
                  className="px-4 py-1.5 text-sm bg-blue-600 text-white rounded hover:bg-blue-700"
                >
                  保存
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {quickGsbQuestion && (
        <div className="fixed inset-0 bg-black/40 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-xl shadow-xl w-full max-w-2xl max-h-[90vh] overflow-y-auto p-6">
            <div className="flex items-center gap-3 mb-5">
              <h2 className="font-semibold text-base flex-1">新增 GSB 对比</h2>
              <div className="text-xs text-gray-400">{quickGsbQuestion.title}</div>
            </div>

            <div className="space-y-4">
              <div className="grid md:grid-cols-2 gap-3">
                <div>
                  <label className="text-xs text-gray-500 mb-1 block">模型 A</label>
                  <select
                    value={quickGsbModelA}
                    onChange={(event) => setQuickGsbModelA(event.target.value)}
                    className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                  >
                    {quickGsbQuestion.branches.map((branch) => (
                      <option key={branch} value={branch}>{getModelLabel(branch)}</option>
                    ))}
                  </select>
                </div>
                <div>
                  <label className="text-xs text-gray-500 mb-1 block">模型 B</label>
                  <select
                    value={quickGsbModelB}
                    onChange={(event) => setQuickGsbModelB(event.target.value)}
                    className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                  >
                    {quickGsbQuestion.branches.map((branch) => (
                      <option key={branch} value={branch}>{getModelLabel(branch)}</option>
                    ))}
                  </select>
                </div>
              </div>

              {(() => {
                const recA = data.records.find((r) => r.question_id === quickGsbQuestion.id && r.model === quickGsbModelA);
                const recB = data.records.find((r) => r.question_id === quickGsbQuestion.id && r.model === quickGsbModelB);
                const canAnalyze = !!recA && !!recB && quickGsbModelA !== quickGsbModelB;
                return (
                  <div className="border border-gray-200 rounded-lg overflow-hidden">
                    <div className="bg-gray-50 px-3 py-2 border-b border-gray-200 flex items-center gap-3">
                      <span className="text-xs font-medium text-gray-500">AI 自动分析</span>
                      {quickGsbAnalyzing && <span className="text-xs text-blue-600">分析中...</span>}
                      {!quickGsbAnalyzing && (quickGsbGood || quickGsbBad || quickGsbNote) && !quickGsbAnalyzeError && <span className="text-xs text-green-600">已生成建议</span>}
                      <button
                        onClick={handleQuickGsbAnalyze}
                        disabled={!canAnalyze || quickGsbAnalyzing}
                        className="ml-auto px-3 py-1 text-xs bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-40"
                      >
                        {quickGsbAnalyzing ? '分析中...' : '分析两条记录'}
                      </button>
                    </div>
                    {!canAnalyze && (
                      <div className="px-3 py-2 text-xs text-gray-400">
                        只有两个模型都已有测试记录时，才能自动生成 GSB 建议。
                      </div>
                    )}
                    {quickGsbAnalyzeError && <div className="px-3 py-2 text-xs text-red-600">{quickGsbAnalyzeError}</div>}
                  </div>
                );
              })()}

              <div>
                <label className="text-xs text-gray-500 mb-1 block">GSB 结果</label>
                <div className="flex gap-3 flex-wrap">
                  {([
                    ['A', `A 更好 (${getModelLabel(quickGsbModelA)})`],
                    ['same', 'same'],
                    ['B', `B 更好 (${getModelLabel(quickGsbModelB)})`],
                  ] as const).map(([value, label]) => (
                    <label key={value} className="flex items-center gap-1.5 text-sm cursor-pointer">
                      <input type="radio" checked={quickGsbWinner === value} onChange={() => setQuickGsbWinner(value)} />
                      {label}
                    </label>
                  ))}
                </div>
              </div>

              <div>
                <label className="text-xs text-gray-500 mb-1 block">好的模型好在哪</label>
                <textarea
                  value={quickGsbGood}
                  onChange={(event) => setQuickGsbGood(event.target.value)}
                  rows={3}
                  className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                />
              </div>
              <div>
                <label className="text-xs text-gray-500 mb-1 block">坏的模型坏在哪</label>
                <textarea
                  value={quickGsbBad}
                  onChange={(event) => setQuickGsbBad(event.target.value)}
                  rows={3}
                  className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                />
              </div>
              <div>
                <label className="text-xs text-gray-500 mb-1 block">其他备注</label>
                <textarea
                  value={quickGsbNote}
                  onChange={(event) => setQuickGsbNote(event.target.value)}
                  rows={2}
                  className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                />
              </div>

              <div className="flex justify-end gap-2">
                <button
                  onClick={resetQuickGsb}
                  className="px-4 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50"
                >
                  取消
                </button>
                <button
                  onClick={saveQuickGsb}
                  className="px-4 py-1.5 text-sm bg-blue-600 text-white rounded hover:bg-blue-700"
                >
                  保存
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {editing && (
        <div className="fixed inset-0 bg-black/40 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-xl shadow-xl w-full max-w-3xl max-h-[90vh] overflow-y-auto p-6">
            <div className="flex items-center gap-3 mb-5">
              <h2 className="font-semibold text-base flex-1">{editing.id ? '编辑题目' : '新建题目'}</h2>
              <div className="flex gap-1 text-xs">
                <button
                  onClick={() => setWizardStep('basic')}
                  className={`px-3 py-1 rounded font-medium ${wizardStep === 'basic' ? 'bg-blue-600 text-white' : 'text-gray-500 hover:bg-gray-100'}`}
                >
                  1. 基本信息
                </button>
                <button
                  onClick={() => setWizardStep('github')}
                  className={`px-3 py-1 rounded font-medium ${wizardStep === 'github' ? 'bg-blue-600 text-white' : 'text-gray-500 hover:bg-gray-100'}`}
                >
                  2. GitHub
                </button>
              </div>
            </div>

            {wizardStep === 'basic' && (
              <div className="space-y-4">
                <div className="flex gap-3">
                  <div className="flex-1">
                    <label className="text-xs text-gray-500 mb-1 block">题目标题 *</label>
                    <input
                      value={form.title}
                      onChange={(event) => setForm({ ...form, title: event.target.value })}
                      className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                    />
                  </div>
                  <div>
                    <label className="text-xs text-gray-500 mb-1 block">分类</label>
                    <select
                      value={form.category}
                      onChange={(event) => setForm({ ...form, category: event.target.value })}
                      className="border border-gray-300 rounded px-2 py-1.5 text-sm"
                    >
                      {CATEGORY_OPTIONS.map((category) => (
                        <option key={category} value={category}>{category}</option>
                      ))}
                    </select>
                  </div>
                  <div>
                    <label className="text-xs text-gray-500 mb-1 block">难度</label>
                    <select
                      value={form.difficulty}
                      onChange={(event) => setForm({ ...form, difficulty: Number(event.target.value) as 1 | 2 | 3 })}
                      className="border border-gray-300 rounded px-2 py-1.5 text-sm"
                    >
                      {DIFFICULTY_OPTIONS.map((item) => (
                        <option key={item.value} value={item.value}>{item.label}</option>
                      ))}
                    </select>
                  </div>
                </div>

                <div className="flex gap-3">
                  <div className="flex-1">
                    <label className="text-xs text-gray-500 mb-1 block">技术栈</label>
                    <input
                      value={form.tech_stack}
                      onChange={(event) => setForm({ ...form, tech_stack: event.target.value })}
                      className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                    />
                  </div>
                  <div className="flex-1">
                    <label className="text-xs text-gray-500 mb-1 block">任务方向</label>
                    <input
                      value={form.task_direction}
                      onChange={(event) => setForm({ ...form, task_direction: event.target.value })}
                      className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                    />
                  </div>
                  <div className="flex-1">
                    <label className="text-xs text-gray-500 mb-1 block">题目方向</label>
                    <input
                      value={form.question_direction}
                      onChange={(event) => setForm({ ...form, question_direction: event.target.value })}
                      className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                    />
                  </div>
                </div>

                <div className="grid md:grid-cols-3 gap-3">
                  <label className="flex items-center gap-2 text-sm text-gray-600">
                    <input
                      type="checkbox"
                      checked={form.is_current_focus}
                      onChange={(event) => setForm({ ...form, is_current_focus: event.target.checked })}
                    />
                    本期重点
                  </label>
                  <label className="flex items-center gap-2 text-sm text-gray-600">
                    <input
                      type="checkbox"
                      checked={form.requires_long_task_eval}
                      onChange={(event) => setForm({ ...form, requires_long_task_eval: event.target.checked })}
                    />
                    需要长程任务评分
                  </label>
                  <label className="flex items-center gap-2 text-sm text-gray-600">
                    <input
                      type="checkbox"
                      checked={form.requires_frontend_eval}
                      onChange={(event) => setForm({ ...form, requires_frontend_eval: event.target.checked })}
                    />
                    需要前端评分
                  </label>
                </div>

                <div>
                  <label className="text-xs text-gray-500 mb-1 block">Prompt *</label>
                  <textarea
                    value={form.prompt}
                    onChange={(event) => setForm({ ...form, prompt: event.target.value })}
                    rows={10}
                    className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm font-mono"
                    placeholder="这里直接写纯文本 Prompt，避免 Markdown 标题、表格、链接、粗体等格式；普通段落、数字步骤、内联代码名可以直接写。"
                  />
                </div>

                <div className="rounded-lg border border-amber-200 bg-amber-50 px-3 py-2 text-xs text-amber-700">
                  题目保存前必须选定 2-3 个模型。后续测试记录和 GSB 都只允许在这些模型分支范围内创建。
                </div>

                <div className="flex justify-between pt-2">
                  <button
                    onClick={resetEditor}
                    className="px-4 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50"
                  >
                    取消
                  </button>
                  <div className="flex gap-2">
                    <button
                      onClick={save}
                      className="px-4 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50"
                    >
                      仅保存
                    </button>
                    <button
                      onClick={() => setWizardStep('github')}
                      className="px-4 py-1.5 text-sm bg-blue-600 text-white rounded hover:bg-blue-700"
                    >
                      下一步：GitHub
                    </button>
                  </div>
                </div>
              </div>
            )}

            {wizardStep === 'github' && (
              <div className="space-y-4">
                <div className="border border-gray-200 rounded-lg p-4">
                  <div className="flex items-center gap-2 mb-3">
                    <span className={`w-5 h-5 rounded-full flex items-center justify-center text-xs font-bold ${repoCreated ? 'bg-green-500 text-white' : 'bg-gray-200 text-gray-600'}`}>
                      {repoCreated ? '✓' : '1'}
                    </span>
                    <span className="text-sm font-medium">自动创建 GitHub 仓库</span>
                  </div>
                  <div className="mb-3">
                    <label className="text-xs text-gray-500 mb-1 block">Repo 介绍</label>
                    <input
                      value={form.repo_desc}
                      onChange={(event) => setForm({ ...form, repo_desc: event.target.value })}
                      placeholder="这里的描述会用于创建 GitHub 仓库"
                      className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                    />
                  </div>
                  <div className="flex gap-2">
                    <input
                      value={form.github_repo_name}
                      onChange={(event) => setForm({ ...form, github_repo_name: event.target.value })}
                      placeholder="仓库名，例如 dogfooding-weather-task"
                      disabled={repoCreated}
                      className="flex-1 border border-gray-300 rounded px-2 py-1.5 text-sm font-mono disabled:bg-gray-50"
                    />
                    <label className="flex items-center gap-1 text-xs text-gray-500 shrink-0">
                      <input type="checkbox" checked={isPrivate} onChange={(event) => setIsPrivate(event.target.checked)} />
                      私有仓库
                    </label>
                    <button
                      onClick={handleCreateRepo}
                      disabled={repoCreating || repoCreated || !form.github_repo_name.trim()}
                      className="px-3 py-1.5 text-xs bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-40"
                    >
                      {repoCreating ? '创建中...' : repoCreated ? '已创建' : '创建仓库'}
                    </button>
                  </div>
                  {form.repo_url && (
                    <div className="mt-2 text-xs text-gray-500 space-y-1">
                      <div>Repo 链接由这一步自动创建回填，不需要在基本信息页手填。</div>
                      <a href={form.repo_url} target="_blank" rel="noreferrer" className="block text-blue-600 hover:underline">{form.repo_url}</a>
                      {form.github_ssh_url && <div className="font-mono break-all">{form.github_ssh_url}</div>}
                    </div>
                  )}
                </div>

                <div className={`border rounded-lg p-4 ${!repoCreated ? 'opacity-70 border-gray-200' : 'border-gray-200'}`}>
                  <div className="flex items-center gap-2 mb-3">
                    <span className={`w-5 h-5 rounded-full flex items-center justify-center text-xs font-bold ${branchesCreated.length > 0 ? 'bg-green-500 text-white' : 'bg-gray-200 text-gray-600'}`}>
                      {branchesCreated.length > 0 ? '✓' : '2'}
                    </span>
                    <span className="text-sm font-medium">选择 2-3 个模型并创建分支</span>
                  </div>

                  <div className="flex flex-wrap gap-2 mb-3">
                    {modelConfigs.map((model) => {
                      const selected = selectedModels.includes(model.value);
                      const disabled = !selected && selectedModels.length >= 3;
                      return (
                        <button
                          key={model.value}
                          onClick={() => toggleModel(model.value)}
                          disabled={disabled}
                          className={`px-3 py-1.5 rounded border text-xs transition-colors ${
                            selected
                              ? 'border-blue-400 bg-blue-50 text-blue-700'
                              : 'border-gray-200 text-gray-600 hover:bg-gray-50'
                          } ${disabled ? 'opacity-50 cursor-not-allowed' : ''}`}
                        >
                          {model.label}
                        </button>
                      );
                    })}
                  </div>

                  <div className="text-xs text-gray-500 mb-3">
                    已选 {selectedModels.length} 个模型。题目要求固定为 2-3 个模型，后续测试记录和 GSB 都会围绕这些分支展开。
                  </div>

                  <div className="border border-dashed border-gray-200 rounded-lg p-3 mb-3">
                    <div className="flex items-center gap-2">
                      <button
                        onClick={() => setDictOpen((current) => !current)}
                        className="text-xs font-medium text-gray-700 hover:text-blue-600"
                      >
                        {dictOpen ? '收起' : '展开'} 模型字典维护
                      </button>
                      <button
                        onClick={addModelConfig}
                        className="ml-auto text-xs text-blue-600 hover:underline"
                      >
                        + 增加模型
                      </button>
                    </div>

                    {dictOpen && (
                      <div className="mt-3 space-y-2">
                        {modelConfigs.map((item, index) => (
                          <div key={`${item.value}-${index}`} className="grid grid-cols-[1fr,1fr,auto] gap-2">
                            <input
                              value={item.value}
                              onChange={(event) => updateModelConfig(index, { value: event.target.value })}
                              placeholder="value，用于分支名"
                              className="border border-gray-300 rounded px-2 py-1.5 text-xs font-mono"
                            />
                            <input
                              value={item.label}
                              onChange={(event) => updateModelConfig(index, { label: event.target.value })}
                              placeholder="label，用于界面显示"
                              className="border border-gray-300 rounded px-2 py-1.5 text-xs"
                            />
                            <button
                              onClick={() => removeModelConfig(index)}
                              className="px-2 py-1.5 text-xs text-red-600 hover:underline"
                            >
                              删除
                            </button>
                          </div>
                        ))}
                        {dictError && <div className="text-xs text-red-600">{dictError}</div>}
                        <div className="flex justify-end">
                          <button
                            onClick={saveModelDictionary}
                            disabled={dictSaving}
                            className="px-3 py-1.5 text-xs bg-slate-900 text-white rounded hover:bg-slate-800 disabled:opacity-40"
                          >
                            {dictSaving ? '保存中...' : '保存模型字典'}
                          </button>
                        </div>
                      </div>
                    )}
                  </div>

                  <button
                    onClick={handleCreateBranches}
                    disabled={!repoCreated || selectedModels.length < 2 || branchCreating}
                    className="px-3 py-1.5 text-xs bg-purple-600 text-white rounded hover:bg-purple-700 disabled:opacity-40"
                  >
                    {branchCreating ? '创建中...' : branchesCreated.length > 0 ? '重新创建分支' : '创建模型分支'}
                  </button>

                  {branchesCreated.length > 0 && (
                    <div className="flex flex-wrap gap-1 mt-3">
                      {branchesCreated.map((branch) => (
                        <span key={branch} className="text-xs bg-green-50 text-green-700 px-2 py-0.5 rounded font-mono">
                          ✓ {branch}
                        </span>
                      ))}
                    </div>
                  )}
                </div>

                {githubError && (
                  <div className="bg-red-50 border border-red-200 rounded p-3 text-xs text-red-600">{githubError}</div>
                )}

                <div className="flex justify-between pt-2">
                  <button
                    onClick={() => setWizardStep('basic')}
                    className="px-4 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50"
                  >
                    返回
                  </button>
                  <div className="flex gap-2">
                    <button
                      onClick={resetEditor}
                      className="px-4 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50"
                    >
                      取消
                    </button>
                    <button
                      onClick={save}
                      className="px-4 py-1.5 text-sm bg-blue-600 text-white rounded hover:bg-blue-700"
                    >
                      保存题目
                    </button>
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
