import { useState } from 'react';
import { AppData } from '../types';
import { exportXlsx } from '../store';

interface Props {
  data: AppData;
}

export default function Export({ data }: Props) {
  const [exporting, setExporting] = useState(false);
  const [msg, setMsg] = useState<{ ok: boolean; text: string } | null>(null);

  async function handleExport() {
    setExporting(true);
    setMsg(null);
    try {
      const bytes = await exportXlsx();
      const blob = new Blob([new Uint8Array(bytes)], {
        type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
      });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `solo-coder-delivery-${new Date().toISOString().slice(0, 10)}.xlsx`;
      a.click();
      URL.revokeObjectURL(url);
      setMsg({ ok: true, text: `导出成功，共 ${data.records.length} 条记录` });
    } catch (e) {
      setMsg({ ok: false, text: String(e) });
    }
    setExporting(false);
  }

  return (
    <div className="max-w-2xl mx-auto">
      <div className="bg-white rounded-2xl shadow-sm border border-slate-100 p-6 space-y-5">
        <div className="flex items-center justify-between">
          <div className="space-y-2">
            <div className="flex items-center gap-3">
              <span className="text-sm text-slate-500">交付记录数</span>
              <span className="text-2xl font-bold text-slate-800">{data.records.length}</span>
            </div>
            <div className="flex items-center gap-3">
              <span className="text-sm text-slate-500">题目数</span>
              <span className="text-2xl font-bold text-slate-800">{data.questions.length}</span>
            </div>
          </div>
          <button
            onClick={handleExport}
            disabled={exporting || data.records.length === 0}
            className="px-6 py-3 text-sm font-semibold bg-emerald-600 text-white rounded-xl hover:bg-emerald-700 hover:shadow-lg hover:shadow-emerald-500/25 active:scale-95 disabled:opacity-40 disabled:hover:shadow-none disabled:active:scale-100 transition-all duration-200"
          >
            {exporting ? '导出中...' : '导出 XLSX'}
          </button>
        </div>

        {msg && (
          <div className={`rounded-xl p-4 text-sm flex items-center gap-2 ${msg.ok ? 'bg-emerald-50 border border-emerald-200 text-emerald-700' : 'bg-rose-50 border border-rose-200 text-rose-600'}`}>
            <span className={`w-2 h-2 rounded-full ${msg.ok ? 'bg-emerald-500' : 'bg-rose-500'}`} />
            {msg.text}
          </div>
        )}

        {data.records.length === 0 && (
          <div className="text-center py-8">
            <div className="w-16 h-16 bg-slate-100 rounded-2xl mx-auto mb-3 flex items-center justify-center">
              <span className="text-slate-300 text-2xl">📊</span>
            </div>
            <p className="text-sm text-slate-400 font-medium">暂无交付记录，请先在任务交付页填写</p>
          </div>
        )}
      </div>
    </div>
  );
}
