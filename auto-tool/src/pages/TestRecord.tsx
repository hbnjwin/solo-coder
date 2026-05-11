import { useEffect, useMemo, useState } from 'react';
import ScoreForm from '../components/ScoreForm';
import {
  AppData,
  EXTENDED_SCORE_LABELS,
  FixCost,
  ISSUE_TYPES,
  MODELS,
  Scores,
  TestRecord as TestRecordType,
  getModelLabel,
  getQuestionModelOptions,
} from '../types';
import {
  SessionInfo,
  analyzeConversation,
  generateId,
  getRecentSessions,
  githubCreatePr,
  nowIso,
  openFileDialog,
  readTextFile,
} from '../store';

interface Props {
  data: AppData;
  onUpdate: (data: AppData) => void;
}

const defaultScores = (): Scores => ({
  ux: 3,
  planning: 3,
  reasoning: 3,
  instruction: 3,
  engineering: 3,
});

const sessionPathHint = 'APPDATA\\Trae CN\\logs\\<日期>\\window1\\renderer.log 和 Modular\\ai-agent_*_stdout.log';

function totalScore(record: TestRecordType) {
  return Object.values(record.scores).reduce((sum, value) => sum + value, 0);
}

function buildPrTitle(questionTitle: string, category: string, model: string) {
  return `[${category}] ${questionTitle} - ${getModelLabel(model)}`;
}

function buildPrBody(questionTitle: string, model: string, sessionId?: string) {
  return [
    `题目：${questionTitle}`,
    `模型：${getModelLabel(model)} (${model})`,
    `Session ID：${sessionId?.trim() || ''}`,
  ].join('\n');
}

function scoreButtonClass(value: number) {
  if (value >= 4) return 'bg-green-100 text-green-700';
  if (value === 3) return 'bg-yellow-100 text-yellow-700';
  return 'bg-red-100 text-red-700';
}

