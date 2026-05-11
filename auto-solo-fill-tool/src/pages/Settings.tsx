import { useState, useEffect } from 'react';
import { LlmConfig } from '../types';
import { getLlmConfig, saveLlmConfig, testConnection, githubGetUsername } from '../store';

const DEFAULT_CONFIG: LlmConfig = {
  provider: 'anthropic',
  base_url: '',
  api_key: '',
  model: '',
  azure_deployment: '',
  azure_api_version: '2025-04-01-preview',
  github_token: '',
  github_username: '',
};

export default function Settings() {
  const [config, setConfig] = useState<LlmConfig>(DEFAULT_CONFIG);
  const [saved, setSaved] = useState(false);
  const [loading, setLoading] = useState(true);
  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<{ ok: boolean; message: string; reply: string } | null>(null);
  const [ghTesting, setGhTesting] = useState(false);
  const [ghResult, setGhResult] = useState<{ ok: boolean; message: string } | null>(null);

  useEffect(() => {
    getLlmConfig().then(c => {
      setConfig({ ...DEFAULT_CONFIG, ...c });
      setLoading(false);
    });
  }, []);

  function set(patch: Partial<LlmConfig>) {
    setConfig(prev => ({ ...prev, ...patch }));
  }

  async function handleSave() {
    await saveLlmConfig(config);
    setSaved(true);
    setTestResult(null);
    setTimeout(() => setSaved(false), 2000);
  }

  async function handleTest() {
    setTesting(true);
    setTestResult(null);
    await saveLlmConfig(config);
    const r = await testConnection();
    setTestResult({ ok: r.ok, message: r.message, reply: r.model_reply });
    setTesting(false);
  }

  async function handleGithubTest() {
    setGhTesting(true);
    setGhResult(null);
    await saveLlmConfig(config);
    const r = await githubGetUsername();
    if (r.ok) {
      set({ github_username: (r as { ok: true; login: string }).login });
      setGhResult({ ok: true, message: `验证成功，用户名：${(r as { ok: true; login: string }).login}` });
    } else {
      setGhResult({ ok: false, message: (r as { ok: false; error: string }).error });
    }
    setGhTesting(false);
  }

  if (loading) return null;

  const isAzure = config.provider === 'azure';
  const isAnthropic = config.provider === 'anthropic';

  return (
    <div className="max-w-2xl mx-auto space-y-6">

      {/* LLM 配置 */}
      <div>
        <p className="text-xs text-slate-400 mb-4">配置后，AI 质检和过程分析将调用 LLM。</p>
        <div className="bg-white rounded-2xl shadow-sm border border-slate-100 p-6 space-y-5">
          <div>
            <label className="text-xs font-semibold text-slate-500 mb-3 block">接口类型</label>
            <div className="flex gap-2 flex-wrap">
              {([
                ['anthropic', 'Anthropic'],
                ['openai_compat', 'OpenAI 兼容'],
                ['azure', 'Azure OpenAI'],
              ] as const).map(([v, l]) => (
                <button key={v} onClick={() => set({ provider: v as LlmConfig['provider'] })}
                  className={`py-2 px-4 rounded-xl text-sm font-semibold transition-all duration-200 ${
                    config.provider === v
                      ? 'bg-blue-600 text-white shadow-md shadow-blue-500/20'
                      : 'bg-slate-50 text-slate-600 border border-slate-200 hover:border-slate-300 hover:bg-slate-100'
                  }`}>{l}</button>
              ))}
            </div>
          </div>

          {!isAzure && (
            <>
              <div>
                <label className="text-xs font-semibold text-slate-500 mb-2 block">Base URL</label>
                <input value={config.base_url} onChange={e => set({ base_url: e.target.value })}
                  placeholder="https://unifiedapi.cloud"
                  className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all" />
                <p className="text-xs text-slate-400 mt-2">
                  {isAnthropic ? '不含 /v1/messages' : '不含 /v1/chat/completions'}
                </p>
              </div>
              <div>
                <label className="text-xs font-semibold text-slate-500 mb-2 block">模型</label>
                <input value={config.model} onChange={e => set({ model: e.target.value })}
                  placeholder="claude-sonnet-4-6"
                  className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all" />
                <div className="flex gap-2 mt-2 flex-wrap">
                  {(isAnthropic
                    ? ['claude-sonnet-4-6', 'claude-opus-4-6', 'claude-haiku-4-5']
                    : ['claude-sonnet-4-6', 'gpt-4o', 'deepseek-chat']
                  ).map(m => (
                    <button key={m} onClick={() => set({ model: m })} className="text-xs font-medium text-blue-600 bg-blue-50 hover:bg-blue-100 px-3 py-1.5 rounded-lg transition-all">{m}</button>
                  ))}
                </div>
              </div>
            </>
          )}

          {isAzure && (
            <>
              <div>
                <label className="text-xs font-semibold text-slate-500 mb-2 block">Endpoint</label>
                <input value={config.base_url} onChange={e => set({ base_url: e.target.value })}
                  placeholder="https://your-resource.openai.azure.com"
                  className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all" />
              </div>
              <div className="flex gap-4">
                <div className="flex-1">
                  <label className="text-xs font-semibold text-slate-500 mb-2 block">Deployment</label>
                  <input value={config.azure_deployment} onChange={e => set({ azure_deployment: e.target.value })}
                    placeholder="gpt-4o" className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all" />
                </div>
                <div className="flex-1">
                  <label className="text-xs font-semibold text-slate-500 mb-2 block">API Version</label>
                  <input value={config.azure_api_version} onChange={e => set({ azure_api_version: e.target.value })}
                    placeholder="2025-04-01-preview" className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all" />
                </div>
              </div>
            </>
          )}

          <div>
            <label className="text-xs font-semibold text-slate-500 mb-2 block">API Key</label>
            <input type="password" value={config.api_key} onChange={e => set({ api_key: e.target.value })}
              placeholder="sk-..." className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all" />
          </div>

          <div className="pt-4 border-t border-slate-100 space-y-4">
            {testResult && (
              <div className={`rounded-xl p-4 text-sm flex items-start gap-3 ${testResult.ok ? 'bg-emerald-50 border border-emerald-200' : 'bg-rose-50 border border-rose-200'}`}>
                <span className={`w-2 h-2 rounded-full mt-1.5 shrink-0 ${testResult.ok ? 'bg-emerald-500' : 'bg-rose-500'}`} />
                <div>
                  <div className={`font-bold mb-1 ${testResult.ok ? 'text-emerald-700' : 'text-rose-700'}`}>
                    {testResult.ok ? '连接成功' : '连接失败'}
                  </div>
                  <div className="text-slate-500 font-mono break-all text-xs">{testResult.message}</div>
                  {testResult.reply && <div className="mt-1 text-slate-600">模型回复：<span className="font-medium">{testResult.reply}</span></div>}
                </div>
              </div>
            )}
            <div className="flex justify-end gap-3">
              <button onClick={handleTest} disabled={testing || !config.api_key}
                className="px-5 py-2.5 text-sm font-medium text-slate-600 bg-slate-50 border border-slate-200 rounded-xl hover:bg-slate-100 hover:border-slate-300 disabled:opacity-40 transition-all flex items-center gap-1.5">
                {testing && <span className="animate-spin inline-block w-3 h-3 border border-slate-400 border-t-transparent rounded-full" />}
                {testing ? '测试中...' : '测试连接'}
              </button>
              <button onClick={handleSave}
                className="px-5 py-2.5 text-sm font-semibold bg-blue-600 text-white rounded-xl hover:bg-blue-700 hover:shadow-lg hover:shadow-blue-500/25 active:scale-95 transition-all duration-200">
                {saved ? '已保存 ✓' : '保存'}
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* GitHub 配置 */}
      <div>
        <p className="text-xs text-slate-400 mb-4">用于自动创建仓库和分支。需要 repo 权限的 Personal Access Token。</p>
        <div className="bg-white rounded-2xl shadow-sm border border-slate-100 p-6 space-y-5">
          <div>
            <label className="text-xs font-semibold text-slate-500 mb-2 block">Personal Access Token</label>
            <input type="password" value={config.github_token} onChange={e => set({ github_token: e.target.value })}
              placeholder="ghp_..."
              className="w-full bg-slate-50 border border-slate-200 rounded-xl px-4 py-2.5 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-400 transition-all" />
            <p className="text-xs text-slate-400 mt-2">
              需要 <span className="font-mono bg-slate-100 px-1 rounded">repo</span> 权限。
              <a href="https://github.com/settings/tokens/new" target="_blank" rel="noreferrer"
                className="text-blue-600 hover:text-blue-700 font-medium ml-1">生成 Token</a>
            </p>
          </div>
          {config.github_username && (
            <div className="text-sm font-medium text-emerald-600 bg-emerald-50 border border-emerald-200 rounded-xl px-4 py-3 flex items-center gap-2">
              <span className="w-2 h-2 rounded-full bg-emerald-500" />
              已验证用户名：{config.github_username}
            </div>
          )}
          {ghResult && (
            <div className={`rounded-xl p-4 text-sm flex items-center gap-2 ${ghResult.ok ? 'bg-emerald-50 border border-emerald-200 text-emerald-700' : 'bg-rose-50 border border-rose-200 text-rose-600'}`}>
              <span className={`w-2 h-2 rounded-full ${ghResult.ok ? 'bg-emerald-500' : 'bg-rose-500'}`} />
              {ghResult.message}
            </div>
          )}
          <div className="flex justify-end">
            <button onClick={handleGithubTest} disabled={ghTesting || !config.github_token}
              className="px-5 py-2.5 text-sm font-medium text-slate-600 bg-slate-50 border border-slate-200 rounded-xl hover:bg-slate-100 hover:border-slate-300 disabled:opacity-40 transition-all flex items-center gap-1.5">
              {ghTesting && <span className="animate-spin inline-block w-3 h-3 border border-slate-400 border-t-transparent rounded-full" />}
              {ghTesting ? '验证中...' : '验证 Token'}
            </button>
          </div>
        </div>
      </div>

    </div>
  );
}
