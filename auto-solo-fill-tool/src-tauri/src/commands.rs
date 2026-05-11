use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

// ── LLM config ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LlmConfig {
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub base_url: String,
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub azure_deployment: String,
    #[serde(default)]
    pub azure_api_version: String,
    #[serde(default)]
    pub github_token: String,
    #[serde(default)]
    pub github_username: String,
}

fn llm_config_path(app: &tauri::AppHandle) -> PathBuf {
    app.path().app_data_dir().unwrap().join("llm_config.json")
}

fn load_llm_config(app: &tauri::AppHandle) -> LlmConfig {
    let path = llm_config_path(app);
    if path.exists() {
        let s = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&s).unwrap_or_default()
    } else {
        LlmConfig::default()
    }
}

#[tauri::command]
pub fn get_llm_config(app: tauri::AppHandle) -> LlmConfig {
    load_llm_config(&app)
}

#[tauri::command]
pub fn save_llm_config(app: tauri::AppHandle, config: LlmConfig) -> Result<(), String> {
    let path = llm_config_path(&app);
    if let Some(p) = path.parent() {
        fs::create_dir_all(p).map_err(|e| e.to_string())?;
    }
    fs::write(&path, serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}

// ── Data model ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: String,
    pub title: String,
    pub abbr: String,
    pub description: String,
    pub prompt: String,
    pub tech_stack: String,
    pub repo_url: String,
    pub github_repo_name: String,
    pub github_owner: String,
    pub github_ssh_url: String,
    #[serde(default = "default_branch_name")]
    pub default_branch: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskRecord {
    pub id: String,
    pub question_id: String,
    pub round_number: u8,
    pub session_id: String,
    pub user_prompt: String,
    pub task_type: String,
    pub business_domain: String,
    pub modify_scope: String,
    pub difficulty: String,
    pub is_completed: String,
    pub is_satisfied: String,
    #[serde(default)]
    pub unsatisfied_reason: String,
    pub github_url: String,
    #[serde(default)]
    pub screenshot_paths: String,
    #[serde(default)]
    pub log_trace: String,
    #[serde(default)]
    pub ai_quality_check_result: String,
    #[serde(default)]
    pub process_analysis_result: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppData {
    pub questions: Vec<Question>,
    pub records: Vec<TaskRecord>,
}

fn default_branch_name() -> String {
    "main".to_string()
}

fn default_data() -> AppData {
    AppData {
        questions: vec![],
        records: vec![],
    }
}

fn data_path(app: &tauri::AppHandle) -> PathBuf {
    app.path().app_data_dir().unwrap().join("data.json")
}

fn load_data(app: &tauri::AppHandle) -> AppData {
    let path = data_path(app);
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_else(|_| default_data())
    } else {
        default_data()
    }
}

fn save_data(app: &tauri::AppHandle, data: &AppData) -> Result<(), String> {
    let path = data_path(app);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let content = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn read_data(app: tauri::AppHandle) -> AppData {
    load_data(&app)
}

#[tauri::command]
pub fn write_data(app: tauri::AppHandle, data: AppData) -> Result<(), String> {
    save_data(&app, &data)
}

// ── File helpers ────────────────────────────────────────────────────────────

#[tauri::command]
pub fn read_text_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_file_dialog() -> Option<String> {
    use std::process::Command;
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            r#"Add-Type -AssemblyName System.Windows.Forms; $d = New-Object System.Windows.Forms.OpenFileDialog; $d.Filter = 'Text files (*.txt;*.md;*.log)|*.txt;*.md;*.log|All files (*.*)|*.*'; $d.Title = '选择文件'; if ($d.ShowDialog() -eq 'OK') { $d.FileName } else { '' }"#,
        ])
        .output()
        .ok()?;
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if path.is_empty() {
        None
    } else {
        Some(path)
    }
}

// ── Trae Session ID reader ──────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub last_seen: String,
}

