import { useState, useEffect, useMemo } from 'react';
import {
  AppData,
  TaskRecord,
  TASK_TYPE_OPTIONS,
  BUSINESS_DOMAIN_OPTIONS,
  MODIFY_SCOPE_OPTIONS,
  DIFFICULTY_OPTIONS,
  COMPLETION_OPTIONS,
  SATISFACTION_OPTIONS,
} from '../types';
import {
  generateId,
  nowIso,
  getRecentSessions,
  aiQualityCheck,
  processAnalysis,
  generateUnsatisfiedReason,
  openFileDialog,
  readTextFile,
} from '../store';

interface Props {
  data: AppData;
  onUpdate: (data: AppData) => void;
}

const EMPTY_RECORD: Omit<TaskRecord, 'id' | 'created_at'> = {
  question_id: '',
  round_number: 1,
  session_id: '',
  user_prompt: '',
  task_type: '',
  business_domain: '',
  modify_scope: '',
  difficulty: '',
  is_completed: '',
  is_satisfied: '',
  unsatisfied_reason: '',
  github_url: '',
  screenshot_paths: '',
  log_trace: '',
  ai_quality_check_result: '',
  process_analysis_result: '',
};

export default function TaskDelivery({ data, onUpdate }: Props) {
  const [view, setView] = useState<'list' | 'new'>('list');
  const [form, setForm] = useState<Omit<TaskRecord, 'id' | 'created_at'>>({ ...EMPTY_RECORD });
  const [sessions, setSessions] = useState<{
    ok: boolean;
    data?: { session_id: string; last_seen: string }[];
    error?: string;
  } | null>(null);
  const [qcLoading, setQcLoading] = useState(false);
  const [paLoading, setPaLoading] = useState(false);
  const [genLoading, setGenLoading] = useState(false);
  const [saved, setSaved] = useState(false);
  const [expandedId, setExpandedId] = useState<string | null>(null);
  const [filterQuestion, setFilterQuestion] = useState('all');

  useEffect(() => {
    getRecentSessions().then(setSessions);
  }, []);

  const filteredRecords = useMemo(() => {
    if (filterQuestion === 'all') return data.records;
    return data.records.filter((r) => r.question_id === filterQuestion);
  }, [data.records, filterQuestion]);

  function set(patch: Partial<Omit<TaskRecord, 'id' | 'created_at'>>) {
    setForm((prev) => ({ ...prev, ...patch }));
  }

  function selectQuestion(qId: string) {
    const q = data.questions.find((q) => q.id === qId);
    set({
      question_id: qId,
      user_prompt: q?.prompt || form.user_prompt,
      github_url: q?.repo_url || form.github_url,
    });
  }

  function resetForm() {
    setForm({ ...EMPTY_RECORD });
    setSaved(false);
  }

  function openNew() {
    resetForm();
    setView('new');
  }

  async function handleSave() {
    const record: TaskRecord = { id: generateId(), ...form, created_at: nowIso() };
    const next = { ...data, records: [...data.records, record] };
    onUpdate(next);
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  }

  async function handleQualityCheck() {
    setQcLoading(true);
    try {
      const result = await aiQualityCheck(form);
      set({ ai_quality_check_result: result });
    } catch (e) {
      set({ ai_quality_check_result: `质检失败: ${e}` });
    }
    setQcLoading(false);
  }

  async function handleProcessAnalysis() {
    if (!form.log_trace) {
      set({ process_analysis_result: '请先填写日志轨迹' });
      return;
    }
    setPaLoading(true);
    try {
      const result = await processAnalysis(form.user_prompt, form.log_trace, form.round_number);
      set({ process_analysis_result: result });
    } catch (e) {
      set({ process_analysis_result: `分析失败: ${e}` });
    }
    setPaLoading(false);
  }

  async function handleGenerateReason() {
    if (!form.log_trace && !form.user_prompt) {
      set({ unsatisfied_reason: '请先填写 User Prompt 或日志轨迹' });
      return;
    }
    setGenLoading(true);
    try {
      const result = await generateUnsatisfiedReason(form.user_prompt, form.log_trace, form.is_completed);
      set({ unsatisfied_reason: result });
    } catch (e) {
      set({ unsatisfied_reason: `生成失败: ${e}` });
    }
    setGenLoading(false);
  }

  async function handleLoadLogFile() {
    const path = await openFileDialog();
    if (path) {
      const content = await readTextFile(path);
      set({ log_trace: content });
    }
  }

  async function handleCopySessionId(sid: string) {
    set({ session_id: sid });
  }

  const selectedQ = data.questions.find((q) => q.id === form.question_id);

  function completionBadgeClass(v: string) {
    if (v === '完成了任务') return 'bg-green-100 text-green-700';
    return 'bg-red-100 text-red-700';
  }

  function satisfactionBadgeClass(v: string) {
    if (v === '满意') return 'bg-green-100 text-green-700';
    return 'bg-amber-100 text-amber-700';
  }

  return (
    <div className="max-w-6xl mx-auto space-y-6">
      {/* 顶部栏 */}
      <div className="flex items-center justify-between gap-4">
        <div className="flex items-center gap-2 bg-white rounded-xl p-1.5 shadow-sm border border-slate-100">
          <button
            onClick={() => setView('list')}
            className={`px-5 py-2 rounded-lg text-sm font-semibold transition-all ${
              view === 'list'
                ? 'bg-blue-600 text-white shadow-md shadow-blue-500/20'
                : 'text-slate-500 hover:text-slate-700 hover:bg-slate-50'
            }`}
          >
            列表
          </button>
          <button
            onClick={openNew}
            className={`px-5 py-2 rounded-lg text-sm font-semibold transition-all ${
              view === 'new'
                ? 'bg-blue-600 text-white shadow-md shadow-blue-500/20'
                : 'text-slate-500 hover:text-slate-700 hover:bg-slate-50'
            }`}
          >
            + 新建交付
          </button>
        </div>
        {view === 'list' && (
          <select
            value={filterQuestion}
            onChange={(e) => setFilterQuestion(e.target.value)}
            className="bg-white border border-slate-200 rounded-xl px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 shadow-sm"
          >
            <option value="all">全部题目</option>
            {data.questions.map((q) => (
              <option key={q.id} value={q.id}>
                {q.title}
              </option>
            ))}
          </select>
        )}
        <span className="text-sm font-medium text-slate-400">{filteredRecords.length} 条记录</span>
      </div>

      {/* 列表视图 */}
      {view === 'list' && (
        <div className="space-y-4">
          {filteredRecords.map((r) => {
            const q = data.questions.find((item) => item.id === r.question_id);
            const expanded = expandedId === r.id;
            return (
              <div
                key={r.id}
                className="bg-white rounded-2xl shadow-sm border border-slate-100 hover:shadow-md hover:border-slate-200 transition-all duration-300 overflow-hidden"
              >
                <div className="p-5">
                  <div className="flex items-start gap-4">
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2 mb-3 flex-wrap">
                        <span className="font-bold text-slate-800 text-sm">{q?.title || '未知题目'}</span>
                        <span className="text-xs text-slate-400 font-mono bg-slate-100 px-2 py-0.5 rounded-full">第{r.round_number}轮</span>
                        <span className={`text-xs font-medium px-2.5 py-1 rounded-full ${completionBadgeClass(r.is_completed)}`}>
                          {r.is_completed}
                        </span>
                        <span className={`text-xs font-medium px-2.5 py-1 rounded-full ${satisfactionBadgeClass(r.is_satisfied)}`}>
                          {r.is_satisfied}
                        </span>
                      </div>
                      <div className="text-sm text-slate-500">
                        {r.task_type} · {r.business_domain} · {r.modify_scope} · {r.difficulty}
                      </div>
                      <div className="text-xs text-slate-400 mt-2">{r.created_at}</div>
                    </div>
                    <button
                      onClick={() => setExpandedId(expanded ? null : r.id)}
                      className={`text-xs font-medium px-3 py-1.5 rounded-lg border transition-all shrink-0 ${
                        expanded
                          ? 'bg-slate-800 text-white border-slate-800'
                          : 'bg-white text-slate-500 border-slate-200 hover:border-slate-400 hover:text-slate-700'
                      }`}
                    >
                      {expanded ? '收起' : '展开'}
                    </button>
                  </div>

                  {expanded && (
                    <div className="mt-4 space-y-3 text-xs">
                      {r.session_id && (
                        <div className="text-slate-500 font-mono bg-slate-50 px-3 py-2 rounded-lg">Session: {r.session_id}</div>
                      )}
                      {r.github_url && (
                        <a
                          href={r.github_url}
                          target="_blank"
                          rel="noreferrer"
                          className="block text-blue-600 hover:text-blue-700 font-medium truncate"
                        >
                          {r.github_url}
                        </a>
                      )}
                      {r.user_prompt && (
                        <pre className="bg-slate-50 border border-slate-200 rounded-xl p-4 whitespace-pre-wrap font-mono leading-relaxed text-slate-700">
                          {r.user_prompt}
                        </pre>
                      )}
                      {r.unsatisfied_reason && (
                        <div className="bg-amber-50 border border-amber-200 rounded-xl p-4 text-amber-800">
                          <div className="font-bold mb-1">不满意原因</div>
                          {r.unsatisfied_reason}
                        </div>
                      )}
                      {r.ai_quality_check_result && (
                        <div className="bg-purple-50 border border-purple-200 rounded-xl p-4 text-purple-800">
                          <div className="font-bold mb-1">AI 质检</div>
                          {r.ai_quality_check_result}
                        </div>
                      )}
                      {r.process_analysis_result && (
                        <div className="bg-teal-50 border border-teal-200 rounded-xl p-4 text-teal-800">
                          <div className="font-bold mb-1">过程分析</div>
                          {r.process_analysis_result}
                        </div>
                      )}
                    </div>
                  )}
                </div>
              </div>
            );
          })}
          {filteredRecords.length === 0 && (
            <div className="text-center py-16">
              <div className="w-16 h-16 bg-slate-100 rounded-2xl mx-auto mb-4 flex items-center justify-center">
                <span className="text-slate-300 text-2xl">📝</span>
              </div>
              <p className="text-slate-400 font-medium">
                {filterQuestion !== 'all' ? '该题目暂无交付记录' : '暂无交付记录，点击「新建交付」创建'}
              </p>
            </div>
          )}
        </div>
      )}

      {/* 新建视图 */}
      {view === 'new' && (
        <div className="bg-white rounded-2xl shadow-sm border border-slate-100 p-6 space-y-5 max-w-4xl">
          {/* 选择题目 */}
          <div>
            <label className="text-xs font-semibold text-slate-500 mb-2 block">选择题目 *</label>
            <select
              value={form.question_id}
              onChange={(e) => selectQuestion(e.target.value)}
              className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
            >
              <option value="">-- 请选择 --</option>
              {data.questions.map((q) => (
                <option key={q.id} value={q.id}>
                  {q.title} ({q.abbr})
                </option>
              ))}
            </select>
            {selectedQ && (
              <p className="text-xs text-slate-400 mt-2">
                技术栈: {selectedQ.tech_stack} | 仓库: {selectedQ.repo_url || '未配置'}
              </p>
            )}
          </div>

          {/* 轮次 + Session ID */}
          <div className="flex gap-4">
            <div className="w-28">
              <label className="text-xs font-semibold text-slate-500 mb-2 block">轮次</label>
              <input
                type="number"
                min={1}
                max={5}
                value={form.round_number}
                onChange={(e) => set({ round_number: parseInt(e.target.value) || 1 })}
                className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
              />
            </div>
            <div className="flex-1">
              <label className="text-xs font-semibold text-slate-500 mb-2 block">Trae Session ID</label>
              <div className="flex gap-2">
                <input
                  value={form.session_id}
                  onChange={(e) => set({ session_id: e.target.value })}
                  className="flex-1 bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
                  placeholder="双击头像复制"
                />
                {sessions?.ok && sessions.data && sessions.data.length > 0 && (
                  <select
                    onChange={(e) => {
                      if (e.target.value) handleCopySessionId(e.target.value);
                    }}
                    className="bg-slate-50 border border-slate-200 rounded-xl px-3 py-2.5 text-xs focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400"
                    title="最近 Session"
                  >
                    <option value="">最近</option>
                    {sessions.data.map((s) => (
                      <option key={s.session_id} value={s.session_id}>
                        {s.session_id.slice(0, 8)}...
                      </option>
                    ))}
                  </select>
                )}
              </div>
            </div>
          </div>

          {/* User Prompt */}
          <div>
            <label className="text-xs font-semibold text-slate-500 mb-2 block">
              User Prompt（必须人话，不带 markdown 格式）*
            </label>
            <textarea
              value={form.user_prompt}
              onChange={(e) => set({ user_prompt: e.target.value })}
              className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
              rows={4}
              placeholder="用自然语言描述你给AI的任务要求，像跟同事说话一样，不要用markdown格式"
            />
            <p className="text-xs text-amber-600 mt-2 bg-amber-50 inline-block px-3 py-1 rounded-full">
              注意：prompt 必须是人话，不要带 markdown 格式
            </p>
          </div>

          {/* 任务类型 / 业务领域 */}
          <div className="flex gap-4">
            <div className="flex-1">
              <label className="text-xs font-semibold text-slate-500 mb-2 block">任务类型</label>
              <select
                value={form.task_type}
                onChange={(e) => set({ task_type: e.target.value })}
                className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
              >
                <option value="">-- 请选择 --</option>
                {TASK_TYPE_OPTIONS.map((o) => (
                  <option key={o} value={o}>
                    {o}
                  </option>
                ))}
              </select>
            </div>
            <div className="flex-1">
              <label className="text-xs font-semibold text-slate-500 mb-2 block">业务领域</label>
              <select
                value={form.business_domain}
                onChange={(e) => set({ business_domain: e.target.value })}
                className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
              >
                <option value="">-- 请选择 --</option>
                {BUSINESS_DOMAIN_OPTIONS.map((o) => (
                  <option key={o} value={o}>
                    {o}
                  </option>
                ))}
              </select>
            </div>
          </div>

          {/* 修改范围 / 任务难度 */}
          <div className="flex gap-4">
            <div className="flex-1">
              <label className="text-xs font-semibold text-slate-500 mb-2 block">修改范围</label>
              <select
                value={form.modify_scope}
                onChange={(e) => set({ modify_scope: e.target.value })}
                className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
              >
                <option value="">-- 请选择 --</option>
                {MODIFY_SCOPE_OPTIONS.map((o) => (
                  <option key={o} value={o}>
                    {o}
                  </option>
                ))}
              </select>
            </div>
            <div className="flex-1">
              <label className="text-xs font-semibold text-slate-500 mb-2 block">任务难度</label>
              <select
                value={form.difficulty}
                onChange={(e) => set({ difficulty: e.target.value })}
                className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
              >
                <option value="">-- 请选择 --</option>
                {DIFFICULTY_OPTIONS.map((o) => (
                  <option key={o} value={o}>
                    {o}
                  </option>
                ))}
              </select>
            </div>
          </div>

          {/* 任务是否完成 / 产物及过程是否满意 */}
          <div className="flex gap-4">
            <div className="flex-1">
              <label className="text-xs font-semibold text-slate-500 mb-2 block">任务是否完成</label>
              <div className="flex gap-2">
                {COMPLETION_OPTIONS.map((o) => (
                  <button
                    key={o}
                    onClick={() => set({ is_completed: o })}
                    className={`flex-1 py-2.5 rounded-xl text-sm font-semibold border-2 transition-all duration-200 ${
                      form.is_completed === o
                        ? 'bg-blue-600 text-white border-blue-600 shadow-md shadow-blue-500/20'
                        : 'bg-white text-slate-600 border-slate-200 hover:border-slate-300 hover:bg-slate-50'
                    }`}
                  >
                    {o}
                  </button>
                ))}
              </div>
            </div>
            <div className="flex-1">
              <label className="text-xs font-semibold text-slate-500 mb-2 block">产物及过程是否满意</label>
              <div className="flex gap-2">
                {SATISFACTION_OPTIONS.map((o) => (
                  <button
                    key={o}
                    onClick={() => set({ is_satisfied: o })}
                    className={`flex-1 py-2.5 rounded-xl text-sm font-semibold border-2 transition-all duration-200 ${
                      form.is_satisfied === o
                        ? 'bg-blue-600 text-white border-blue-600 shadow-md shadow-blue-500/20'
                        : 'bg-white text-slate-600 border-slate-200 hover:border-slate-300 hover:bg-slate-50'
                    }`}
                  >
                    {o}
                  </button>
                ))}
              </div>
            </div>
          </div>

          {/* 不满意原因 */}
          {form.is_satisfied === '不满意' && (
            <div>
              <div className="flex items-center justify-between mb-2">
                <label className="text-xs font-semibold text-slate-500">不满意原因</label>
                <button
                  onClick={handleGenerateReason}
                  disabled={genLoading}
                  className="text-xs font-medium text-purple-600 bg-purple-50 border border-purple-200 hover:bg-purple-100 hover:border-purple-300 px-3 py-1.5 rounded-lg transition-all disabled:opacity-40"
                >
                  {genLoading ? '生成中...' : 'AI 生成'}
                </button>
              </div>
              <textarea
                value={form.unsatisfied_reason}
                onChange={(e) => set({ unsatisfied_reason: e.target.value })}
                className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
                rows={3}
                placeholder="产物不满意：xxx  过程不满意：xxx"
              />
              <p className="text-xs text-amber-600 mt-2 bg-amber-50 inline-block px-3 py-1 rounded-full">
                AI 生成的内容也要像人写的，检查后再提交
              </p>
            </div>
          )}

          {/* GitHub 地址 */}
          <div>
            <label className="text-xs font-semibold text-slate-500 mb-2 block">GitHub 地址</label>
            <input
              value={form.github_url}
              onChange={(e) => set({ github_url: e.target.value })}
              className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
              placeholder="https://github.com/..."
            />
          </div>

          {/* 截图 */}
          <div>
            <label className="text-xs font-semibold text-slate-500 mb-2 block">截图（产物/运行结果/对话）</label>
            <input
              value={form.screenshot_paths}
              onChange={(e) => set({ screenshot_paths: e.target.value })}
              className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
              placeholder="截图文件路径或描述"
            />
          </div>

          {/* 日志轨迹 */}
          <div>
            <div className="flex items-center justify-between mb-2">
              <label className="text-xs font-semibold text-slate-500">日志轨迹</label>
              <button
                onClick={handleLoadLogFile}
                className="text-xs font-medium text-blue-600 bg-blue-50 border border-blue-200 hover:bg-blue-100 hover:border-blue-300 px-3 py-1.5 rounded-lg transition-all"
              >
                从文件加载
              </button>
            </div>
            <textarea
              value={form.log_trace}
              onChange={(e) => set({ log_trace: e.target.value })}
              className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
              rows={4}
              placeholder="粘贴日志轨迹，或从文件加载"
            />
          </div>

          {/* AI 质检 + 过程分析 */}
          <div className="flex gap-4">
            <div className="flex-1">
              <button
                onClick={handleQualityCheck}
                disabled={qcLoading || !form.user_prompt}
                className="w-full px-4 py-2.5 text-sm font-medium border-2 border-purple-200 text-purple-700 rounded-xl hover:bg-purple-50 hover:border-purple-300 disabled:opacity-40 transition-all"
              >
                {qcLoading ? '质检中...' : 'AI 质检'}
              </button>
              {form.ai_quality_check_result && (
                <div className="mt-3 text-xs bg-purple-50 border border-purple-200 rounded-xl p-4 whitespace-pre-wrap text-purple-800 leading-relaxed">
                  {form.ai_quality_check_result}
                </div>
              )}
            </div>
            <div className="flex-1">
              <button
                onClick={handleProcessAnalysis}
                disabled={paLoading || !form.log_trace}
                className="w-full px-4 py-2.5 text-sm font-medium border-2 border-teal-200 text-teal-700 rounded-xl hover:bg-teal-50 hover:border-teal-300 disabled:opacity-40 transition-all"
              >
                {paLoading ? '分析中...' : '过程分析'}
              </button>
              {form.process_analysis_result && (
                <div className="mt-3 text-xs bg-teal-50 border border-teal-200 rounded-xl p-4 whitespace-pre-wrap text-teal-800 leading-relaxed">
                  {form.process_analysis_result}
                </div>
              )}
            </div>
          </div>

          {/* 保存 */}
          <div className="flex justify-between pt-4 border-t border-slate-100">
            <button
              onClick={() => {
                resetForm();
                setView('list');
              }}
              className="px-6 py-2.5 text-sm font-medium text-slate-600 bg-slate-50 border border-slate-200 rounded-xl hover:bg-slate-100 hover:border-slate-300 transition-all"
            >
              取消
            </button>
            <button
              onClick={handleSave}
              disabled={!form.question_id || !form.user_prompt}
              className="px-6 py-2.5 text-sm font-semibold bg-blue-600 text-white rounded-xl hover:bg-blue-700 hover:shadow-lg hover:shadow-blue-500/25 active:scale-95 disabled:opacity-40 disabled:hover:shadow-none disabled:active:scale-100 transition-all duration-200"
            >
              {saved ? '已保存 ✓' : '保存交付记录'}
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
