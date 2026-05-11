import { useState } from 'react';
import { SCORE_CRITERIA, Scores } from '../types';

const SCORE_COLORS: Record<number, string> = {
  5: 'bg-green-50 border-green-200 text-green-800',
  4: 'bg-blue-50 border-blue-200 text-blue-800',
  3: 'bg-yellow-50 border-yellow-200 text-yellow-800',
  2: 'bg-orange-50 border-orange-200 text-orange-800',
  1: 'bg-red-50 border-red-200 text-red-800',
};

const SCORE_BADGE: Record<number, string> = {
  5: 'bg-green-500 text-white',
  4: 'bg-blue-500 text-white',
  3: 'bg-yellow-400 text-white',
  2: 'bg-orange-400 text-white',
  1: 'bg-red-500 text-white',
};

const SCORE_LABEL: Record<number, string> = {
  5: '非常好',
  4: '良好',
  3: '一般',
  2: '较差',
  1: '很差',
};

const ISSUE_TYPES = [
  { name: '幻觉', desc: '引用不存在的库、API、变量或文件路径' },
  { name: '上下文丢失', desc: '忽略了 codebase 中已有的工具类，或忘记了上一轮的对话约束' },
  { name: '指令遵循失败', desc: '明确说了"不要动配置文件"，但它还是改了；或者没按要求输出格式' },
  { name: '死循环', desc: '指出了错误，模型道歉但下一轮输出一模一样的错误代码' },
  { name: '偷懒', desc: '省略主要代码或提供伪代码，且无法通过指令让其补全' },
  { name: '代码破坏', desc: '修复了一个 bug，引发了新 bug；或破坏了原有的正确逻辑' },
  { name: '代码 Bug', desc: '生成的代码存在 bug，无法运行' },
  { name: '废话过多', desc: '代码很少，解释性废话极多，且无法抓住重点' },
  { name: '输出中断', desc: '模型调用工具过程中出错或其他原因导致输出突然中断' },
  { name: '目标漂移/分心', desc: '执行过程中偏离用户主目标，长时间处理无关问题、边缘优化或重复解释' },
  { name: '其他', desc: '其他问题，可以是模型风格等主观体感问题，需要在问题描述中说明' },
];

const FIX_COST = [
  { level: '🟢 低', desc: '模型犯了小错，提醒了一次，它立刻完美修正了' },
  { level: '🟡 中', desc: '模型理解有偏差，反复说明了 2 次，或者需要把报错日志贴给它，它才修好' },
  { level: '🔴 高', desc: '引导超过 2 次仍未解决；或者模型陷入死循环；或者必须自己动手写代码才能解决' },
];

export default function Criteria() {
  const [activeKey, setActiveKey] = useState<keyof Scores>('ux');
  const keys = Object.keys(SCORE_CRITERIA) as (keyof Scores)[];

  return (
    <div className="p-4 max-w-5xl mx-auto">
      <div className="mb-5">
        <h2 className="font-semibold text-base text-gray-800 mb-1">模型表现打分详情</h2>
        <p className="text-xs text-gray-400">共 5 个维度，每个维度 1-5 分，总分 25 分</p>
      </div>

      {/* 维度 Tab */}
      <div className="flex gap-1 mb-4 flex-wrap">
        {keys.map(k => (
          <button key={k} onClick={() => setActiveKey(k)}
            className={`px-3 py-1.5 rounded text-sm font-medium transition-colors ${
              activeKey === k ? 'bg-blue-600 text-white' : 'bg-white border border-gray-200 text-gray-600 hover:bg-gray-50'
            }`}>
            {SCORE_CRITERIA[k].label}
          </button>
        ))}
      </div>

      {/* 当前维度评分表 */}
      <div className="bg-white border border-gray-200 rounded-xl overflow-hidden mb-6">
        <div className="px-4 py-3 bg-gray-50 border-b border-gray-200">
          <span className="font-medium text-sm text-gray-700">{SCORE_CRITERIA[activeKey].label}</span>
          {activeKey === 'ux' && (
            <span className="ml-2 text-xs text-gray-400">站在用户角度，可以带主观意见</span>
          )}
          {activeKey === 'engineering' && (
            <span className="ml-2 text-xs text-gray-400">评估是否主动补充测试、避免只交付"表面代码"</span>
          )}
        </div>
        <div className="divide-y divide-gray-100">
          {[5, 4, 3, 2, 1].map(n => (
            <div key={n} className={`flex gap-4 px-4 py-3 border-l-4 ${SCORE_COLORS[n]}`}>
              <div className="shrink-0 flex flex-col items-center gap-1 w-14">
                <span className={`w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold ${SCORE_BADGE[n]}`}>
                  {n}
                </span>
                <span className="text-xs font-medium">{SCORE_LABEL[n]}</span>
              </div>
              <p className="text-sm leading-relaxed pt-1">{SCORE_CRITERIA[activeKey].tips[n]}</p>
            </div>
          ))}
        </div>
      </div>

      {/* 所有维度总览表 */}
      <div className="bg-white border border-gray-200 rounded-xl overflow-hidden mb-6">
        <div className="px-4 py-3 bg-gray-50 border-b border-gray-200 text-sm font-medium text-gray-700">
          总览：所有维度 × 所有分数
        </div>
        <div className="overflow-x-auto">
          <table className="w-full text-xs">
            <thead>
              <tr className="border-b border-gray-100 bg-gray-50">
                <th className="text-left px-3 py-2 text-gray-500 font-medium w-8">分</th>
                {keys.map(k => (
                  <th key={k} className="text-left px-3 py-2 text-gray-500 font-medium min-w-40">
                    {SCORE_CRITERIA[k].label}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {[5, 4, 3, 2, 1].map(n => (
                <tr key={n} className="border-b border-gray-50 hover:bg-gray-50 align-top">
                  <td className="px-3 py-2">
                    <span className={`w-6 h-6 rounded-full flex items-center justify-center font-bold ${SCORE_BADGE[n]}`}>
                      {n}
                    </span>
                  </td>
                  {keys.map(k => (
                    <td key={k} className="px-3 py-2 text-gray-600 leading-relaxed">
                      {SCORE_CRITERIA[k].tips[n]}
                    </td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* 问题类型 */}
      <div className="bg-white border border-gray-200 rounded-xl overflow-hidden mb-6">
        <div className="px-4 py-3 bg-gray-50 border-b border-gray-200 text-sm font-medium text-gray-700">
          问题类型（满意度 &lt; 5 分时多选）
        </div>
        <div className="divide-y divide-gray-50">
          {ISSUE_TYPES.map(t => (
            <div key={t.name} className="flex gap-3 px-4 py-2.5">
              <span className="shrink-0 text-xs font-medium bg-red-50 text-red-600 px-2 py-0.5 rounded self-start mt-0.5">
                {t.name}
              </span>
              <span className="text-xs text-gray-500 leading-relaxed">{t.desc}</span>
            </div>
          ))}
        </div>
      </div>

      {/* 修复成本 */}
      <div className="bg-white border border-gray-200 rounded-xl overflow-hidden">
        <div className="px-4 py-3 bg-gray-50 border-b border-gray-200 text-sm font-medium text-gray-700">
          修复成本
        </div>
        <div className="divide-y divide-gray-50">
          {FIX_COST.map(f => (
            <div key={f.level} className="flex gap-3 px-4 py-3">
              <span className="shrink-0 text-sm font-medium w-16">{f.level}</span>
              <span className="text-xs text-gray-500 leading-relaxed">{f.desc}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
