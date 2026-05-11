import { useMemo, useState } from 'react';
import { AppData, Question } from '../types';
import { generateId } from '../store';
import { githubCreateRepo, githubCreateBranch } from '../store';

interface Props {
  data: AppData;
  onUpdate: (data: AppData) => void;
}

const EMPTY_Q: Omit<Question, 'id'> = {
  title: '',
  abbr: '',
  description: '',
  prompt: '',
  tech_stack: '',
  repo_url: '',
  github_repo_name: '',
  github_owner: '',
  github_ssh_url: '',
  default_branch: 'main',
};

export default function QuestionBank({ data, onUpdate }: Props) {
  const [editing, setEditing] = useState<Question | null>(null);
  const [form, setForm] = useState<Omit<Question, 'id'>>(EMPTY_Q);
  const [showPrompt, setShowPrompt] = useState<string | null>(null);
  const [ghLoading, setGhLoading] = useState<string | null>(null);
  const [ghMsg, setGhMsg] = useState<{ id: string; ok: boolean; msg: string } | null>(null);
  const [filterText, setFilterText] = useState('');

  const filtered = useMemo(() => {
    if (!filterText.trim()) return data.questions;
    const t = filterText.toLowerCase();
    return data.questions.filter(
      (q) =>
        q.title.toLowerCase().includes(t) ||
        q.abbr.toLowerCase().includes(t) ||
        (q.tech_stack && q.tech_stack.toLowerCase().includes(t))
    );
  }, [data.questions, filterText]);

  function resetEditor() {
    setEditing(null);
    setForm(EMPTY_Q);
  }

  function openNew() {
    setForm(EMPTY_Q);
    setEditing({ id: '', ...EMPTY_Q });
  }

  function openEdit(q: Question) {
    setEditing(q);
    setForm({ ...q });
  }

  function set(patch: Partial<Omit<Question, 'id'>>) {
    setForm((prev) => ({ ...prev, ...patch }));
  }

  function save() {
    if (!form.title.trim() || !form.abbr.trim()) return;
    if (editing?.id) {
      const next = {
        ...data,
        questions: data.questions.map((q) =>
          q.id === editing.id ? { id: editing.id, ...form } : q
        ),
      };
      onUpdate(next);
    } else {
      const q: Question = { id: generateId(), ...form };
      const next = { ...data, questions: [...data.questions, q] };
      onUpdate(next);
    }
    resetEditor();
  }

  function remove(id: string) {
    const next = { ...data, questions: data.questions.filter((q) => q.id !== id) };
    onUpdate(next);
  }

  async function handleCreateRepo(q: Question) {
    if (!q.abbr) return;
    setGhLoading(q.id);
    setGhMsg(null);
    const name = `solo-coder-${q.abbr.toLowerCase()}`;
    const r = await githubCreateRepo(name, q.description || q.title, true);
    if (r.ok) {
      const updated = {
        ...data,
        questions: data.questions.map((item) =>
          item.id === q.id
            ? {
                ...item,
                repo_url: r.repo.html_url,
                github_repo_name: r.repo.full_name.split('/')[1],
                github_owner: r.repo.full_name.split('/')[0],
                github_ssh_url: r.repo.ssh_url,
                default_branch: r.repo.default_branch,
              }
            : item
        ),
      };
      onUpdate(updated);
      setGhMsg({ id: q.id, ok: true, msg: `仓库已创建: ${r.repo.html_url}` });
    } else {
      setGhMsg({ id: q.id, ok: false, msg: r.error });
    }
    setGhLoading(null);
  }

  async function handleCreateBranch(q: Question) {
    if (!q.github_owner || !q.github_repo_name || !q.abbr) return;
    setGhLoading(q.id);
    setGhMsg(null);
    const branchName = `feat/${q.abbr.toLowerCase()}`;
    const r = await githubCreateBranch(q.github_owner, q.github_repo_name, branchName, q.default_branch);
    if (r.ok) {
      setGhMsg({ id: q.id, ok: true, msg: `分支已创建: ${branchName}` });
    } else {
      setGhMsg({ id: q.id, ok: false, msg: r.error });
    }
    setGhLoading(null);
  }

  return (
    <div className="max-w-6xl mx-auto space-y-6">
      {/* 顶部栏 */}
      <div className="flex items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <span className="text-sm font-medium text-slate-500">{filtered.length} 个题目</span>
          <input
            value={filterText}
            onChange={(e) => setFilterText(e.target.value)}
            placeholder="搜索题目..."
            className="w-64 bg-white border border-slate-200 rounded-xl px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all shadow-sm"
          />
        </div>
        <button
          onClick={openNew}
          className="bg-blue-600 text-white px-5 py-2.5 rounded-xl text-sm font-semibold hover:bg-blue-700 hover:shadow-lg hover:shadow-blue-500/25 active:scale-95 transition-all duration-200 flex items-center gap-2"
        >
          <span className="text-lg leading-none">+</span> 新建题目
        </button>
      </div>

      {/* 列表 */}
      <div className="space-y-4">
        {filtered.map((q) => (
          <div
            key={q.id}
            className="bg-white rounded-2xl shadow-sm border border-slate-100 hover:shadow-md hover:border-slate-200 transition-all duration-300 overflow-hidden"
          >
            <div className="p-5">
              <div className="flex items-start gap-4">
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 mb-3 flex-wrap">
                    {q.tech_stack && (
                      <span className="text-xs font-medium bg-indigo-50 text-indigo-600 px-2.5 py-1 rounded-full">
                        {q.tech_stack}
                      </span>
                    )}
                    <span className="text-xs font-mono bg-slate-100 text-slate-600 px-2.5 py-1 rounded-full">
                      {q.abbr}
                    </span>
                  </div>
                  <div className="font-bold text-slate-800 text-base">{q.title}</div>
                  <div className="text-sm text-slate-500 mt-1.5 line-clamp-2">
                    {q.description || '无描述'}
                  </div>
                  {q.repo_url && (
                    <div className="mt-3 space-y-1.5">
                      <a
                        href={q.repo_url}
                        target="_blank"
                        rel="noreferrer"
                        className="block text-xs text-blue-600 hover:text-blue-700 font-medium truncate"
                      >
                        {q.repo_url}
                      </a>
                      {q.github_ssh_url && (
                        <div className="text-xs text-slate-400 font-mono break-all">{q.github_ssh_url}</div>
                      )}
                    </div>
                  )}
                </div>

                <div className="flex flex-wrap gap-2 shrink-0 max-w-xs justify-end content-start">
                  <button
                    onClick={() => setShowPrompt(showPrompt === q.id ? null : q.id)}
                    className={`text-xs font-medium px-3 py-1.5 rounded-lg border transition-all ${
                      showPrompt === q.id
                        ? 'bg-slate-800 text-white border-slate-800'
                        : 'bg-white text-slate-500 border-slate-200 hover:border-slate-400 hover:text-slate-700'
                    }`}
                  >
                    {showPrompt === q.id ? '收起 Prompt' : '查看 Prompt'}
                  </button>
                  <button
                    onClick={() => handleCreateRepo(q)}
                    disabled={!!ghLoading || !q.abbr}
                    className="text-xs font-medium text-blue-600 bg-blue-50 border border-blue-200 hover:bg-blue-100 hover:border-blue-300 px-3 py-1.5 rounded-lg transition-all disabled:opacity-40 disabled:cursor-not-allowed"
                  >
                    {ghLoading === q.id ? '创建中...' : '创建仓库'}
                  </button>
                  <button
                    onClick={() => handleCreateBranch(q)}
                    disabled={!!ghLoading || !q.github_owner}
                    className="text-xs font-medium text-emerald-600 bg-emerald-50 border border-emerald-200 hover:bg-emerald-100 hover:border-emerald-300 px-3 py-1.5 rounded-lg transition-all disabled:opacity-40 disabled:cursor-not-allowed"
                  >
                    {ghLoading === q.id ? '创建中...' : '创建分支'}
                  </button>
                  <button
                    onClick={() => openEdit(q)}
                    className="text-xs font-medium text-slate-600 bg-slate-50 border border-slate-200 hover:bg-slate-100 hover:border-slate-300 px-3 py-1.5 rounded-lg transition-all"
                  >
                    编辑
                  </button>
                  <button
                    onClick={() => remove(q.id)}
                    className="text-xs font-medium text-rose-600 bg-rose-50 border border-rose-200 hover:bg-rose-100 hover:border-rose-300 px-3 py-1.5 rounded-lg transition-all"
                  >
                    删除
                  </button>
                </div>
              </div>

              {showPrompt === q.id && (
                <pre className="mt-4 bg-slate-50 border border-slate-200 rounded-xl p-4 text-xs whitespace-pre-wrap font-mono leading-relaxed text-slate-700">
                  {q.prompt || '（空）'}
                </pre>
              )}

              {ghMsg && ghMsg.id === q.id && (
                <div
                  className={`mt-4 text-sm rounded-xl p-3 flex items-center gap-2 ${
                    ghMsg.ok
                      ? 'bg-emerald-50 border border-emerald-200 text-emerald-700'
                      : 'bg-rose-50 border border-rose-200 text-rose-600'
                  }`}
                >
                  <span className={`w-2 h-2 rounded-full ${ghMsg.ok ? 'bg-emerald-500' : 'bg-rose-500'}`} />
                  {ghMsg.msg}
                </div>
              )}
            </div>
          </div>
        ))}
      </div>

      {filtered.length === 0 && (
        <div className="text-center py-16">
          <div className="w-16 h-16 bg-slate-100 rounded-2xl mx-auto mb-4 flex items-center justify-center">
            <span className="text-slate-300 text-2xl">📋</span>
          </div>
          <p className="text-slate-400 font-medium">
            {filterText ? '无匹配题目' : '暂无题目，点击上方按钮创建'}
          </p>
        </div>
      )}

      {/* 模态框编辑 */}
      {editing && (
        <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-3xl shadow-2xl w-full max-w-3xl max-h-[90vh] overflow-y-auto p-8">
            <h2 className="font-bold text-lg text-slate-800 mb-6">
              {editing.id ? '编辑题目' : '新建题目'}
            </h2>

            <div className="space-y-4">
              <div className="flex gap-4">
                <div className="flex-1">
                  <label className="text-xs font-semibold text-slate-500 mb-2 block">题目名称 *</label>
                  <input
                    value={form.title}
                    onChange={(e) => set({ title: e.target.value })}
                    className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
                    placeholder="如：影院购票跨模块迭代"
                  />
                </div>
                <div>
                  <label className="text-xs font-semibold text-slate-500 mb-2 block">英文缩写 *</label>
                  <input
                    value={form.abbr}
                    onChange={(e) => set({ abbr: e.target.value })}
                    className="w-44 bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
                    placeholder="cinema-iter"
                  />
                </div>
              </div>

              <div>
                <label className="text-xs font-semibold text-slate-500 mb-2 block">题目描述</label>
                <textarea
                  value={form.description}
                  onChange={(e) => set({ description: e.target.value })}
                  rows={2}
                  className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
                  placeholder="简要描述题目内容"
                />
              </div>

              <div>
                <label className="text-xs font-semibold text-slate-500 mb-2 block">技术栈</label>
                <input
                  value={form.tech_stack}
                  onChange={(e) => set({ tech_stack: e.target.value })}
                  className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
                  placeholder="如：Vue3, Vite, TypeScript"
                />
              </div>

              <div>
                <label className="text-xs font-semibold text-slate-500 mb-2 block">
                  User Prompt（必须人话，不带 markdown 格式）*
                </label>
                <textarea
                  value={form.prompt}
                  onChange={(e) => set({ prompt: e.target.value })}
                  rows={6}
                  className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all"
                  placeholder="用自然语言描述你给AI的任务要求，不要用markdown格式，不要用模板化的语言"
                />
                <p className="text-xs text-amber-600 mt-2 bg-amber-50 inline-block px-3 py-1 rounded-full">
                  提示：prompt 必须是人话，不要带 markdown 格式
                </p>
              </div>

              <div className="rounded-xl border border-amber-200/60 bg-amber-50/50 px-4 py-3 text-xs text-amber-700">
                GitHub 仓库信息可以通过上方列表中的「创建仓库」按钮自动填写，无需在此手动输入。
              </div>

              <div className="flex justify-between pt-4">
                <button
                  onClick={resetEditor}
                  className="px-6 py-2.5 text-sm font-medium text-slate-600 bg-slate-50 border border-slate-200 rounded-xl hover:bg-slate-100 hover:border-slate-300 transition-all"
                >
                  取消
                </button>
                <button
                  onClick={save}
                  disabled={!form.title.trim() || !form.abbr.trim()}
                  className="px-6 py-2.5 text-sm font-semibold bg-blue-600 text-white rounded-xl hover:bg-blue-700 hover:shadow-lg hover:shadow-blue-500/25 active:scale-95 disabled:opacity-40 disabled:hover:shadow-none disabled:active:scale-100 transition-all duration-200"
                >
                  {editing.id ? '保存题目' : '创建题目'}
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
