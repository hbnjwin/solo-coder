import { invoke } from '@tauri-apps/api/core';
import { AnalyzeResult, AppData, LlmConfig } from './types';

export interface TestConnectionResult {
  ok: boolean;
  message: string;
  model_reply: string;
}

export interface SessionInfo {
  session_id: string;
  last_seen: string;
}

export interface GithubRepo {
  full_name: string;
  html_url: string;
  ssh_url: string;
  default_branch: string;
}

export async function loadData(): Promise<AppData> {
  return invoke<AppData>('read_data');
}

export async function saveData(data: AppData): Promise<void> {
  return invoke('write_data', { data });
}

export async function exportCsv(): Promise<string> {
  return invoke<string>('export_csv');
}

export async function exportXlsxQuestion(questionId: string): Promise<number[]> {
  return invoke<number[]>('export_xlsx_question', { questionId });
}

export async function exportXlsx(): Promise<number[]> {
  return invoke<number[]>('export_xlsx');
}

export async function openFileDialog(): Promise<string | null> {
  return invoke<string | null>('open_file_dialog');
}

export async function readTextFile(path: string): Promise<string> {
  return invoke<string>('read_text_file', { path });
}

export async function getLlmConfig(): Promise<LlmConfig> {
  return invoke<LlmConfig>('get_llm_config');
}

export async function saveLlmConfig(config: LlmConfig): Promise<void> {
  return invoke('save_llm_config', { config });
}

export async function getRecentSessions(): Promise<{ ok: true; data: SessionInfo[] } | { ok: false; error: string }> {
  try {
    const data = await invoke<SessionInfo[]>('get_recent_sessions');
    return { ok: true, data };
  } catch (error) {
    return { ok: false, error: String(error) };
  }
}

export async function testConnection(): Promise<TestConnectionResult> {
  return invoke<TestConnectionResult>('test_connection');
}

export async function analyzeConversation(questionPrompt: string, conversation: string): Promise<AnalyzeResult> {
  return invoke<AnalyzeResult>('analyze_conversation', {
    req: { question_prompt: questionPrompt, conversation },
  });
}

export async function analyzeGsb(
  questionTitle: string,
  modelA: string,
  modelB: string,
  recordA: object,
  recordB: object
): Promise<{ winner: string; good: string; bad: string; note: string }> {
  return invoke('analyze_gsb', {
    req: { question_title: questionTitle, model_a: modelA, model_b: modelB, record_a: recordA, record_b: recordB },
  });
}

export async function githubGetUsername(): Promise<{ ok: true; login: string } | { ok: false; error: string }> {
  try {
    const login = await invoke<string>('github_get_username');
    return { ok: true, login };
  } catch (error) {
    return { ok: false, error: String(error) };
  }
}

export async function githubCreateRepo(
  name: string,
  description: string,
  isPrivate: boolean
): Promise<{ ok: true; repo: GithubRepo } | { ok: false; error: string }> {
  try {
    const repo = await invoke<GithubRepo>('github_create_repo', { name, description, private: isPrivate });
    return { ok: true, repo };
  } catch (error) {
    return { ok: false, error: String(error) };
  }
}

export async function githubCreateBranch(
  owner: string,
  repo: string,
  branch: string,
  fromBranch: string
): Promise<{ ok: true } | { ok: false; error: string }> {
  try {
    await invoke('github_create_branch', { owner, repo, branch, fromBranch });
    return { ok: true };
  } catch (error) {
    return { ok: false, error: String(error) };
  }
}

export async function githubCreatePr(
  owner: string,
  repo: string,
  head: string,
  title: string,
  body: string,
  base = 'main'
): Promise<{ ok: true; url: string; number: number } | { ok: false; error: string }> {
  try {
    const result = await invoke<{ url: string; number: number }>('github_create_pr', { owner, repo, head, title, body, base });
    return { ok: true, ...result };
  } catch (error) {
    return { ok: false, error: String(error) };
  }
}

export function generateId(): string {
  return `${Date.now().toString(36)}${Math.random().toString(36).slice(2, 7)}`;
}

export function nowIso(): string {
  return new Date().toISOString().slice(0, 19).replace('T', ' ');
}