#[tauri::command]
pub fn get_recent_sessions() -> Result<Vec<SessionInfo>, String> {
    let appdata = std::env::var("APPDATA")
        .map_err(|e| format!("APPDATA 环境变量读取失败: {}", e))?;
    let log_dir = PathBuf::from(&appdata).join("Trae CN").join("logs");
    if !log_dir.exists() {
        return Err(format!("日志目录不存在: {}", log_dir.display()));
    }

    let mut dirs: Vec<PathBuf> = fs::read_dir(&log_dir)
        .map_err(|e| format!("读取日志目录失败: {}", e))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.is_dir()
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with("2026") || n.starts_with("2025"))
                    .unwrap_or(false)
        })
        .collect();
    dirs.sort();
    let latest = dirs.last().ok_or("日志目录为空")?.clone();

    let modular = latest.join("Modular");
    if !modular.exists() {
        return Err(format!("Modular 目录不存在: {}", modular.display()));
    }

    let log_file = fs::read_dir(&modular)
        .map_err(|e| format!("读取 Modular 目录失败: {}", e))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with("ai-agent_") && n.ends_with("_stdout.log"))
                .unwrap_or(false)
        });

    let log_file =
        log_file.ok_or_else(|| format!("未找到 ai-agent stdout 日志，目录: {}", modular.display()))?;

    let content = fs::read_to_string(&log_file)
        .map_err(|e| format!("读取日志文件失败 {}: {}", log_file.display(), e))?;

    let mut seen: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for line in content.lines() {
        if let Some(sid_pos) = line.find("session_id=") {
            let sid = &line[sid_pos + 11..];
            let end = sid
                .find(|c: char| !c.is_ascii_hexdigit())
                .unwrap_or(sid.len());
            let sid = &sid[..end];
            if sid.len() == 24 {
                let ts = line.get(..32).unwrap_or("").to_string();
                seen.insert(sid.to_string(), ts);
            }
        }
    }

    if seen.is_empty() {
        return Err(format!(
            "日志文件中未找到 session_id，文件: {}",
            log_file.display()
        ));
    }

    let mut results: Vec<SessionInfo> = seen
        .into_iter()
        .map(|(session_id, last_seen)| SessionInfo {
            session_id,
            last_seen,
        })
        .collect();
    results.sort_by(|a, b| b.last_seen.cmp(&a.last_seen));
    results.truncate(10);
    Ok(results)
}

// ── LLM call helper ─────────────────────────────────────────────────────────

async fn call_llm(app: &tauri::AppHandle, prompt: String, max_tokens: u16) -> Result<String, String> {
    let cfg = load_llm_config(app);
    if cfg.api_key.is_empty() {
        return Err("请先在设置页配置 API Key".to_string());
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| e.to_string())?;

    let is_anthropic = cfg.provider == "anthropic";

    let (_url, request) = match cfg.provider.as_str() {
        "azure" => {
            if cfg.base_url.is_empty() || cfg.azure_deployment.is_empty() {
                return Err("Azure 模式需要填写 Endpoint 和 Deployment 名称".to_string());
            }
            let api_version = if cfg.azure_api_version.is_empty() {
                "2025-04-01-preview".to_string()
            } else {
                cfg.azure_api_version.clone()
            };
            let endpoint = cfg.base_url.trim_end_matches('/');
            let url = format!(
                "{}/openai/deployments/{}/chat/completions?api-version={}",
                endpoint, cfg.azure_deployment, api_version
            );
            let body = serde_json::json!({
                "messages": [{ "role": "user", "content": prompt }],
                "temperature": 0.3,
                "max_completion_tokens": max_tokens,
            });
            let req = client
                .post(&url)
                .header("api-key", &cfg.api_key)
                .header("Content-Type", "application/json")
                .json(&body);
            (url, req)
        }
        "anthropic" => {
            if cfg.base_url.is_empty() {
                return Err("Anthropic 模式需要填写 Base URL".to_string());
            }
            let model = if cfg.model.is_empty() {
                "claude-sonnet-4-6".to_string()
            } else {
                cfg.model.clone()
            };
            let url = format!("{}/v1/messages", cfg.base_url.trim_end_matches('/'));
            let body = serde_json::json!({
                "model": model,
                "messages": [{ "role": "user", "content": prompt }],
                "temperature": 0.3,
                "max_tokens": max_tokens,
            });
            let req = client
                .post(&url)
                .header("x-api-key", &cfg.api_key)
                .header("anthropic-version", "2023-06-01")
                .header("Content-Type", "application/json")
                .json(&body);
            (url, req)
        }
        _ => {
            if cfg.base_url.is_empty() {
                return Err("请先在设置页配置 Base URL".to_string());
            }
            let model = if cfg.model.is_empty() {
                "claude-sonnet-4-6".to_string()
            } else {
                cfg.model.clone()
            };
            let url = format!(
                "{}/v1/chat/completions",
                cfg.base_url.trim_end_matches('/')
            );
            let body = serde_json::json!({
                "model": model,
                "messages": [{ "role": "user", "content": prompt }],
                "temperature": 0.3,
                "max_tokens": max_tokens,
            });
            let req = client
                .post(&url)
                .header("Authorization", format!("Bearer {}", cfg.api_key))
                .header("Content-Type", "application/json")
                .json(&body);
            (url, req)
        }
    };

    let resp = request.send().await.map_err(|e| format!("请求失败: {}", e))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("API 错误 {} ({}): {}", status, _url, text));
    }

    let resp_json: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    let content = if is_anthropic {
        resp_json["content"][0]["text"]
            .as_str()
            .ok_or("Anthropic 响应格式异常，未找到 content[0].text")?
    } else {
        resp_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("响应格式异常，未找到 choices[0].message.content")?
    };

    Ok(content.to_string())
}

