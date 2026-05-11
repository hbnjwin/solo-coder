import { useMemo, useState } from 'react';
import { AppData, FIX_COST_LABELS, SCORE_LABELS, Scores, getModelLabel } from '../types';
import { exportCsv, exportXlsx } from '../store';
import { validateDataset } from '../utils/validate';

interface Props {
  data: AppData;
}

const SCORE_KEYS: (keyof Scores)[] = ['ux', 'planning', 'reasoning', 'instruction', 'engineering'];

function getTotalScore(scores: Scores) {
  return SCORE_KEYS.reduce((sum, key) => sum + scores[key], 0);
}

export default function Export({ data }: Props) {
  const [copiedCsv, setCopiedCsv] = useState(false);
  const [copiedJob, setCopiedJob] = useState(false);
  const [xlsxExporting, setXlsxExporting] = useState(false);
  const [viewMode, setViewMode] = useState<'table' | 'job'>('table');

  const validation = useMemo(() => validateDataset(data), [data]);
  const issueCount = validation.questionIssues.size + validation.recordIssues.size + validation.gsbIssues.size;
  const hasErrors = [...validation.questionIssues.values(), ...validation.recordIssues.values(), ...validation.gsbIssues.values()]
    .flat()
    .some((item) => item.level === 'error');

  async function handleExportCsv() {
    const csv = await exportCsv();
    const blob = new Blob(['\uFEFF' + csv], { type: 'text/csv;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement('a');
    anchor.href = url;
    anchor.download = `yingji_records_${new Date().toISOString().slice(0, 10)}.csv`;
    anchor.click();
    URL.revokeObjectURL(url);
  }

  async function handleCopyCsv() {
    const csv = await exportCsv();
    await navigator.clipboard.writeText(csv);
    setCopiedCsv(true);
    setTimeout(() => setCopiedCsv(false), 2000);
  }

  async function handleExportXlsx() {
    setXlsxExporting(true);
    try {
      const bytes = await exportXlsx();
      const blob = new Blob([new Uint8Array(bytes)], {
        type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
      });
      const url = URL.createObjectURL(blob);
      const anchor = document.createElement('a');
      anchor.href = url;
      anchor.download = `yingji_feishu_${new Date().toISOString().slice(0, 10)}.xlsx`;
      anchor.click();
      URL.revokeObjectURL(url);
    } finally {
      setXlsxExporting(false);
    }
  }

  function buildJobFormat() {
    const sections: string[] = [];
    for (const question of data.questions) {
      const records = data.records.filter((item) => item.question_id === question.id);
      const gsbs = data.gsb_records.filter((item) => item.question_id === question.id);

      sections.push(`## 题目：${question.title}`);
      sections.push('');
      sections.push(`技术栈：${question.tech_stack || '未填写'}`);
      sections.push(`任务方向：${question.task_direction || '未填写'}`);
      sections.push(`题目方向：${question.question_direction || '未填写'}`);
      sections.push(`Repo：${question.repo_url || question.github_ssh_url || '未填写'}`);
      sections.push(`模型范围：${question.branches.map(getModelLabel).join('、')}`);
      sections.push('');
      sections.push('Prompt');
      sections.push(question.prompt || '未填写');
      sections.push('');

      for (const record of records) {
        sections.push(`### ${getModelLabel(record.model)}`);
        sections.push(`Session ID：${record.session_id || '未填写'}`);
        sections.push(`GitHub PR：${record.pr_url || '未填写'}`);
        sections.push(`交互轮次：${record.interaction_rounds}`);
        sections.push(`总分：${getTotalScore(record.scores)}`);
        for (const key of SCORE_KEYS) {
          sections.push(`${SCORE_LABELS[key]}：${record.scores[key]}`);
        }
        if (record.long_task_score != null) {
          sections.push(`长程任务：${record.long_task_score}`);
        }
        if (record.frontend_3d_score != null) {
          sections.push(`前端 3D 产物：${record.frontend_3d_score}`);
        }
        if (record.frontend_aesthetic_score != null) {
          sections.push(`前端产物美观度：${record.frontend_aesthetic_score}`);
        }
        if (record.issue_types.length > 0) {
          sections.push(`问题类型：${record.issue_types.join('、')}`);
        }
        if (record.issue_desc) {
          sections.push(`问题描述：${record.issue_desc}`);
        }
        if (record.fix_cost) {
          sections.push(`修复成本：${FIX_COST_LABELS[record.fix_cost]}`);
        }
        if (record.pros) {
          sections.push(`模型优点：${record.pros}`);
        }
        if (record.analysis_summary) {
          sections.push(`分析总结：${record.analysis_summary}`);
        }
        sections.push('');
      }

      if (gsbs.length > 0) {
        sections.push('### GSB 对比');
        sections.push('');
        for (const gsb of gsbs) {
          sections.push(`${getModelLabel(gsb.model_a)} vs ${getModelLabel(gsb.model_b)}：${gsb.winner === 'A' ? getModelLabel(gsb.model_a) : gsb.winner === 'B' ? getModelLabel(gsb.model_b) : 'same'}`);
          if (gsb.good) sections.push(`好的模型好在哪：${gsb.good}`);
          if (gsb.bad) sections.push(`坏的模型坏在哪：${gsb.bad}`);
          if (gsb.note) sections.push(`其他备注：${gsb.note}`);
          sections.push('');
        }
      }

      sections.push('---');
      sections.push('');
    }
    return sections.join('\n');
  }

  async function handleCopyJob() {
    await navigator.clipboard.writeText(buildJobFormat());
    setCopiedJob(true);
    setTimeout(() => setCopiedJob(false), 2000);
  }

  const activeModels = useMemo(() => {
    const values: string[] = [];

    for (const question of data.questions) {
      for (const branch of question.branches) {
        if (!values.includes(branch)) {
          values.push(branch);
        }
      }
    }

    for (const record of data.records) {
      if (!values.includes(record.model)) {
        values.push(record.model);
      }
    }

    for (const gsb of data.gsb_records) {
      if (!values.includes(gsb.model_a)) {
        values.push(gsb.model_a);
      }
      if (!values.includes(gsb.model_b)) {
        values.push(gsb.model_b);
      }
    }

    return values;
  }, [data]);

  const stats = activeModels.map((modelValue) => {
    const records = data.records.filter((item) => item.model === modelValue);
    if (records.length === 0) {
      return { modelValue, label: getModelLabel(modelValue), count: 0, totalAverage: null };
    }
    const totalAverage = records.reduce((sum, item) => sum + getTotalScore(item.scores), 0) / records.length;
    return { modelValue, label: getModelLabel(modelValue), count: records.length, totalAverage };
  });

  return (
    <div className="p-4 max-w-6xl mx-auto">
      {issueCount > 0 && (
        <div className={`mb-4 px-4 py-2.5 rounded-lg border text-sm ${hasErrors ? 'bg-red-50 border-red-200 text-red-700' : 'bg-amber-50 border-amber-200 text-amber-700'}`}>
          <div className="font-medium">{hasErrors ? '质检未通过' : '质检有待确认项'}</div>
          <div className="text-xs mt-1">{validation.summary}</div>
        </div>
      )}

      {issueCount === 0 && (
        <div className="mb-4 px-4 py-2.5 rounded-lg border text-sm bg-green-50 border-green-200 text-green-700">
          <div className="font-medium">当前数据已通过内置质检</div>
          <div className="text-xs mt-1">可以直接导出 xlsx 供飞书上传。</div>
        </div>
      )}

      <div className="flex items-center gap-3 mb-6 flex-wrap">
        <h2 className="font-semibold text-base">导出与质检</h2>
        <div className="ml-auto flex gap-2 flex-wrap">
          <button
            onClick={handleExportXlsx}
            disabled={xlsxExporting}
            className="px-3 py-1.5 text-sm bg-orange-600 text-white rounded hover:bg-orange-700 disabled:opacity-50"
          >
            {xlsxExporting ? '生成中...' : '下载 xlsx（飞书格式）'}
          </button>
          <button onClick={handleCopyJob} className="px-3 py-1.5 text-sm bg-green-600 text-white rounded hover:bg-green-700">
            {copiedJob ? '已复制作业格式' : '复制作业格式'}
          </button>
          <button onClick={handleCopyCsv} className="px-3 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50">
            {copiedCsv ? '已复制 CSV' : '复制 CSV'}
          </button>
          <button onClick={handleExportCsv} className="px-3 py-1.5 text-sm bg-blue-600 text-white rounded hover:bg-blue-700">
            下载 CSV
          </button>
        </div>
      </div>

      <div className="grid md:grid-cols-3 gap-4 mb-6">
        {stats.map((item) => (
          <div key={item.modelValue} className="bg-white border border-gray-200 rounded-xl p-4">
            <div className="font-medium text-sm">{item.label}</div>
            <div className="text-xs text-gray-500 mt-2">{item.count} 条测试记录</div>
            <div className="text-xs text-gray-500 mt-1">
              平均总分：{item.totalAverage == null ? '暂无' : item.totalAverage.toFixed(1)}
            </div>
          </div>
        ))}
      </div>

      <div className="bg-white border border-gray-200 rounded-xl overflow-hidden">
        <div className="px-4 py-2 bg-gray-50 border-b border-gray-200 flex items-center gap-3">
          <span className="text-xs font-medium text-gray-500">导出预览</span>
          <div className="ml-auto flex gap-1">
            {([
              ['table', '表格'],
              ['job', '作业格式'],
            ] as const).map(([value, label]) => (
              <button
                key={value}
                onClick={() => setViewMode(value)}
                className={`px-2 py-0.5 rounded text-xs font-medium ${viewMode === value ? 'bg-blue-600 text-white' : 'text-gray-500 hover:bg-gray-100'}`}
              >
                {label}
              </button>
            ))}
          </div>
        </div>

        {viewMode === 'job' ? (
          <pre className="p-4 text-xs font-mono leading-relaxed overflow-auto whitespace-pre-wrap bg-gray-50 text-gray-700 max-h-[700px]">
            {buildJobFormat()}
          </pre>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full text-xs">
              <thead>
                <tr className="border-b border-gray-100 bg-gray-50">
                  <th className="text-left px-3 py-2 text-gray-500 font-medium">题目</th>
                  <th className="text-left px-3 py-2 text-gray-500 font-medium">模型</th>
                  <th className="text-left px-3 py-2 text-gray-500 font-medium">Session ID</th>
                  <th className="text-center px-2 py-2 text-gray-500 font-medium">总分</th>
                  <th className="text-left px-3 py-2 text-gray-500 font-medium">问题类型</th>
                  <th className="text-left px-3 py-2 text-gray-500 font-medium">修复成本</th>
                  <th className="text-left px-3 py-2 text-gray-500 font-medium">创建时间</th>
                </tr>
              </thead>
              <tbody>
                {data.records.map((record) => {
                  const question = data.questions.find((item) => item.id === record.question_id);
                  return (
                    <tr key={record.id} className="border-b border-gray-50 hover:bg-gray-50">
                      <td className="px-3 py-2 max-w-48 truncate">{question?.title ?? record.question_id}</td>
                      <td className="px-3 py-2">{getModelLabel(record.model)}</td>
                      <td className="px-3 py-2 font-mono text-gray-400 max-w-40 truncate">{record.session_id}</td>
                      <td className="px-2 py-2 text-center font-semibold">{getTotalScore(record.scores)}</td>
                      <td className="px-3 py-2">{record.issue_types.join('、') || '无'}</td>
                      <td className="px-3 py-2">{record.fix_cost ? FIX_COST_LABELS[record.fix_cost] : '无'}</td>
                      <td className="px-3 py-2 text-gray-400 whitespace-nowrap">{record.created_at.slice(0, 10)}</td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
}
