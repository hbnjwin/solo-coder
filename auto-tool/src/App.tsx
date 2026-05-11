import { useState, useEffect } from 'react';
import { AppData, setRuntimeModels } from './types';
import { loadData, saveData, getLlmConfig } from './store';
import Questions from './pages/Questions';
import TestRecord from './pages/TestRecord';
import Gsb from './pages/Gsb';
import Export from './pages/Export';
import Criteria from './pages/Criteria';
import Settings from './pages/Settings';
import './index.css';

type Tab = 'questions' | 'record' | 'gsb' | 'export' | 'criteria' | 'settings';

const TABS: { key: Tab; label: string }[] = [
  { key: 'questions', label: '题库' },
  { key: 'record', label: '测试记录' },
  { key: 'gsb', label: 'GSB 对比' },
  { key: 'export', label: '导出' },
  { key: 'criteria', label: '评分标准' },
  { key: 'settings', label: '⚙ 设置' },
];

export default function App() {
  const [tab, setTab] = useState<Tab>('questions');
  const [data, setData] = useState<AppData | null>(null);

  useEffect(() => {
    loadData().then(setData);
    getLlmConfig().then(cfg => {
      if (cfg.models && cfg.models.length > 0) {
        setRuntimeModels(cfg.models);
      }
    });
  }, []);

  async function update(next: AppData) {
    setData(next);
    await saveData(next);
  }

  if (!data) {
    return (
      <div className="flex items-center justify-center h-screen text-gray-400">
        加载中...
      </div>
    );
  }

  return (
    <div className="flex flex-col h-screen bg-gray-50 text-gray-900">
      <header className="bg-white border-b border-gray-200 px-4 py-3 flex items-center gap-6">
        <span className="font-semibold text-blue-600 text-sm">众测辅助工具</span>
        <nav className="flex gap-1">
          {TABS.map((t) => (
            <button
              key={t.key}
              onClick={() => setTab(t.key)}
              className={`px-3 py-1.5 rounded text-sm font-medium transition-colors ${
                tab === t.key
                  ? 'bg-blue-600 text-white'
                  : 'text-gray-600 hover:bg-gray-100'
              }`}
            >
              {t.label}
            </button>
          ))}
        </nav>
        <span className="ml-auto text-xs text-gray-400">
          {data.records.length} 条记录 · {data.questions.length} 道题
        </span>
      </header>

      <main className="flex-1 overflow-auto">
        {tab === 'questions' && <Questions data={data} onUpdate={update} />}
        {tab === 'record' && <TestRecord data={data} onUpdate={update} />}
        {tab === 'gsb' && <Gsb data={data} onUpdate={update} />}
        {tab === 'export' && <Export data={data} />}
        {tab === 'criteria' && <Criteria />}
        {tab === 'settings' && <Settings />}
      </main>
    </div>
  );
}