// ── AI 质检 ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn ai_quality_check(
    app: tauri::AppHandle,
    record: serde_json::Value,
) -> Result<String, String> {
    let prompt = format!(
        r#"你是一个数据质检员，请检查以下 Solo Coder 标注数据是否存在问题。

检查项：
1. User Prompt 是否像真人写的自然语言，而不是模板化的markdown格式描述
2. 不满意原因是否具体，是否包含范围对象、现象证据、与需求偏差、影响范围等要素中的至少两项
3. 任务类型、业务领域、修改范围、难度是否与 Prompt 内容匹配
4. 如果标记为满意但写了不满意原因，或者标记为不满意但没写原因，指出矛盾
5. 日志轨迹是否为空（不应为空）

标注数据：
- Session ID: {session_id}
- User Prompt: {user_prompt}
- 任务类型: {task_type}
- 业务领域: {business_domain}
- 修改范围: {modify_scope}
- 任务难度: {difficulty}
- 任务是否完成: {is_completed}
- 产物及过程是否满意: {is_satisfied}
- 不满意原因: {unsatisfied_reason}
- GitHub 地址: {github_url}

请用简洁的中文指出问题，每条一行，格式为：问题字段：问题描述。
如果没有问题，回复：数据质量良好，未发现问题。
注意语气要自然，像同事在审核，不要用markdown格式，不要用emoji，不要用英文术语。"#,
        session_id = record["session_id"].as_str().unwrap_or(""),
        user_prompt = record["user_prompt"].as_str().unwrap_or(""),
        task_type = record["task_type"].as_str().unwrap_or(""),
        business_domain = record["business_domain"].as_str().unwrap_or(""),
        modify_scope = record["modify_scope"].as_str().unwrap_or(""),
        difficulty = record["difficulty"].as_str().unwrap_or(""),
        is_completed = record["is_completed"].as_str().unwrap_or(""),
        is_satisfied = record["is_satisfied"].as_str().unwrap_or(""),
        unsatisfied_reason = record["unsatisfied_reason"].as_str().unwrap_or(""),
        github_url = record["github_url"].as_str().unwrap_or(""),
    );

    call_llm(&app, prompt, 512).await
}

// ── 过程分析 ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn process_analysis(
    app: tauri::AppHandle,
    user_prompt: String,
    log_trace: String,
    round_number: u8,
) -> Result<String, String> {
    let prompt = format!(
        r#"你是过程质检员。请基于输入信息判断本轮执行过程是否满意。

输入：
- User Prompt: {user_prompt}
- 轮次: 第{round_number}轮
- 日志轨迹: {log_trace}

判定目标：仅判断过程是否满意，不直接判断最终产物质量。

过程满意判定维度：
1. 需求理解与对齐：是否正确理解意图，是否有跑题误解
2. 过程规划与推进效率：是否有清晰步骤，是否无效循环重复
3. 工具使用与故障处理：工具调用是否必要合理，失败后是否有替代方案
4. 沟通体验：是否及时反馈，语气是否自然

如果不满意，列出原因，用中文冒号分条说明。

输出要求：只输出纯文本，不要用markdown格式，不要用emoji，不要用英文术语，语气要像有经验的同事在复盘，自然口语化，不要让人觉得是AI生成的。"#,
        user_prompt = user_prompt,
        round_number = round_number,
        log_trace = if log_trace.len() > 8000 {
            format!("（过长已截断）{}", &log_trace[..8000])
        } else {
            log_trace
        },
    );

    call_llm(&app, prompt, 1024).await
}