function extraScoreSelector(
  label: string,
  value: number | null,
  onChange: (next: number | null) => void
) {
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

export default function TestRecord({ data, onUpdate }: Props) {
  const [view, setView] = useState<'list' | 'new'>('list');
  const [filterModel, setFilterModel] = useState('all');
  const [filterQuestion, setFilterQuestion] = useState('all');
  const [expandedId, setExpandedId] = useState<string | null>(null);
  const [loadedConversation, setLoadedConversation] = useState<Record<string, string>>({});
  const [reanalyzingId, setReanalyzingId] = useState<string | null>(null);

  const [questionId, setQuestionId] = useState('');
  const selectedQuestion = data.questions.find((item) => item.id === questionId) ?? null;
  const availableModels = getQuestionModelOptions(selectedQuestion);

  const [model, setModel] = useState(MODELS[0]?.value ?? '');
  const [sessionId, setSessionId] = useState('');
  const [prUrl, setPrUrl] = useState('');
  const [interactionRounds, setInteractionRounds] = useState(1);
  const [scores, setScores] = useState<Scores>(defaultScores());
  const [longTaskScore, setLongTaskScore] = useState<number | null>(null);
  const [frontend3dScore, setFrontend3dScore] = useState<number | null>(null);
  const [frontendAestheticScore, setFrontendAestheticScore] = useState<number | null>(null);
  const [issueTypes, setIssueTypes] = useState<string[]>([]);
  const [issueDesc, setIssueDesc] = useState('');
  const [fixCost, setFixCost] = useState<FixCost | null>(null);
  const [pros, setPros] = useState('');
  const [conversationText, setConversationText] = useState('');
  const [conversationFile, setConversationFile] = useState('');
  const [analysisSummary, setAnalysisSummary] = useState('');
  const [conversationMode, setConversationMode] = useState<'text' | 'file'>('text');
  const [fileLoading, setFileLoading] = useState(false);
  const [analyzing, setAnalyzing] = useState(false);
  const [analyzeError, setAnalyzeError] = useState('');
  const [sessions, setSessions] = useState<SessionInfo[]>([]);
  const [sessionsLoading, setSessionsLoading] = useState(false);
  const [sessionsError, setSessionsError] = useState('');
  const [showSessionPicker, setShowSessionPicker] = useState(false);
  const [prCreating, setPrCreating] = useState(false);
  const [prError, setPrError] = useState('');

  const uxSatisfied = scores.ux === 5;

  useEffect(() => {
    if (availableModels.length === 0) {
      return;
    }
    if (!availableModels.some((item) => item.value === model)) {
      setModel(availableModels[0].value);
    }
  }, [availableModels, model]);

  function resetForm() {
    setQuestionId('');
    setModel(MODELS[0]?.value ?? '');
    setSessionId('');
    setPrUrl('');
    setInteractionRounds(1);
    setScores(defaultScores());
    setLongTaskScore(null);
    setFrontend3dScore(null);
    setFrontendAestheticScore(null);
    setIssueTypes([]);
    setIssueDesc('');
    setFixCost(null);
    setPros('');
    setConversationText('');
    setConversationFile('');
    setAnalysisSummary('');
    setConversationMode('text');
    setAnalyzeError('');
    setSessions([]);
    setSessionsError('');
    setShowSessionPicker(false);
    setPrError('');
  }

  async function handleImportJsonToForm() {
    try {
      const path = await openFileDialog();
      if (!path) return;
      const text = await readTextFile(path);
      const parsed = JSON.parse(text);
      if (parsed.question_id) setQuestionId(parsed.question_id);
      if (parsed.model) setModel(parsed.model);
      if (parsed.session_id) setSessionId(parsed.session_id);
      if (parsed.pr_url) setPrUrl(parsed.pr_url);
      if (parsed.interaction_rounds) setInteractionRounds(parsed.interaction_rounds);
      if (parsed.scores) setScores({ ...defaultScores(), ...parsed.scores });
      if (parsed.long_task_score != null) setLongTaskScore(parsed.long_task_score);
      if (parsed.frontend_3d_score != null) setFrontend3dScore(parsed.frontend_3d_score);
      if (parsed.frontend_aesthetic_score != null) setFrontendAestheticScore(parsed.frontend_aesthetic_score);
      if (parsed.issue_types) setIssueTypes(parsed.issue_types);
      if (parsed.issue_desc) setIssueDesc(parsed.issue_desc);
      if (parsed.fix_cost) setFixCost(parsed.fix_cost);
      if (parsed.pros) setPros(parsed.pros);
      if (parsed.analysis_summary) setAnalysisSummary(parsed.analysis_summary);
      if (parsed.conversation_text) {
        setConversationText(parsed.conversation_text);
        setConversationMode('text');
      }
      if (parsed.conversation_file) {
        setConversationFile(parsed.conversation_file);
        setConversationMode('file');
      }
    } catch (error) {
      alert(`导入失败：${String(error)}`);
    }
  }

  async function handleFetchSessions() {
    setSessionsLoading(true);
    setShowSessionPicker(true);
    setSessionsError('');
    const result = await getRecentSessions();
    if (result.ok) {
      setSessions(result.data);
    } else {
      setSessions([]);
      setSessionsError(result.error);
    }
    setSessionsLoading(false);
  }

  async function handleCreatePr() {
    if (!selectedQuestion?.github_owner || !selectedQuestion.github_repo_name) {
      return '';
    }
    setPrCreating(true);
    setPrError('');
    const result = await githubCreatePr(
      selectedQuestion.github_owner,
      selectedQuestion.github_repo_name,
      model,
      buildPrTitle(selectedQuestion.title, selectedQuestion.category, model),
      buildPrBody(selectedQuestion.title, model, sessionId),
      selectedQuestion.default_branch || 'main'
    );
    if (result.ok) {
      setPrUrl(result.url);
      setPrCreating(false);
      return result.url;
    } else {
      setPrError(result.error);
    }
    setPrCreating(false);
    return '';
  }

  async function runAnalysis(conversation: string) {
    if (!selectedQuestion || !conversation.trim()) {
      return;
    }
    setAnalyzing(true);
    setAnalyzeError('');
    try {
      const result = await analyzeConversation(selectedQuestion.prompt, conversation);
      if (result.rounds && result.rounds > 0) {
        setInteractionRounds(result.rounds);
      }
      setScores({
        ux: result.ux,
        planning: result.planning,
        reasoning: result.reasoning,
        instruction: result.instruction,
        engineering: result.engineering,
      });
      setIssueTypes(result.issue_types ?? []);
      setIssueDesc(result.issue_desc ?? '');
      setFixCost(result.fix_cost ?? null);
      setPros(result.pros ?? '');
      setAnalysisSummary(result.analysis ?? '');
    } catch (error) {
      setAnalyzeError(String(error));
    } finally {
      setAnalyzing(false);
    }
  }

  async function handlePickFile() {
    setFileLoading(true);
    try {
      const path = await openFileDialog();
      if (!path) {
        return;
      }
      setConversationFile(path);
      const text = await readTextFile(path);
      await runAnalysis(text);
    } catch (error) {
      setAnalyzeError(String(error));
    } finally {
      setFileLoading(false);
    }
  }

  async function submit() {
    if (!selectedQuestion) {
      alert('请先选择题目。');
      return;
    }
    if (!sessionId.trim()) {
      alert('请填写 Session ID。');
      return;
    }

    let resolvedPrUrl = prUrl.trim();
    if (!resolvedPrUrl && selectedQuestion.github_owner && selectedQuestion.github_repo_name) {
      const createdPrUrl = await handleCreatePr();
      if (!createdPrUrl) {
        return;
      }
      resolvedPrUrl = createdPrUrl;
    }

    const nextRecord: TestRecordType = {
      id: generateId(),
      question_id: selectedQuestion.id,
      model,
      branch_name: model,
      session_id: sessionId.trim(),
      pr_url: resolvedPrUrl,
      interaction_rounds: interactionRounds,
      scores,
      long_task_score: selectedQuestion.requires_long_task_eval ? longTaskScore : null,
      frontend_3d_score: selectedQuestion.requires_frontend_eval ? frontend3dScore : null,
      frontend_aesthetic_score: selectedQuestion.requires_frontend_eval ? frontendAestheticScore : null,
      issue_types: uxSatisfied ? [] : issueTypes,
      issue_desc: uxSatisfied ? '' : issueDesc.trim(),
      fix_cost: uxSatisfied ? null : fixCost,
      pros: uxSatisfied ? pros.trim() : '',
      analysis_summary: analysisSummary.trim(),
      conversation_text: conversationMode === 'text' ? conversationText : '',
      conversation_file: conversationMode === 'file' ? conversationFile : '',
      created_at: nowIso(),
    };

    const existing = data.records.find((item) => item.question_id === selectedQuestion.id && item.model === model);
    const nextRecords = existing
      ? data.records.map((item) => item.id === existing.id ? { ...nextRecord, id: existing.id } : item)
      : [...data.records, nextRecord];

    onUpdate({ ...data, records: nextRecords });
    resetForm();
    setView('list');
  }

  async function handleImportJson() {
    try {
      const path = await openFileDialog();
      if (!path) return;
      const text = await readTextFile(path);
      const parsed = JSON.parse(text);
      const nextRecord: TestRecordType = {
        id: generateId(),
        question_id: parsed.question_id ?? '',
        model: parsed.model ?? '',
        branch_name: parsed.branch_name ?? parsed.model ?? '',
        session_id: parsed.session_id ?? '',
        pr_url: parsed.pr_url ?? '',
        interaction_rounds: parsed.interaction_rounds ?? 1,
        scores: parsed.scores ?? { ux: 3, planning: 3, reasoning: 3, instruction: 3, engineering: 3 },
        long_task_score: parsed.long_task_score ?? null,
        frontend_3d_score: parsed.frontend_3d_score ?? null,
        frontend_aesthetic_score: parsed.frontend_aesthetic_score ?? null,
        issue_types: parsed.issue_types ?? [],
        issue_desc: parsed.issue_desc ?? '',
        fix_cost: parsed.fix_cost ?? null,
        pros: parsed.pros ?? '',
        analysis_summary: parsed.analysis_summary ?? '',
        conversation_text: parsed.conversation_text ?? '',
        conversation_file: parsed.conversation_file ?? '',
        created_at: parsed.created_at ?? nowIso(),
      };
      const existing = data.records.find(
        (item) => item.question_id === nextRecord.question_id && item.model === nextRecord.model
      );
      const nextRecords = existing
        ? data.records.map((item) => item.id === existing.id ? { ...nextRecord, id: existing.id } : item)
        : [...data.records, nextRecord];
      onUpdate({ ...data, records: nextRecords });
      alert(`已导入：${nextRecord.model} / ${nextRecord.question_id || '未知题目'}`);
    } catch (error) {
      alert(`导入失败：${String(error)}`);
    }
  }

  function remove(recordId: string) {
    if (!confirm('确认删除这条测试记录吗？')) {
      return;
    }
    onUpdate({ ...data, records: data.records.filter((item) => item.id !== recordId) });
  }

  async function loadFileContent(recordId: string, filePath: string) {
    if (loadedConversation[recordId]) {
      return;
    }
    try {
      const text = await readTextFile(filePath);
      setLoadedConversation((current) => ({ ...current, [recordId]: text }));
    } catch (error) {
      setLoadedConversation((current) => ({ ...current, [recordId]: `[无法读取文件] ${String(error)}` }));
    }
  }

  async function handleReanalyzeRecord(record: TestRecordType) {
    const question = data.questions.find((q) => q.id === record.question_id);
    if (!question) return;

    let conversation = record.conversation_text;
    if (!conversation && record.conversation_file) {
      try {
        conversation = await readTextFile(record.conversation_file);
      } catch {
        return;
      }
    }
    if (!conversation?.trim()) return;

    setReanalyzingId(record.id);
    try {
      const result = await analyzeConversation(question.prompt, conversation);
      const updated: TestRecordType = {
        ...record,
        interaction_rounds: result.rounds && result.rounds > 0 ? result.rounds : record.interaction_rounds,
        scores: {
          ux: result.ux,
          planning: result.planning,
          reasoning: result.reasoning,
          instruction: result.instruction,
          engineering: result.engineering,
        },
        issue_types: result.issue_types ?? [],
        issue_desc: result.issue_desc ?? '',
        fix_cost: result.fix_cost ?? null,
        pros: result.pros ?? '',
        analysis_summary: result.analysis ?? '',
      };
      onUpdate({ ...data, records: data.records.map((r) => r.id === record.id ? updated : r) });
    } catch (error) {
      alert(`重新分析失败：${String(error)}`);
    } finally {
      setReanalyzingId(null);
    }
  }

  const groupedRecords = useMemo(() => {
    const items = data.questions
      .map((question) => ({
        question,
        records: data.records.filter((item) => item.question_id === question.id),
      }))
      .filter((item) => {
        const matchQuestion = filterQuestion === 'all' || item.question.id === filterQuestion;
        const matchModel = filterModel === 'all' || item.records.some((record) => record.model === filterModel);
        return matchQuestion && matchModel;
      });
    return items;
  }, [data.questions, data.records, filterModel, filterQuestion]);

  if (view === 'new') {
    return (
      <div className="p-4 max-w-3xl mx-auto">
        <div className="flex items-center gap-3 mb-5">
          <button onClick={() => { resetForm(); setView('list'); }} className="text-sm text-gray-500 hover:text-gray-700">
            返回
          </button>
          <h2 className="font-semibold text-base">新建测试记录</h2>
        </div>

        <div className="space-y-4">
          <div className="flex gap-3">
            <div className="flex-1">
              <label className="text-xs text-gray-500 mb-1 block">题目 *</label>
              <select
                value={questionId}
                onChange={(event) => setQuestionId(event.target.value)}
                className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
              >
                <option value="">请选择题目</option>
                {data.questions.map((question) => (
                  <option key={question.id} value={question.id}>[{question.id}] {question.title}</option>
                ))}
              </select>
            </div>
            <div className="w-56">
              <label className="text-xs text-gray-500 mb-1 block">模型 *</label>
              <select
                value={model}
                onChange={(event) => setModel(event.target.value)}
                className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                disabled={!selectedQuestion}
              >
                {availableModels.map((item) => (
                  <option key={item.value} value={item.value}>{item.label}</option>
                ))}
              </select>
            </div>
          </div>

          {selectedQuestion && (
            <div className="rounded-lg border border-blue-200 bg-blue-50 px-3 py-2 text-xs text-blue-700">
              当前题目只允许在这几个模型下建记录：{selectedQuestion.branches.map(getModelLabel).join('、')}。
              同一题目 + 同一模型会自动覆盖旧记录，避免重复导出。
            </div>
          )}

          <div className="grid md:grid-cols-[1.4fr,1fr,120px] gap-3">
            <div>
              <div className="flex items-center justify-between mb-1">
                <label className="text-xs text-gray-500">Session ID *</label>
                <button
                  onClick={handleImportJsonToForm}
                  className="text-xs text-blue-600 hover:underline"
                >
                  导入 JSON
                </button>
              </div>
              <textarea
                value={sessionId}
                onChange={(event) => setSessionId(event.target.value)}
                rows={3}
                className="w-full border border-gray-300 rounded px-2 py-1.5 text-xs font-mono leading-relaxed resize-none"
                placeholder="支持直接粘贴长串 Session ID；“读取最近会话”会从本机 Trae CN 日志提取核心 session_id。"
              />
              <div className="flex items-center justify-between mt-1">
                <div className="text-[11px] text-gray-400">日志来源：{sessionPathHint}</div>
                <button
                  onClick={handleFetchSessions}
                  disabled={sessionsLoading}
                  className="text-xs text-blue-600 hover:underline disabled:opacity-40"
                >
                  {sessionsLoading ? '读取中...' : '读取最近会话'}
                </button>
              </div>
              {showSessionPicker && (
                <div className="mt-2 border border-gray-200 rounded-lg overflow-hidden bg-white shadow-sm">
                  {sessionsError && <div className="px-3 py-2 text-xs text-red-600 break-all">{sessionsError}</div>}
                  {!sessionsError && sessions.length === 0 && !sessionsLoading && (
                    <div className="px-3 py-2 text-xs text-gray-400">没有读取到会话，必要时可以手动粘贴完整长串。</div>
                  )}
                  {sessions.map((item) => (
                    <button
                      key={`${item.session_id}-${item.last_seen}`}
                      onClick={() => {
                        setSessionId(item.session_id);
                        setShowSessionPicker(false);
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

              <div>
                <label className="text-xs text-gray-500 mb-1 block">PR 链接</label>
                <div className="flex gap-2">
                <input
                  value={prUrl}
                  onChange={(event) => setPrUrl(event.target.value)}
                  className="flex-1 border border-gray-300 rounded px-2 py-1.5 text-sm"
                  placeholder="https://github.com/..."
                />
                {selectedQuestion?.github_owner && selectedQuestion.branches.includes(model) && (
                  <button
                    onClick={handleCreatePr}
                    disabled={prCreating}
                    className="px-2 py-1.5 text-xs bg-purple-600 text-white rounded hover:bg-purple-700 disabled:opacity-40"
                  >
                    {prCreating ? '创建中...' : '自动 PR'}
                  </button>
                )}
                </div>
              <div className="text-[11px] text-gray-400 mt-1">可留空，点击“自动 PR”或保存记录时自动创建并回填。</div>
              {prError && <div className="text-xs text-red-600 mt-1">{prError}</div>}
            </div>

            <div>
              <label className="text-xs text-gray-500 mb-1 block">交互轮次</label>
              <input
                type="number"
                min={1}
                max={20}
                value={interactionRounds}
                onChange={(event) => setInteractionRounds(Math.max(1, Number(event.target.value) || 1))}
                className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm text-center"
              />
            </div>
          </div>

          <div className="border border-gray-200 rounded-lg overflow-hidden">
            <div className="bg-gray-50 px-3 py-2 border-b border-gray-200 flex items-center gap-3">
              <span className="text-xs font-medium text-gray-500">对话过程记录</span>
              {analyzing && <span className="text-xs text-blue-600">分析中...</span>}
              {!analyzing && analysisSummary && <span className="text-xs text-green-600">已自动分析</span>}
              <div className="ml-auto flex gap-1">
                <button
                  onClick={() => setConversationMode('text')}
                  className={`px-2 py-0.5 rounded text-xs ${conversationMode === 'text' ? 'bg-blue-600 text-white' : 'text-gray-500 hover:bg-gray-100'}`}
                >
                  粘贴文本
                </button>
                <button
                  onClick={() => setConversationMode('file')}
                  className={`px-2 py-0.5 rounded text-xs ${conversationMode === 'file' ? 'bg-blue-600 text-white' : 'text-gray-500 hover:bg-gray-100'}`}
                >
                  关联文件
                </button>
              </div>
            </div>

            {conversationMode === 'text' ? (
              <div className="p-3">
                <textarea
                  value={conversationText}
                  onChange={(event) => setConversationText(event.target.value)}
                  rows={9}
                  className="w-full border border-gray-200 rounded px-2 py-1.5 text-xs font-mono leading-relaxed resize-y"
                  placeholder="粘贴完整对话后，可手动点击“重新分析”。"
                />
                <div className="flex items-center justify-between mt-1">
                  <span className="text-xs text-gray-400">{conversationText.length.toLocaleString()} 字符</span>
                  <button
                    onClick={() => runAnalysis(conversationText)}
                    disabled={!conversationText.trim() || analyzing}
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
                    value={conversationFile}
                    onChange={(event) => setConversationFile(event.target.value)}
                    className="flex-1 border border-gray-200 rounded px-2 py-1.5 text-xs font-mono"
                    placeholder="选择 .txt / .md 文件后自动读取分析"
                  />
                  <button
                    onClick={handlePickFile}
                    disabled={fileLoading || analyzing}
                    className="px-3 py-1.5 text-xs border border-gray-300 rounded hover:bg-gray-50 disabled:opacity-40"
                  >
                    {fileLoading ? '读取中...' : '选择文件'}
                  </button>
                </div>
                {conversationFile && (
                  <button
                    onClick={async () => {
                      const content = await readTextFile(conversationFile);
                      await runAnalysis(content);
                    }}
                    className="mt-2 text-xs text-blue-600 hover:underline"
                  >
                    用当前文件重新分析
                  </button>
                )}
              </div>
            )}

            {analyzeError && (
              <div className="px-3 py-2 bg-red-50 border-t border-red-100 text-xs text-red-600">{analyzeError}</div>
            )}
            {analysisSummary && !analyzeError && (
              <div className="px-3 py-2 bg-blue-50 border-t border-blue-100 text-xs text-blue-700">
                AI 总结：{analysisSummary}
              </div>
            )}
          </div>

          <ScoreForm scores={scores} onChange={setScores} />

          {(selectedQuestion?.requires_long_task_eval || selectedQuestion?.requires_frontend_eval) && (
            <div className="border border-gray-200 rounded-lg p-4 space-y-3">
              <div className="text-xs font-medium text-gray-500">扩展维度</div>
              <div className="grid md:grid-cols-3 gap-3">
                {selectedQuestion?.requires_long_task_eval && extraScoreSelector(
                  EXTENDED_SCORE_LABELS.long_task_score,
                  longTaskScore,
                  setLongTaskScore
                )}
                {selectedQuestion?.requires_frontend_eval && extraScoreSelector(
                  EXTENDED_SCORE_LABELS.frontend_3d_score,
                  frontend3dScore,
                  setFrontend3dScore
                )}
                {selectedQuestion?.requires_frontend_eval && extraScoreSelector(
                  EXTENDED_SCORE_LABELS.frontend_aesthetic_score,
                  frontendAestheticScore,
                  setFrontendAestheticScore
                )}
              </div>
            </div>
          )}

          {!uxSatisfied && (
            <div className="space-y-3 border-t border-gray-200 pt-4">
              <div>
                <label className="text-xs text-gray-500 mb-1.5 block">问题类型（多选）</label>
                <div className="flex flex-wrap gap-2">
                  {ISSUE_TYPES.map((item) => (
                    <label key={item} className="flex items-center gap-1 text-sm cursor-pointer">
                      <input
                        type="checkbox"
                        checked={issueTypes.includes(item)}
                        onChange={(event) => setIssueTypes(event.target.checked
                          ? [...issueTypes, item]
                          : issueTypes.filter((value) => value !== item))}
                      />
                      {item}
                    </label>
                  ))}
                </div>
              </div>
              <div>
                <label className="text-xs text-gray-500 mb-1 block">问题描述</label>
                <textarea
                  value={issueDesc}
                  onChange={(event) => setIssueDesc(event.target.value)}
                  rows={4}
                  className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                  placeholder="尽量覆盖所有非满分维度，并与问题类型对应。"
                />
              </div>
              <div>
                <label className="text-xs text-gray-500 mb-1.5 block">修复成本</label>
                <div className="flex gap-3">
                  {(['low', 'medium', 'high'] as FixCost[]).map((item) => (
                    <label key={item} className="flex items-center gap-1 text-sm cursor-pointer">
                      <input type="radio" checked={fixCost === item} onChange={() => setFixCost(item)} />
                      {item}
                    </label>
                  ))}
                </div>
              </div>
            </div>
          )}

          {uxSatisfied && (
            <div className="border-t border-gray-200 pt-4">
              <label className="text-xs text-gray-500 mb-1 block">模型优点</label>
              <textarea
                value={pros}
                onChange={(event) => setPros(event.target.value)}
                rows={4}
                className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
                placeholder="满分记录建议按规划&执行反馈、理解/推理能力、复杂指令遵循、工程完备度逐项写优点。"
              />
            </div>
          )}

          <div className="flex justify-end gap-2 pt-2">
            <button
              onClick={() => { resetForm(); setView('list'); }}
              className="px-4 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50"
            >
              取消
            </button>
            <button
              onClick={submit}
              className="px-4 py-1.5 text-sm bg-blue-600 text-white rounded hover:bg-blue-700"
            >
              保存记录
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="p-4 max-w-6xl mx-auto">
      <div className="flex items-center gap-3 mb-4">
        <select value={filterModel} onChange={(event) => setFilterModel(event.target.value)} className="border border-gray-300 rounded px-2 py-1 text-sm">
          <option value="all">全部模型</option>
          {MODELS.map((item) => (
            <option key={item.value} value={item.value}>{item.label}</option>
          ))}
        </select>
        <select value={filterQuestion} onChange={(event) => setFilterQuestion(event.target.value)} className="border border-gray-300 rounded px-2 py-1 text-sm">
          <option value="all">全部题目</option>
          {data.questions.map((question) => (
            <option key={question.id} value={question.id}>{question.title}</option>
          ))}
        </select>
        <span className="text-sm text-gray-400">{data.records.length} 条记录</span>
        <button onClick={handleImportJson} className="ml-auto mr-2 border border-gray-300 text-gray-600 px-3 py-1.5 rounded text-sm hover:bg-gray-50">
          导入 JSON
        </button>
        <button onClick={() => setView('new')} className="bg-blue-600 text-white px-3 py-1.5 rounded text-sm hover:bg-blue-700">
          + 新建记录
        </button>
      </div>

      {groupedRecords.length === 0 ? (
        <div className="text-center text-gray-400 py-16 text-sm">暂无测试记录</div>
      ) : (
        <div className="space-y-4">
          {groupedRecords.map(({ question, records }) => (
            <div key={question.id} className="bg-white border border-gray-200 rounded-xl overflow-hidden">
              <div className="px-4 py-3 border-b border-gray-100 bg-gray-50">
                <div className="font-medium text-sm">{question.title}</div>
                <div className="text-xs text-gray-500 mt-1">模型范围：{question.branches.map(getModelLabel).join('、')}</div>
              </div>

              <div className="divide-y divide-gray-100">
                {records
                  .filter((record) => filterModel === 'all' || record.model === filterModel)
                  .map((record) => {
                    const hasConversation = Boolean(record.conversation_text || record.conversation_file);
                    const isExpanded = expandedId === record.id;
                    const conversation = record.conversation_text || loadedConversation[record.id];

                    return (
                      <div key={record.id} className="p-4">
                        <div className="flex items-start gap-3">
                          <div className="flex-1 min-w-0">
                            <div className="flex items-center gap-2 mb-1 flex-wrap">
                              <span className="text-xs bg-purple-100 text-purple-700 px-1.5 py-0.5 rounded">
                                {getModelLabel(record.model)}
                              </span>
                              <span className="text-xs text-gray-500 font-mono">{record.branch_name || record.model}</span>
                              <span className="text-xs text-gray-400">{record.created_at}</span>
                              <span className="text-xs text-gray-400">轮次 {record.interaction_rounds}</span>
                            </div>
                            <div className="text-xs text-gray-500 break-all">Session: {record.session_id}</div>
                            {record.pr_url && (
                              <a href={record.pr_url} target="_blank" rel="noreferrer" className="text-xs text-blue-600 hover:underline break-all">
                                {record.pr_url}
                              </a>
                            )}
                          </div>

                          <div className="flex items-center gap-2 shrink-0">
                            <div className="flex gap-1 text-xs">
                              {(Object.entries(record.scores) as [keyof Scores, number][]).map(([key, value]) => (
                                <span key={key} className={`w-6 h-6 flex items-center justify-center rounded font-medium ${scoreButtonClass(value)}`}>
                                  {value}
                                </span>
                              ))}
                              <span className="ml-1 font-semibold text-gray-700">{totalScore(record)}</span>
                            </div>
                            {hasConversation && (
                              <button
                                onClick={() => {
                                  const next = expandedId === record.id ? null : record.id;
                                  setExpandedId(next);
                                  if (next && record.conversation_file && !loadedConversation[record.id]) {
                                    loadFileContent(record.id, record.conversation_file);
                                  }
                                }}
                                className="text-xs text-gray-500 hover:text-blue-600 px-2 py-1 border border-gray-200 rounded"
                              >
                                {isExpanded ? '收起过程' : '查看过程'}
                              </button>
                            )}
                            {hasConversation && (
                              <button
                                disabled={reanalyzingId === record.id}
                                onClick={() => handleReanalyzeRecord(record)}
                                className="text-xs text-gray-500 hover:text-green-600 px-2 py-1 border border-gray-200 rounded disabled:opacity-50"
                              >
                                {reanalyzingId === record.id ? '分析中...' : '重新分析'}
                              </button>
                            )}
                            <button
                              onClick={() => remove(record.id)}
                              className="text-xs text-gray-500 hover:text-red-600 px-2 py-1 border border-gray-200 rounded"
                            >
                              删除
                            </button>
                          </div>
                        </div>

                        {(record.issue_types.length > 0 || record.fix_cost || record.analysis_summary) && (
                          <div className="mt-2 space-y-2">
                            {record.issue_types.length > 0 && (
                              <div className="flex flex-wrap gap-1">
                                {record.issue_types.map((item) => (
                                  <span key={item} className="text-xs bg-red-50 text-red-600 px-1.5 py-0.5 rounded">{item}</span>
                                ))}
                              </div>
                            )}
                            {record.analysis_summary && (
                              <div className="text-xs text-blue-700 bg-blue-50 border border-blue-100 rounded px-2 py-1.5">
                                {record.analysis_summary}
                              </div>
                            )}
                          </div>
                        )}

                        {isExpanded && (
                          <div className="mt-3 border border-gray-200 rounded-lg overflow-hidden">
                            {record.conversation_file && (
                              <div className="px-3 py-2 bg-gray-50 border-b border-gray-100 text-xs text-gray-500 font-mono break-all">
                                {record.conversation_file}
                              </div>
                            )}
                            <pre className="px-3 py-3 text-xs font-mono leading-relaxed whitespace-pre-wrap max-h-96 overflow-y-auto bg-gray-50 text-gray-700">
                              {conversation || '没有保存对话正文。'}
                            </pre>
                          </div>
                        )}
                      </div>
                    );
                  })}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
