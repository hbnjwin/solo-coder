import { Scores, SCORE_CRITERIA } from '../types';
import { useState } from 'react';

interface Props {
  scores: Scores;
  onChange: (s: Scores) => void;
}

export default function ScoreForm({ scores, onChange }: Props) {
  const [expanded, setExpanded] = useState<keyof Scores | null>(null);

  function set(key: keyof Scores, val: number) {
    onChange({ ...scores, [key]: val });
  }

  return (
    <div className="border border-gray-200 rounded-lg overflow-hidden">
      <div className="bg-gray-50 px-3 py-2 text-xs font-medium text-gray-500 border-b border-gray-200">
        评分（各维度 1-5 分）
      </div>
      {(Object.keys(SCORE_CRITERIA) as (keyof Scores)[]).map(key => {
        const { label, tips } = SCORE_CRITERIA[key];
        const val = scores[key];
        const isOpen = expanded === key;
        return (
          <div key={key} className="border-b border-gray-100 last:border-0">
            <div className="flex items-center gap-3 px-3 py-2.5">
              <button onClick={() => setExpanded(isOpen ? null : key)}
                className="text-sm font-medium text-gray-700 flex-1 text-left hover:text-blue-600 flex items-center gap-1">
                <span className={`text-xs transition-transform ${isOpen ? 'rotate-90' : ''}`}>▶</span>
                {label}
              </button>
              <div className="flex gap-1">
                {[1, 2, 3, 4, 5].map(n => (
                  <button key={n} onClick={() => set(key, n)}
                    className={`w-7 h-7 rounded text-sm font-medium transition-colors ${
                      val === n
                        ? n >= 4 ? 'bg-green-500 text-white'
                          : n === 3 ? 'bg-yellow-400 text-white'
                          : 'bg-red-400 text-white'
                        : 'bg-gray-100 text-gray-500 hover:bg-gray-200'
                    }`}>
                    {n}
                  </button>
                ))}
              </div>
            </div>
            {isOpen && (
              <div className="px-3 pb-3 space-y-1">
                {[5, 4, 3, 2, 1].map(n => (
                  <div key={n} onClick={() => set(key, n)}
                    className={`flex gap-2 text-xs p-2 rounded cursor-pointer transition-colors ${
                      val === n ? 'bg-blue-50 border border-blue-200' : 'hover:bg-gray-50'
                    }`}>
                    <span className={`font-bold shrink-0 ${
                      n >= 4 ? 'text-green-600' : n === 3 ? 'text-yellow-600' : 'text-red-500'
                    }`}>{n}分</span>
                    <span className="text-gray-600">{tips[n]}</span>
                  </div>
                ))}
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}
