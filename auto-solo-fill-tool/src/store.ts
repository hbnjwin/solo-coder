import { invoke } from '@tauri-apps/api/core';
import { AppData, LlmConfig } from './types';

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

export async function getLlmConfig(): Promise<LlmConfig> {
  return invoke<LlmConfig>('get_llm_config');
}

export async function saveLlmConfig(config: LlmConfig): Promise<void> {
  return invoke('save_llm_config', { config });
}

export async function testConnection(): Promise<TestConnectionResult> {
  return invoke<TestConnectionResult>('test_connection');
}

export async function getRecentSessions(): Promise<{ ok: true; data: SessionInfo[] } | { ok: false; error: string }> {
  try {
    const data = await invoke<SessionInfo[]>('get_recent_sessions');
    return { ok: true, data };
  } catch (error) {
    return { ok: false, error: String(error) };
  }
}

export async function aiQualityCheck(record: object): Promise<string> {
  return invoke<string>('ai_quality_check', { record });
}

export async function processAnalysis(userPrompt: string, logTrace: string, roundNumber: number): Promise<string> {
  return invoke<string>('process_analysis', { userPrompt, logTrace, roundNumber });
}

export async function generateUnsatisfiedReason(userPrompt: string, logTrace: string, isCompleted: string): Promise<string> {
  return invoke<string>('generate_unsatisfied_reason', { userPrompt, logTrace, isCompleted });
}

export async function openFileDialog(): Promise<string | null> {
  return invoke<string | null>('open_file_dialog');
}

export async function readTextFile(path: string): Promise<string> {
  return invoke<string>('read_text_file', { path });
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

export async function exportXlsx(): Promise<number[]> {
  return invoke<number[]>('export_xlsx');
}

export function generateId(): string {
  return `${Date.now().toString(36)}${Math.random().toString(36).slice(2, 7)}`;
}

export function nowIso(): string {
  return new Date().toISOString().slice(0, 19).replace('T', ' ');
}
