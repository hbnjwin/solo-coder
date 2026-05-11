import { useEffect, useMemo, useState } from 'react';
import {
  AppData,
  GsbRecord,
  GsbWinner,
  MODELS,
  buildGsbReason,
  getModelLabel,
  getQuestionModelOptions,
} from '../types';
import { analyzeGsb, generateId, nowIso } from '../store';

interface Props {
  data: AppData;
  onUpdate: (data: AppData) => void;
}

function normalizedPair(modelA: string, modelB: string) {
  return [modelA, modelB].sort().join('::');
}

export default function Gsb({ data, onUpdate }: Props) {
  const [view, setView] = useState<'list' | 'new'>('list');
  const [questionId, setQuestionId] = useState('');
  const selectedQuestion = data.questions.find((item) => item.id === questionId) ?? null;
  const availableModels = getQuestionModelOptions(selectedQuestion);

  const [modelA, setModelA] = useState(MODELS[0]?.value ?? '');
  const [modelB, setModelB] = useState(MODELS[1]?.value ?? MODELS[0]?.value ?? '');
  const [winner, setWinner] = useState<GsbWinner>('same');
  const [good, setGood] = useState('');
  const [bad, setBad] = useState('');
  const [note, setNote] = useState('');
  const [analyzing, setAnalyzing] = useState(false);
  const [analyzeError, setAnalyzeError] = useState('');

  const recordA = selectedQuestion
    ? data.records.find((item) => item.question_id === selectedQuestion.id && item.model === modelA) ?? null
    : null;
  const recordB = selectedQuestion
    ? data.records.find((item) => item.question_id === selectedQuestion.id && item.model === modelB) ?? null
    : null;

  const canAnalyze = Boolean(selectedQuestion && modelA !== modelB && recordA && recordB);

  useEffect(() => {
    if (availableModels.length < 2) {
      return;
    }
    if (!availableModels.some((item) => item.value === modelA)) {
      setModelA(availableModels[0].value);
    }
    if (!availableModels.some((item) => item.value === modelB) || modelA === modelB) {
      setModelB(availableModels[1]?.value ?? availableModels[0].value);
    }
  }, [availableModels, modelA, modelB]);

  async function handleAnalyze() {
    if (!selectedQuestion || !recordA || !recordB) {
      return;
    }
    setAnalyzing(true);
    setAnalyzeError('');
    try {
      const result = await analyzeGsb(
        selectedQuestion.title,
        getModelLabel(modelA),
        getModelLabel(modelB),
        recordA,
        recordB
      );
      setWinner(result.winner as GsbWinner);
      setGood(result.good);
      setBad(result.bad);
      setNote(result.note);
    } catch (error) {
      setAnalyzeError(String(error));
    } finally {
      setAnalyzing(false);
    }
  }

  function resetForm() {
    setQuestionId('');
    setModelA(MODELS[0]?.value ?? '');
    setModelB(MODELS[1]?.value ?? MODELS[0]?.value ?? '');
    setWinner('same');
    setGood('');
    setBad('');
    setNote('');
    setAnalyzeError('');
  }

  function submit() {
    if (!selectedQuestion) {
      alert('请先选择题目。');
      return;
    }
    if (modelA === modelB) {
      alert('请选择两个不同模型。');
      return;
    }

    const nextRecord: GsbRecord = {
      id: generateId(),
      question_id: selectedQuestion.id,
      model_a: modelA,
      model_b: modelB,
      winner,
      good: good.trim(),
      bad: bad.trim(),
      note: note.trim(),
      reason: buildGsbReason(good.trim(), bad.trim(), note.trim()),
      created_at: nowIso(),
    };

    const existing = data.gsb_records.find((item) => (
      item.question_id === selectedQuestion.id &&
      normalizedPair(item.model_a, item.model_b) === normalizedPair(modelA, modelB)
    ));

    const nextGsbs = existing
      ? data.gsb_records.map((item) => item.id === existing.id ? { ...nextRecord, id: existing.id } : item)
      : [...data.gsb_records, nextRecord];

    onUpdate({ ...data, gsb_records: nextGsbs });
    resetForm();
    setView('list');
  }

  function remove(recordId: string) {
    if (!confirm('确认删除这条 GSB 记录吗？')) {
      return;
    }
    onUpdate({ ...data, gsb_records: data.gsb_records.filter((item) => item.id !== recordId) });
  }

  const groupedGsbs = useMemo(() => data.questions
    .map((question) => ({
      question,
      gsbs: data.gsb_records.filter((item) => item.question_id === question.id),
    }))
    .filter((item) => item.gsbs.length > 0), [data.gsb_records, data.questions]);

  if (view === 'new') {
    return (
      <div className="p-4 max-w-3xl mx-auto">
        <div className="flex items-center gap-3 mb-5">
          <button onClick={() => { resetForm(); setView('list'); }} className="text-sm text-gray-500 hover:text-gray-700">
            返回
          </button>
          <h2 className="font-semibold text-base">新建 GSB 对比</h2>
        </div>

        <div className="space-y-4">
          <div>
            <label className="text-xs text-gray-500 mb-1 block">题目 *</label>
            <select
              value={questionId}
              onChange={(event) => setQuestionId(event.target.value)}
              className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm"
            >
              <option value="">请选择题目</option>
              {data.questions.map((question) => (
                <option key={question.id} value={question.id}>{question.title}</option>
              ))}
            </select>
          </div>

          {selectedQuestion && (
            <div className="rounded-lg border border-blue-200 bg-blue-50 px-3 py-2 text-xs text-blue-700">
              当前题目可对比模型：{selectedQuestion.branches.map(getModelLabel).join('、')}。同一题目下相同 pair 会自动覆盖旧 GSB。
            </div>
          )}

          <div className="grid md:grid-cols-2 gap-3">
            <div>
              <label className="text-xs text-gray-500 mb-1 block">模型 A</label>
              <select value={modelA} onChange={(event) => setModelA(event.target.value)} className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm">
                {availableModels.map((item) => (
                  <option key={item.value} value={item.value}>{item.label}</option>
                ))}
              </select>
              <div className={`mt-1 text-xs ${recordA ? 'text-emerald-600' : 'text-amber-600'}`}>
                {recordA ? `已存在测试记录，总分 ${Object.values(recordA.scores).reduce((sum, value) => sum + value, 0)}` : '该模型还没有测试记录'}
              </div>
            </div>

            <div>
              <label className="text-xs text-gray-500 mb-1 block">模型 B</label>
              <select value={modelB} onChange={(event) => setModelB(event.target.value)} className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm">
                {availableModels.map((item) => (
                  <option key={item.value} value={item.value}>{item.label}</option>
                ))}
              </select>
              <div className={`mt-1 text-xs ${recordB ? 'text-emerald-600' : 'text-amber-600'}`}>
                {recordB ? `已存在测试记录，总分 ${Object.values(recordB.scores).reduce((sum, value) => sum + value, 0)}` : '该模型还没有测试记录'}
              </div>
            </div>
          </div>

          <div className="border border-gray-200 rounded-lg overflow-hidden">
            <div className="bg-gray-50 px-3 py-2 border-b border-gray-200 flex items-center gap-3">
              <span className="text-xs font-medium text-gray-500">AI 自动分析</span>
              {analyzing && <span className="text-xs text-blue-600">分析中...</span>}
              {!analyzing && (good || bad || note) && !analyzeError && <span className="text-xs text-green-600">已生成建议</span>}
              <button
                onClick={handleAnalyze}
                disabled={!canAnalyze || analyzing}
                className="ml-auto px-3 py-1 text-xs bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-40"
              >
                {analyzing ? '分析中...' : '分析两条记录'}
              </button>
            </div>
            {!canAnalyze && selectedQuestion && (
              <div className="px-3 py-2 text-xs text-gray-400">
                只有两个模型都已有测试记录时，才能自动生成 GSB 建议。
              </div>
            )}
            {analyzeError && <div className="px-3 py-2 text-xs text-red-600">{analyzeError}</div>}
          </div>

          <div>
            <label className="text-xs text-gray-500 mb-1.5 block">GSB 结果</label>
            <div className="flex gap-3 flex-wrap">
              {([
                ['A', `A 更好 (${getModelLabel(modelA)})`],
                ['same', 'same'],
                ['B', `B 更好 (${getModelLabel(modelB)})`],
              ] as const).map(([value, label]) => (
                <label key={value} className="flex items-center gap-1.5 text-sm cursor-pointer">
                  <input type="radio" checked={winner === value} onChange={() => setWinner(value)} />
                  {label}
                </label>
              ))}
            </div>
          </div>

          <div className="space-y-3">
            <div>
              <label className="text-xs text-gray-500 mb-1 block">好的模型好在哪</label>
              <textarea value={good} onChange={(event) => setGood(event.target.value)} rows={3} className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm" />
            </div>
            <div>
              <label className="text-xs text-gray-500 mb-1 block">坏的模型坏在哪</label>
              <textarea value={bad} onChange={(event) => setBad(event.target.value)} rows={3} className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm" />
            </div>
            <div>
              <label className="text-xs text-gray-500 mb-1 block">其他备注</label>
              <textarea value={note} onChange={(event) => setNote(event.target.value)} rows={2} className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm" />
            </div>
          </div>

          <div className="flex justify-end gap-2">
            <button onClick={() => { resetForm(); setView('list'); }} className="px-4 py-1.5 text-sm border border-gray-300 rounded hover:bg-gray-50">
              取消
            </button>
            <button onClick={submit} className="px-4 py-1.5 text-sm bg-blue-600 text-white rounded hover:bg-blue-700">
              保存 GSB
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="p-4 max-w-5xl mx-auto">
      <div className="flex items-center mb-4">
        <span className="text-sm text-gray-400">{data.gsb_records.length} 条 GSB 记录</span>
        <button onClick={() => setView('new')} className="ml-auto bg-blue-600 text-white px-3 py-1.5 rounded text-sm hover:bg-blue-700">
          + 新建 GSB
        </button>
      </div>

      {groupedGsbs.length === 0 ? (
        <div className="text-center text-gray-400 py-16 text-sm">暂无 GSB 对比记录</div>
      ) : (
        <div className="space-y-4">
          {groupedGsbs.map(({ question, gsbs }) => (
            <div key={question.id} className="bg-white border border-gray-200 rounded-xl overflow-hidden">
              <div className="px-4 py-3 border-b border-gray-100 bg-gray-50">
                <div className="font-medium text-sm">{question.title}</div>
                <div className="text-xs text-gray-500 mt-1">题目模型：{question.branches.map(getModelLabel).join('、')}</div>
              </div>
              <div className="divide-y divide-gray-100">
                {gsbs.map((record) => (
                  <div key={record.id} className="p-4">
                    <div className="flex items-start gap-3">
                      <div className="flex-1">
                        <div className="flex items-center gap-2 text-xs flex-wrap">
                          <span className="bg-blue-50 text-blue-700 px-1.5 py-0.5 rounded">{getModelLabel(record.model_a)}</span>
                          <span className="text-gray-400">vs</span>
                          <span className="bg-purple-50 text-purple-700 px-1.5 py-0.5 rounded">{getModelLabel(record.model_b)}</span>
                          <span className={`px-1.5 py-0.5 rounded ${record.winner === 'same' ? 'bg-gray-100 text-gray-600' : 'bg-green-100 text-green-700'}`}>
                            {record.winner === 'A' ? `${getModelLabel(record.model_a)} 更好` : record.winner === 'B' ? `${getModelLabel(record.model_b)} 更好` : 'same'}
                          </span>
                          <span className="text-gray-400">{record.created_at}</span>
                        </div>
                        {(record.good || record.bad || record.note || record.reason) && (
                          <div className="mt-2 text-xs text-gray-600 whitespace-pre-line">
                            {record.good && `好的模型好在哪：${record.good}\n\n`}
                            {record.bad && `坏的模型坏在哪：${record.bad}\n\n`}
                            {record.note && `其他备注：${record.note}`}
                            {!record.good && !record.bad && !record.note && record.reason}
                          </div>
                        )}
                      </div>
                      <button
                        onClick={() => remove(record.id)}
                        className="text-xs text-gray-500 hover:text-red-600 px-2 py-1 border border-gray-200 rounded"
                      >
                        删除
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