// ── 自动生成不满意原因 ──────────────────────────────────────────────────────

#[tauri::command]
pub async fn generate_unsatisfied_reason(
    app: tauri::AppHandle,
    user_prompt: String,
    log_trace: String,
    is_completed: String,
) -> Result<String, String> {
    let prompt = format!(
        r#"根据以下信息，帮我写一段不满意原因，要分产物不满意和过程不满意两部分。

信息：
- User Prompt: {user_prompt}
- 任务是否完成: {is_completed}
- 日志轨迹: {log_trace}

要求：
1. 产物不满意要写具体：在哪个页面或模块，做了什么操作，出现了什么现象，期望什么但实际什么，影响了什么
2. 过程不满意要写具体：哪里规划混乱，哪里重复操作，哪里沟通缺失
3. 语气要像真人写的，自然口语化，不要用markdown格式，不要用emoji，不要用方括号标记
4. 格式：产物不满意：xxx 过程不满意：xxx
5. 不要让人觉得是AI生成的，要像开发者在写bug报告"#,
        user_prompt = user_prompt,
        is_completed = is_completed,
        log_trace = if log_trace.len() > 6000 {
            format!("（过长已截断）{}", &log_trace[..6000])
        } else {
            log_trace
        },
    );

    call_llm(&app, prompt, 1024).await
}

// ── Test connection ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct TestConnectionResult {
    pub ok: bool,
    pub message: String,
    pub model_reply: String,
}

#[tauri::command]
pub async fn test_connection(app: tauri::AppHandle) -> TestConnectionResult {
    let cfg = load_llm_config(&app);

    if cfg.api_key.is_empty() {
        return TestConnectionResult {
            ok: false,
            message: "未配置 API Key".into(),
            model_reply: String::new(),
        };
    }
    if cfg.base_url.is_empty() {
        return TestConnectionResult {
            ok: false,
            message: "未配置 Base URL / Endpoint".into(),
            model_reply: String::new(),
        };
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap();

    let test_msg = "请只回复连接成功四个字，不要输出其他内容。";
    let is_anthropic = cfg.provider == "anthropic";

    let (url, request) = match cfg.provider.as_str() {
        "azure" => {
            if cfg.azure_deployment.is_empty() {
                return TestConnectionResult {
                    ok: false,
                    message: "未配置 Azure Deployment 名称".into(),
                    model_reply: String::new(),
                };
            }
            let api_version = if cfg.azure_api_version.is_empty() {
                "2025-04-01-preview".to_string()
            } else {
                cfg.azure_api_version.clone()
            };
            let url = format!(
                "{}/openai/deployments/{}/chat/completions?api-version={}",
                cfg.base_url.trim_end_matches('/'),
                cfg.azure_deployment,
                api_version
            );
            let body = serde_json::json!({ "messages": [{"role":"user","content": test_msg}], "max_completion_tokens": 100 });
            let req = client
                .post(&url)
                .header("api-key", &cfg.api_key)
                .header("Content-Type", "application/json")
                .json(&body);
            (url, req)
        }
        "anthropic" => {
            let model = if cfg.model.is_empty() {
                "claude-sonnet-4-6".to_string()
            } else {
                cfg.model.clone()
            };
            let url = format!("{}/v1/messages", cfg.base_url.trim_end_matches('/'));
            let body = serde_json::json!({ "model": model, "messages": [{"role":"user","content": test_msg}], "max_tokens": 20 });
            let req = client
                .post(&url)
                .header("x-api-key", &cfg.api_key)
                .header("anthropic-version", "2023-06-01")
                .header("Content-Type", "application/json")
                .json(&body);
            (url, req)
        }
        _ => {
            let model = if cfg.model.is_empty() {
                "claude-sonnet-4-6".to_string()
            } else {
                cfg.model.clone()
            };
            let url = format!(
                "{}/v1/chat/completions",
                cfg.base_url.trim_end_matches('/')
            );
            let body = serde_json::json!({ "model": model, "messages": [{"role":"user","content": test_msg}], "max_tokens": 20 });
            let req = client
                .post(&url)
                .header("Authorization", format!("Bearer {}", cfg.api_key))
                .header("Content-Type", "application/json")
                .json(&body);
            (url, req)
        }
    };

    let resp = match request.send().await {
        Ok(r) => r,
        Err(e) => {
            return TestConnectionResult {
                ok: false,
                message: format!("网络请求失败: {}", e),
                model_reply: String::new(),
            }
        }
    };

    let status = resp.status();
    let body_text = resp.text().await.unwrap_or_default();

    if !status.is_success() {
        return TestConnectionResult {
            ok: false,
            message: format!("HTTP {} - {}", status, url),
            model_reply: body_text.chars().take(300).collect(),
        };
    }

    let json: serde_json::Value = match serde_json::from_str(&body_text) {
        Ok(v) => v,
        Err(e) => {
            return TestConnectionResult {
                ok: false,
                message: format!("响应解析失败: {}", e),
                model_reply: body_text.chars().take(200).collect(),
            }
        }
    };

    let reply = if is_anthropic {
        json["content"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string()
    } else {
        json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string()
    };

    if json["choices"].is_array() || json["content"].is_array() {
        let display = if reply.is_empty() {
            format!("(模型未输出文本，但接口已连通)")
        } else {
            reply
        };
        TestConnectionResult {
            ok: true,
            message: format!("连接成功（{}）", url),
            model_reply: display,
        }
    } else {
        TestConnectionResult {
            ok: false,
            message: "响应结构异常，未找到 choices/content 字段".into(),
            model_reply: body_text.chars().take(300).collect(),
        }
    }
}

// ── GitHub API ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubRepo {
    pub full_name: String,
    pub html_url: String,
    pub ssh_url: String,
    pub default_branch: String,
}

#[tauri::command]
pub async fn github_get_username(app: tauri::AppHandle) -> Result<String, String> {
    let cfg = load_llm_config(&app);
    if cfg.github_token.is_empty() {
        return Err("请先在设置页配置 GitHub Token".to_string());
    }
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;
    let resp = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("token {}", cfg.github_token))
        .header("User-Agent", "solo-coder-fill-tool")
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;
    if !resp.status().is_success() {
        return Err(format!("GitHub API 错误 {}", resp.status()));
    }
    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    json["login"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "无法获取 GitHub 用户名".to_string())
}

