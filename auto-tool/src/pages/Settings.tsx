import { useState, useEffect } from 'react';
import { LlmConfig, ModelConfig, DEFAULT_MODELS } from '../types';
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
  models: [...DEFAULT_MODELS],
};

export default function Settings() {
  const [config, setConfig] = useState<LlmConfig>(DEFAULT_CONFIG);
  const [saved, setSaved] = useState(false);
  const [loading, setLoading] = useState(true);
  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<{ ok: boolean; message: string; reply: string } | null>(null);
  const [ghTesting, setGhTesting] = useState(false);
  const [ghResult, setGhResult] = useState<{ ok: boolean; message: string } | null>(null);
  const [editingModel, setEditingModel] = useState<number | null>(null);

  useEffect(() => {
    getLlmConfig().then(c => {
      setConfig({
        ...DEFAULT_CONFIG,
        ...c,
        models: c.models && c.models.length > 0 ? c.models : [...DEFAULT_MODELS],
      });
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
      set({ github_username: r.login });
      setGhResult({ ok: true, message: `验证成功，用户名：${r.login}` });
    } else {
      setGhResult({ ok: false, message: r.error });
    }
    setGhTesting(false);
  }

  function updateModel(idx: number, patch: Partial<ModelConfig>) {
    const models = config.models.map((m, i) => i === idx ? { ...m, ...patch } : m);
    set({ models });
  }

  function addModel() {
    set({ models: [...config.models, { value: '', label: '' }] });
    setEditingModel(config.models.length);
  }

  function removeModel(idx: number) {
    set({ models: config.models.filter((_, i) => i !== idx) });
  }

  if (loading) return null;

  const isAzure = config.provider === 'azure';
  const isAnthropic = config.provider === 'anthropic';

  return (
    <div className="p-4 max-w-xl mx-auto space-y-5">

      {/* LLM 配置 */}
      <div>
        <h2 className="font-semibold text-base mb-1">LLM 分析配置</h2>
        <p className="text-xs text-gray-400 mb-4">配置后，填写对话过程记录时将自动调用 LLM 分析并预填评分。</p>
        <div className="bg-white border border-gray-200 rounded-xl p-4 space-y-4">
          <div>
            <label className="text-xs text-gray-500 mb-1.5 block">接口类型</label>
            <div className="flex gap-2 flex-wrap">
              {([
                ['anthropic', 'Anthropic'],
                ['openai_compat', 'OpenAI 兼容'],
                ['azure', 'Azure OpenAI'],
              ] as const).map(([v, l]) => (
                <button key={v} onClick={() => set({ provider: v })}
                  className={`py-1.5 px-3 rounded-lg border text-sm font-medium transition-colors ${
                    config.provider === v ? 'bg-blue-600 text-white border-blue-600' : 'bg-white text-gray-600 border-gray-200 hover:bg-gray-50'
                  }`}>{l}</button>
              ))}
            </div>
          </div>

          {!isAzure && (
            <>
              <div>
                <label className="text-xs text-gray-500 mb-1 block">Base URL</label>
                <input value={config.base_url} onChange={e => set({ base_url: e.target.value })}
                  placeholder="https://unifiedapi.cloud"
                  className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm font-mono" />
                <p className="text-xs text-gray-400 mt-1">
                  {isAnthropic ? '不含 /v1/messages' : '不含 /v1/chat/completions'}
                </p>
              </div>
              <div>
                <label className="text-xs text-gray-500 mb-1 block">模型</label>
                <input value={config.model} onChange={e => set({ model: e.target.value })}
                  placeholder="claude-sonnet-4-6"
                  className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm font-mono" />
                <div className="flex gap-2 mt-1.5 flex-wrap">
                  {(isAnthropic
                    ? ['claude-sonnet-4-6', 'claude-opus-4-6', 'claude-haiku-4-5']
                    : ['claude-sonnet-4-6', 'gpt-4o', 'deepseek-chat']
                  ).map(m => (
                    <button key={m} onClick={() => set({ model: m })} className="text-xs text-blue-500 hover:underline">{m}</button>
                  ))}
                </div>
              </div>
            </>
          )}

          {isAzure && (
            <>
              <div>
                <label className="text-xs text-gray-500 mb-1 block">Endpoint</label>
                <input value={config.base_url} onChange={e => set({ base_url: e.target.value })}
                  placeholder="https://your-resource.openai.azure.com"
                  className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm font-mono" />
              </div>
              <div className="flex gap-3">
                <div className="flex-1">
                  <label className="text-xs text-gray-500 mb-1 block">Deployment</label>
                  <input value={config.azure_deployment} onChange={e => set({ azure_deployment: e.target.value })}
                    placeholder="gpt-4o" className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm font-mono" />
                </div>
                <div className="flex-1">
                  <label className="text-xs text-gray-500 mb-1 block">API Version</label>
                  <input value={config.azure_api_version} onChange={e => set({ azure_api_version: e.target.value })}
                    placeholder="2025-04-01-preview" className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm font-mono" />
                </div>
              </div>
            </>
          )}

          <div>
            <label className="text-xs text-gray-500 mb-1 block">API Key</label>
            <input type="password" value={config.api_key} onChange={e => set({ api_key: e.target.value })}
              placeholder="sk-..." className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm font-mono" />
          </div>

          <div className="pt-2 border-t border-gray-100 space-y-3">
            {testResult && (
              <div className={`rounded-lg p-3 text-xs ${testResult.ok ? 'bg-green-50 border border-green-200' : 'bg-red-50 border border-red-200'}`}>
                <div className={`font-medium mb-1 ${testResult.ok ? 'text-green-700' : 'text-red-700'}`}>
                  {testResult.ok ? '✓ 连接成功' : '✗ 连接失败'}
                </div>
                <div className="text-gray-500 font-mono break-all">{testResult.message}</div>
                {testResult.reply && <div className="mt-1 text-gray-600">模型回复：<span className="font-medium">{testResult.reply}</span></div>}
              </div>
            )}
            <div className="flex justify-end gap-2">
              <button onClick={handleTest} disabled={testing || !config.api_key}
                className="px-4 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50 disabled:opacity-40 flex items-center gap-1.5">
                {testing && <span className="animate-spin inline-block w-3 h-3 border border-gray-400 border-t-transparent rounded-full" />}
                {testing ? '测试中...' : '测试连接'}
              </button>
              <button onClick={handleSave}
                className="px-4 py-1.5 text-sm bg-blue-600 text-white rounded hover:bg-blue-700">
                {saved ? '已保存 ✓' : '保存'}
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* GitHub 配置 */}
      <div>
        <h2 className="font-semibold text-base mb-1">GitHub 配置</h2>
        <p className="text-xs text-gray-400 mb-4">用于自动创建仓库、分支和 PR。需要 repo 权限的 Personal Access Token。</p>
        <div className="bg-white border border-gray-200 rounded-xl p-4 space-y-4">
          <div>
            <label className="text-xs text-gray-500 mb-1 block">Personal Access Token</label>
            <input type="password" value={config.github_token} onChange={e => set({ github_token: e.target.value })}
              placeholder="ghp_..."
              className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm font-mono" />
            <p className="text-xs text-gray-400 mt-1">
              需要 <span className="font-mono">repo</span> 权限。
              <a href="https://github.com/settings/tokens/new" target="_blank" rel="noreferrer"
                className="text-blue-500 hover:underline ml-1">生成 Token →</a>
            </p>
          </div>
          {config.github_username && (
            <div className="text-xs text-green-600">✓ 已验证用户名：{config.github_username}</div>
          )}
          {ghResult && (
            <div className={`rounded-lg p-2 text-xs ${ghResult.ok ? 'bg-green-50 text-green-700' : 'bg-red-50 text-red-600'}`}>
              {ghResult.message}
            </div>
          )}
          <div className="flex justify-end">
            <button onClick={handleGithubTest} disabled={ghTesting || !config.github_token}
              className="px-4 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50 disabled:opacity-40 flex items-center gap-1.5">
              {ghTesting && <span className="animate-spin inline-block w-3 h-3 border border-gray-400 border-t-transparent rounded-full" />}
              {ghTesting ? '验证中...' : '验证 Token'}
            </button>
          </div>
        </div>
      </div>

      {/* 模型配置 */}
      <div>
        <h2 className="font-semibold text-base mb-1">测试模型配置</h2>
        <p className="text-xs text-gray-400 mb-4">配置本期参与众测的模型，value 用于分支名，label 用于显示。</p>
        <div className="bg-white border border-gray-200 rounded-xl p-4 space-y-2">
          {config.models.map((m, idx) => (
            <div key={idx} className="flex gap-2 items-center">
              {editingModel === idx ? (
                <>
                  <input value={m.value} onChange={e => updateModel(idx, { value: e.target.value })}
                    placeholder="kimi-k2.5（分支名）"
                    className="flex-1 border border-gray-300 rounded px-2 py-1 text-xs font-mono" />
                  <input value={m.label} onChange={e => updateModel(idx, { label: e.target.value })}
                    placeholder="Kimi K2.5（显示名）"
                    className="flex-1 border border-gray-300 rounded px-2 py-1 text-xs" />
                  <button onClick={() => setEditingModel(null)}
                    className="text-xs text-green-600 hover:underline px-2">完成</button>
                </>
              ) : (
                <>
                  <span className="flex-1 text-xs font-mono text-gray-600 bg-gray-50 rounded px-2 py-1">{m.value || '(空)'}</span>
                  <span className="flex-1 text-xs text-gray-700 bg-gray-50 rounded px-2 py-1">{m.label || '(空)'}</span>
                  <button onClick={() => setEditingModel(idx)} className="text-xs text-blue-500 hover:underline px-2">编辑</button>
                </>
              )}
              <button onClick={() => removeModel(idx)} className="text-xs text-red-400 hover:text-red-600 px-1">✕</button>
            </div>
          ))}
          <button onClick={addModel}
            className="mt-2 text-xs text-blue-500 hover:underline">+ 添加模型</button>
          <div className="pt-3 border-t border-gray-100 flex justify-end">
            <button onClick={handleSave}
              className="px-4 py-1.5 text-sm bg-blue-600 text-white rounded hover:bg-blue-700">
              {saved ? '已保存 ✓' : '保存'}
            </button>
          </div>
        </div>
      </div>

    </div>
  );
}
