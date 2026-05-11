import { useState, useEffect } from 'react';
import { AppData } from './types';
import { loadData, saveData } from './store';
import QuestionBank from './pages/QuestionBank';
import TaskDelivery from './pages/TaskDelivery';
import Export from './pages/Export';
import Settings from './pages/Settings';

type Tab = 'bank' | 'delivery' | 'export' | 'settings';

const TABS: { key: Tab; label: string }[] = [
  { key: 'bank', label: '题库' },
  { key: 'delivery', label: '交付记录' },
  { key: 'export', label: '导出' },
  { key: 'settings', label: '⚙ 设置' },
];

export default function App() {
  const [tab, setTab] = useState<Tab>('bank');
  const [data, setData] = useState<AppData | null>(null);

  useEffect(() => {
    loadData().then(setData);
  }, []);

  async function update(next: AppData) {
    setData(next);
    await saveData(next);
  }

  if (!data) {
    return (
      <div className="flex items-center justify-center h-screen text-gray-400 text-sm">
        加载中...
      </div>
    );
  }

  return (
    <div className="flex flex-col h-screen bg-slate-50 text-slate-800">
      {/* Header */}
      <header className="bg-gradient-to-r from-slate-900 via-slate-800 to-slate-900 text-white px-6 py-4 flex items-center shadow-lg shrink-0">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 bg-blue-500 rounded-lg flex items-center justify-center shadow-inner">
            <span className="text-white font-bold text-sm">S</span>
          </div>
          <span className="font-bold text-base tracking-wide">Solo Coder 交付工具</span>
        </div>
        <nav className="flex gap-1 ml-8">
          {TABS.map((t) => (
            <button
              key={t.key}
              onClick={() => setTab(t.key)}
              className={`px-4 py-2 rounded-xl text-sm font-medium transition-all duration-200 ${
                tab === t.key
                  ? 'bg-white/20 text-white shadow-inner backdrop-blur-sm'
                  : 'text-slate-300 hover:text-white hover:bg-white/10'
              }`}
            >
              {t.label}
            </button>
          ))}
        </nav>
        <span className="ml-auto text-xs text-slate-400 bg-white/10 px-3 py-1.5 rounded-full">
          {data.questions.length} 题 · {data.records.length} 条记录
        </span>
      </header>

      <main className="flex-1 overflow-auto p-6">
        {tab === 'bank' && <QuestionBank data={data} onUpdate={update} />}
        {tab === 'delivery' && <TaskDelivery data={data} onUpdate={update} />}
        {tab === 'export' && <Export data={data} />}
        {tab === 'settings' && <Settings />}
      </main>
    </div>
  );
}