#[tauri::command]
pub async fn github_create_repo(
    app: tauri::AppHandle,
    name: String,
    description: String,
    private: bool,
) -> Result<GithubRepo, String> {
    let cfg = load_llm_config(&app);
    if cfg.github_token.is_empty() {
        return Err("请先在设置页配置 GitHub Token".to_string());
    }
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?;
    let body = serde_json::json!({
        "name": name,
        "description": description,
        "private": private,
        "auto_init": true,
    });
    let resp = client
        .post("https://api.github.com/user/repos")
        .header("Authorization", format!("token {}", cfg.github_token))
        .header("User-Agent", "solo-coder-fill-tool")
        .header("Accept", "application/vnd.github.v3+json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("创建仓库失败 {}: {}", status, text));
    }
    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    Ok(GithubRepo {
        full_name: json["full_name"].as_str().unwrap_or("").to_string(),
        html_url: json["html_url"].as_str().unwrap_or("").to_string(),
        ssh_url: json["ssh_url"].as_str().unwrap_or("").to_string(),
        default_branch: json["default_branch"]
            .as_str()
            .unwrap_or("main")
            .to_string(),
    })
}

#[tauri::command]
pub async fn github_create_branch(
    app: tauri::AppHandle,
    owner: String,
    repo: String,
    branch: String,
    from_branch: String,
) -> Result<(), String> {
    let cfg = load_llm_config(&app);
    if cfg.github_token.is_empty() {
        return Err("请先在设置页配置 GitHub Token".to_string());
    }
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?;

    let ref_url = format!(
        "https://api.github.com/repos/{}/{}/git/ref/heads/{}",
        owner, repo, from_branch
    );
    let ref_resp = client
        .get(&ref_url)
        .header("Authorization", format!("token {}", cfg.github_token))
        .header("User-Agent", "solo-coder-fill-tool")
        .send()
        .await
        .map_err(|e| format!("获取分支 SHA 失败: {}", e))?;
    if !ref_resp.status().is_success() {
        return Err(format!(
            "获取分支 {} 失败: {}",
            from_branch,
            ref_resp.status()
        ));
    }
    let ref_json: serde_json::Value = ref_resp.json().await.map_err(|e| e.to_string())?;
    let sha = ref_json["object"]["sha"]
        .as_str()
        .ok_or_else(|| "无法获取分支 SHA".to_string())?;

    let create_url = format!(
        "https://api.github.com/repos/{}/{}/git/refs",
        owner, repo
    );
    let body = serde_json::json!({
        "ref": format!("refs/heads/{}", branch),
        "sha": sha,
    });
    let resp = client
        .post(&create_url)
        .header("Authorization", format!("token {}", cfg.github_token))
        .header("User-Agent", "solo-coder-fill-tool")
        .header("Accept", "application/vnd.github.v3+json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("创建分支失败 {}: {}", status, text));
    }
    Ok(())
}

// ── XLSX export ─────────────────────────────────────────────────────────────

#[tauri::command]
pub fn export_xlsx(app: tauri::AppHandle) -> Result<Vec<u8>, String> {
    let data = load_data(&app);
    build_xlsx_bytes(data)
}

fn build_xlsx_bytes(data: AppData) -> Result<Vec<u8>, String> {
    use rust_xlsxwriter::*;

    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet
        .set_name("交付数据")
        .map_err(|e| e.to_string())?;

    let hdr_fmt = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0xBDD7EE))
        .set_border(FormatBorder::Thin)
        .set_text_wrap()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter);

    let cell_fmt = Format::new()
        .set_border(FormatBorder::Thin)
        .set_text_wrap()
        .set_align(FormatAlign::VerticalCenter);

    let headers = vec![
        "题目名称",
        "英文缩写",
        "轮次",
        "Session ID",
        "User Prompt",
        "任务类型",
        "业务领域",
        "修改范围",
        "任务难度",
        "任务是否完成",
        "产物及过程是否满意",
        "不满意原因",
        "GitHub 地址",
        "截图",
        "日志轨迹",
        "AI 质检结果",
        "过程分析结果",
        "创建时间",
    ];

    let widths = vec![
        25u16, 12, 6, 30, 50, 12, 16, 14, 8, 12, 14, 40, 30, 20, 40, 30, 30, 18,
    ];

    for (col, header) in headers.iter().enumerate() {
        sheet
            .write_with_format(0, col as u16, *header, &hdr_fmt)
            .map_err(|e| e.to_string())?;
    }
    for (col, w) in widths.iter().enumerate() {
        sheet
            .set_column_width(col as u16, *w)
            .map_err(|e| e.to_string())?;
    }

    sheet
        .set_freeze_panes(1, 0)
        .map_err(|e| e.to_string())?;

    let mut row: u32 = 1;
    for r in &data.records {
        let q = data
            .questions
            .iter()
            .find(|q| q.id == r.question_id);
        let (title, abbr) = match q {
            Some(q) => (q.title.as_str(), q.abbr.as_str()),
            None => ("", ""),
        };

        let mut c: u16 = 0;
        let write_cell = |sheet: &mut Worksheet, row: u32, col: &mut u16, val: &str| -> Result<(), String> {
            sheet
                .write_with_format(row, *col, val, &cell_fmt)
                .map_err(|e| e.to_string())?;
            *col += 1;
            Ok(())
        };

        write_cell(sheet, row, &mut c, title)?;
        write_cell(sheet, row, &mut c, abbr)?;
        write_cell(sheet, row, &mut c, &r.round_number.to_string())?;
        write_cell(sheet, row, &mut c, &r.session_id)?;
        write_cell(sheet, row, &mut c, &r.user_prompt)?;
        write_cell(sheet, row, &mut c, &r.task_type)?;
        write_cell(sheet, row, &mut c, &r.business_domain)?;
        write_cell(sheet, row, &mut c, &r.modify_scope)?;
        write_cell(sheet, row, &mut c, &r.difficulty)?;
        write_cell(sheet, row, &mut c, &r.is_completed)?;
        write_cell(sheet, row, &mut c, &r.is_satisfied)?;
        write_cell(sheet, row, &mut c, &r.unsatisfied_reason)?;
        write_cell(sheet, row, &mut c, &r.github_url)?;
        write_cell(sheet, row, &mut c, &r.screenshot_paths)?;
        write_cell(sheet, row, &mut c, &r.log_trace)?;
        write_cell(sheet, row, &mut c, &r.ai_quality_check_result)?;
        write_cell(sheet, row, &mut c, &r.process_analysis_result)?;
        write_cell(sheet, row, &mut c, &r.created_at)?;

        sheet
            .set_row_height(row, 60.0)
            .map_err(|e| e.to_string())?;
        row += 1;
    }

    workbook.save_to_buffer().map_err(|e| e.to_string())
}
