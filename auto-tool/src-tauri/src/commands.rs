use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

// 鈹€鈹€ LLM config 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ModelConfig {
    pub value: String,
    pub label: String,
}

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
    #[serde(default)]
    pub models: Vec<ModelConfig>,
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
    if let Some(p) = path.parent() { fs::create_dir_all(p).map_err(|e| e.to_string())?; }
    fs::write(&path, serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}

// 鈹€鈹€ LLM analyze 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeRequest {
    pub question_prompt: String,
    pub conversation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeResult {
    pub rounds: Option<u32>,
    pub ux: u8,
    pub planning: u8,
    pub reasoning: u8,
    pub instruction: u8,
    pub engineering: u8,
    pub issue_types: Vec<String>,
    pub issue_desc: String,
    pub fix_cost: Option<String>,
    pub pros: String,
    pub analysis: String,
}

fn build_scoring_prompt(question_prompt: &str, conversation: &str) -> String {
    format!(r#"你是一名专业的 AI 编程助手评测员。请根据以下评分标准，对给定的对话过程进行客观评分。

## 评测题目（用户给模型的 Prompt）
```
{question_prompt}
```

## 模型对话过程

```
{conversation}
```

## 评分标准（5个维度，每项1-5分）

ux（用户体验满意度）：5=推理高效流畅，迅速抓住核心，输出可信；4=整体良好，偶有冗余不影响完成；3=重复冗余较多，偶尔复杂化简单问题；2=拖沓反复，用户需干预；1=死循环或无有效产出

planning（规划&执行反馈）：5=规划详细合理，主动建清单并实时更新，工具使用合理，遇歧义主动确认；4=规划合理，清单更新不够及时，工具略有缺失（约25%）；3=清单不完整或只在开头列出，工具缺失约50%；2=规划模糊，缺清单，状态断续；1=无规划无清单无工具

reasoning（理解/推理能力）：5=精炼整合全部上下文，最少代码更改，无需引导，1轮完成；4=整合上下文准确，偶有冗余，正确位置正确更改，无需引导，1轮完成；3=丢失部分细节，引导后可完成主线；2=误解上下文，引导后完成部分；1=完全误解，引导后也无法完成
【硬性约束】4分及以上要求无需人工引导且1轮完成。多轮引导不得高于3分。唯一例外：第二轮并非修正第一轮结果而是针对工程完备度修复，可给4-5分，但必须在 issue_desc 中说明原因。

instruction（复杂指令遵循）：5=完美遵循所有约束，后续修复不破坏前置要求；4=遗漏1-2个非核心约束，指出1次后全部纠正；3=忽略关键约束，2次警告后勉强调整；2=2次纠正后仍顾此失彼；1=无视约束，死循环或狡辩

engineering（工程完备度）：5=主动补测试，位置正确覆盖核心路径，风格一致；4=多数能补测试，覆盖略窄；3=提醒后才补，只覆盖基础路径；2=多次提醒后测试仍有明显问题；1=无测试意识或伪测试

## 问题类型（可多选）
幻觉、上下文丢失、指令遵循失败、死循环、偷懒、代码破坏、代码Bug、废话过多、输出中断、目标漂移/分心、其他

## 修复成本
- low：提醒1次立刻修正
- medium：反复说明2次或需贴报错日志才修好
- high：引导超2次未解决；或死循环；或必须自己写代码；或三轮对话都未修复

## 填写规则

rounds：统计用户发送消息次数。首轮卡住重试仍算第1轮；上下文过长继续也算第1轮；模型声称完成但实际未完成需继续则算新一轮。

当 ux ≠ 5：
- issue_types、issue_desc、fix_cost 必填
- issue_desc 覆盖所有非满分维度（不只是 <=3 的），每个维度不超过30字
- 每个 issue_type 必须有对应说明（选了代码破坏就要写具体破坏了哪里）
- 格式用中文冒号分条，示例：「代码bug：UserService类存在标红代码无法编译。复杂指令遵循：未遵循本地缓存要求，首次用了数据库，二次引导后修正。工程完备度：测试意识薄弱，仅对环境文件有校验。」

当 ux = 5：
- pros 必填，覆盖全部4个维度（含非5分的），每个维度15-25字概括打分原因
- 示例：「规划执行反馈：主动建清单并实时更新，工具调用完整。理解推理能力：整合上下文准确，思维链偶有冗余。复杂指令遵循：遗漏缓存约束，提醒后立刻修正。工程完备度：主动补单元测试，覆盖核心路径。」

通用规则：planning/reasoning/instruction/engineering 任意一项 <=3 时在 issue_desc 说明低分原因；任意一项 = 5 时在 pros 说明5分理由（即使 ux ≠ 5 也要写 pros）。ux ≠ 5 且四项均不为5时 pros 填空字符串。

## 输出要求

文风：禁止方括号标记，用中文冒号（代码破坏：xxx）；禁止 markdown 语法；禁止 emoji；禁止英文术语（第1轮/第2轮，绕过，清单）；analysis 不要以模型名开头。

严格按以下 JSON 输出，不要输出其他内容：

```json
{{
  "rounds": <正整数>,
  "ux": <1-5>,
  "planning": <1-5>,
  "reasoning": <1-5>,
  "instruction": <1-5>,
  "engineering": <1-5>,
  "issue_types": [<ux≠5时必填；ux=5且四项均>3时为空数组>],
  "issue_desc": "<ux≠5时必填，见规则；否则空字符串>",
  "fix_cost": <"low"/"medium"/"high"，ux≠5时必填；ux=5时为null>,
  "pros": "<见规则>",
  "analysis": "<100字以内综合评价>"
}}
```"#,
        question_prompt = question_prompt,
        conversation = conversation,
    )
}
#[tauri::command]
pub async fn analyze_conversation(
    app: tauri::AppHandle,
    req: AnalyzeRequest,
) -> Result<AnalyzeResult, String> {
    let cfg = load_llm_config(&app);
    if cfg.api_key.is_empty() {
        return Err("璇峰厛鍦ㄨ缃〉閰嶇疆 API Key".to_string());
    }

    let prompt = build_scoring_prompt(&req.question_prompt, &req.conversation);
    let client = reqwest::Client::new();

    let is_anthropic = cfg.provider == "anthropic";

    let (url, request) = match cfg.provider.as_str() {
        "azure" => {
            if cfg.base_url.is_empty() || cfg.azure_deployment.is_empty() {
                return Err("Azure 妯″紡闇€瑕佸～鍐?Endpoint 鍜?Deployment 鍚嶇О".to_string());
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
                "temperature": 0.1,
                "max_completion_tokens": 1024,
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
                return Err("Anthropic 妯″紡闇€瑕佸～鍐?Base URL".to_string());
            }
            let model = if cfg.model.is_empty() { "claude-sonnet-4-6".to_string() } else { cfg.model.clone() };
            let url = format!("{}/v1/messages", cfg.base_url.trim_end_matches('/'));
            let body = serde_json::json!({
                "model": model,
                "messages": [{ "role": "user", "content": prompt }],
                "temperature": 0.1,
                "max_tokens": 1024,
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
            // openai_compat (default)
            if cfg.base_url.is_empty() {
                return Err("璇峰厛鍦ㄨ缃〉閰嶇疆 Base URL".to_string());
            }
            let model = if cfg.model.is_empty() { "claude-sonnet-4-6".to_string() } else { cfg.model.clone() };
            let url = format!("{}/v1/chat/completions", cfg.base_url.trim_end_matches('/'));
            let body = serde_json::json!({
                "model": model,
                "messages": [{ "role": "user", "content": prompt }],
                "temperature": 0.1,
                "max_tokens": 1024,
            });
            let req = client
                .post(&url)
                .header("Authorization", format!("Bearer {}", cfg.api_key))
                .header("Content-Type", "application/json")
                .json(&body);
            (url, req)
        }
    };

    let resp = request.send().await.map_err(|e| format!("璇锋眰澶辫触: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("API 閿欒 {} ({}): {}", status, url, text));
    }

    let resp_json: serde_json::Value = resp.json().await
        .map_err(|e| format!("瑙ｆ瀽鍝嶅簲澶辫触: {}", e))?;

    // Anthropic: content[0].text  /  OpenAI: choices[0].message.content
    let content = if is_anthropic {
        resp_json["content"][0]["text"]
            .as_str()
            .ok_or("Anthropic 鍝嶅簲鏍煎紡寮傚父锛屾湭鎵惧埌 content[0].text")?
    } else {
        resp_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("鍝嶅簲鏍煎紡寮傚父锛屾湭鎵惧埌 choices[0].message.content")?
    };

    let json_str = if let Some(start) = content.find("```json") {
        let s = &content[start + 7..];
        let end = s.find("```").unwrap_or(s.len());
        s[..end].trim()
    } else if let Some(start) = content.find('{') {
        let end = content.rfind('}').map(|i| i + 1).unwrap_or(content.len());
        &content[start..end]
    } else {
        content.trim()
    };

    let cleaned: String = json_str.chars().map(|c| match c {
        '\n' | '\r' | '\t' => ' ',
        c if (c as u32) < 0x20 => ' ',
        c => c,
    }).collect();
    let json_str = cleaned.as_str();

    let result: AnalyzeResult = serde_json::from_str(json_str)
        .map_err(|e| format!("瑙ｆ瀽璇勫垎缁撴灉澶辫触: {}\n鍘熷鍐呭: {}", e, json_str))?;

    Ok(result)
}

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
        return TestConnectionResult { ok: false, message: "未配置 API Key".into(), model_reply: String::new() };
    }
    if cfg.base_url.is_empty() {
        return TestConnectionResult { ok: false, message: "未配置 Base URL / Endpoint".into(), model_reply: String::new() };
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap();

    let test_msg = "请只回复“连接成功”四个字，不要输出其他内容。";
    let is_anthropic = cfg.provider == "anthropic";

    let (url, request) = match cfg.provider.as_str() {
        "azure" => {
            if cfg.azure_deployment.is_empty() {
                return TestConnectionResult { ok: false, message: "未配置 Azure Deployment 名称".into(), model_reply: String::new() };
            }
            let api_version = if cfg.azure_api_version.is_empty() { "2025-04-01-preview".to_string() } else { cfg.azure_api_version.clone() };
            let url = format!("{}/openai/deployments/{}/chat/completions?api-version={}", cfg.base_url.trim_end_matches('/'), cfg.azure_deployment, api_version);
            let body = serde_json::json!({ "messages": [{"role":"user","content": test_msg}], "max_completion_tokens": 100 });
            let req = client.post(&url).header("api-key", &cfg.api_key).header("Content-Type","application/json").json(&body);
            (url, req)
        }
        "anthropic" => {
            let model = if cfg.model.is_empty() { "claude-sonnet-4-6".to_string() } else { cfg.model.clone() };
            let url = format!("{}/v1/messages", cfg.base_url.trim_end_matches('/'));
            let body = serde_json::json!({ "model": model, "messages": [{"role":"user","content": test_msg}], "max_tokens": 20 });
            let req = client.post(&url).header("x-api-key", &cfg.api_key).header("anthropic-version","2023-06-01").header("Content-Type","application/json").json(&body);
            (url, req)
        }
        _ => {
            let model = if cfg.model.is_empty() { "claude-sonnet-4-6".to_string() } else { cfg.model.clone() };
            let url = format!("{}/v1/chat/completions", cfg.base_url.trim_end_matches('/'));
            let body = serde_json::json!({ "model": model, "messages": [{"role":"user","content": test_msg}], "max_tokens": 20 });
            let req = client.post(&url).header("Authorization", format!("Bearer {}", cfg.api_key)).header("Content-Type","application/json").json(&body);
            (url, req)
        }
    };

    let resp = match request.send().await {
        Ok(r) => r,
        Err(e) => return TestConnectionResult { ok: false, message: format!("网络请求失败: {}", e), model_reply: String::new() },
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
        Err(e) => return TestConnectionResult { ok: false, message: format!("响应解析失败: {}", e), model_reply: body_text.chars().take(200).collect() },
    };

    let reply = if is_anthropic {
        json["content"][0]["text"].as_str().unwrap_or("").to_string()
    } else {
        json["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string()
    };

    let finish_reason = json["choices"][0]["finish_reason"].as_str().unwrap_or("");

    // treat any successful HTTP 200 with a choices array as connected
    if json["choices"].is_array() || json["content"].is_array() {
        let display = if reply.is_empty() {
            format!("(finish_reason={}，模型未输出文本，但接口已连通)", finish_reason)
        } else {
            reply
        };
        TestConnectionResult { ok: true, message: format!("连接成功（{}）", url), model_reply: display }
    } else {
        TestConnectionResult { ok: false, message: "响应结构异常，未找到 choices/content 字段".into(), model_reply: body_text.chars().take(300).collect() }
    }
}

// 鈹€鈹€ GSB analyze 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

#[derive(Debug, Serialize, Deserialize)]
pub struct GsbAnalyzeRequest {
    pub question_title: String,
    pub model_a: String,
    pub model_b: String,
    pub record_a: serde_json::Value,
    pub record_b: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GsbAnalyzeResult {
    pub winner: String,
    pub good: String,
    pub bad: String,
    pub note: String,
}

#[tauri::command]
pub async fn analyze_gsb(
    app: tauri::AppHandle,
    req: GsbAnalyzeRequest,
) -> Result<GsbAnalyzeResult, String> {
    let cfg = load_llm_config(&app);
    if cfg.api_key.is_empty() {
        return Err("请先在设置页配置 API Key".to_string());
    }

    let prompt = format!(r#"你是一名专业的 AI 编程助手评测员。请对以下两个模型在同一道题目上的表现进行 GSB 对比分析。

## 题目
{question_title}

## 模型 A：{model_a}
评分：
- 用户体验满意度：{ux_a}
- 规划&执行反馈：{planning_a}
- 理解/推理能力：{reasoning_a}
- 复杂指令遵循：{instruction_a}
- 工程完备度：{engineering_a}
- 总分：{total_a}/25

问题类型：{issues_a}
问题描述：{desc_a}
修复成本：{cost_a}
{pros_a_section}

## 模型 B：{model_b}
评分：
- 用户体验满意度：{ux_b}
- 规划&执行反馈：{planning_b}
- 理解/推理能力：{reasoning_b}
- 复杂指令遵循：{instruction_b}
- 工程完备度：{engineering_b}
- 总分：{total_b}/25

问题类型：{issues_b}
问题描述：{desc_b}
修复成本：{cost_b}
{pros_b_section}

## 任务
请判断哪个模型表现更好，并给出理由。

文风要求（所有文本字段必须遵守）：
- 禁止使用方括号标记，改用中文冒号格式
- 禁止使用 markdown 语法（加粗、反引号、列表符号等）
- 禁止使用 emoji
- 禁止使用英文术语，Round 1/2 写为第1轮/第2轮

输出要求：严格按以下 JSON 格式输出，不要输出其他内容：
```json
{{
  "winner": "<A 或 B 或 same>",
  "good": "<好的模型好在哪里，1-2句，必须结合该模型的【模型优点】字段来写>",
  "bad": "<差的模型差在哪里，1-2句，必须结合该模型的【问题类型】和【问题描述】字段来写>",
  "note": "<其他备注，可为空字符串>"
}}
```

winner：A 表示模型 A 更好，B 表示模型 B 更好，same 表示两者相当。
每个字段都必须是单行字符串，不要在字段内输出换行。"#,
        question_title = req.question_title,
        model_a = req.model_a,
        model_b = req.model_b,
        ux_a = req.record_a["scores"]["ux"].as_i64().unwrap_or(0),
        planning_a = req.record_a["scores"]["planning"].as_i64().unwrap_or(0),
        reasoning_a = req.record_a["scores"]["reasoning"].as_i64().unwrap_or(0),
        instruction_a = req.record_a["scores"]["instruction"].as_i64().unwrap_or(0),
        engineering_a = req.record_a["scores"]["engineering"].as_i64().unwrap_or(0),
        total_a = req.record_a["scores"]["ux"].as_i64().unwrap_or(0)
            + req.record_a["scores"]["planning"].as_i64().unwrap_or(0)
            + req.record_a["scores"]["reasoning"].as_i64().unwrap_or(0)
            + req.record_a["scores"]["instruction"].as_i64().unwrap_or(0)
            + req.record_a["scores"]["engineering"].as_i64().unwrap_or(0),
        issues_a = req.record_a["issue_types"].as_array().map(|v| v.iter().filter_map(|x| x.as_str()).collect::<Vec<_>>().join("、")).unwrap_or_default(),
        desc_a = req.record_a["issue_desc"].as_str().unwrap_or("无"),
        cost_a = req.record_a["fix_cost"].as_str().unwrap_or("无"),
        pros_a_section = if req.record_a["pros"].as_str().unwrap_or("").is_empty() { String::new() } else { format!("模型优点：{}", req.record_a["pros"].as_str().unwrap_or("")) },
        ux_b = req.record_b["scores"]["ux"].as_i64().unwrap_or(0),
        planning_b = req.record_b["scores"]["planning"].as_i64().unwrap_or(0),
        reasoning_b = req.record_b["scores"]["reasoning"].as_i64().unwrap_or(0),
        instruction_b = req.record_b["scores"]["instruction"].as_i64().unwrap_or(0),
        engineering_b = req.record_b["scores"]["engineering"].as_i64().unwrap_or(0),
        total_b = req.record_b["scores"]["ux"].as_i64().unwrap_or(0)
            + req.record_b["scores"]["planning"].as_i64().unwrap_or(0)
            + req.record_b["scores"]["reasoning"].as_i64().unwrap_or(0)
            + req.record_b["scores"]["instruction"].as_i64().unwrap_or(0)
            + req.record_b["scores"]["engineering"].as_i64().unwrap_or(0),
        issues_b = req.record_b["issue_types"].as_array().map(|v| v.iter().filter_map(|x| x.as_str()).collect::<Vec<_>>().join("、")).unwrap_or_default(),
        desc_b = req.record_b["issue_desc"].as_str().unwrap_or("无"),
        cost_b = req.record_b["fix_cost"].as_str().unwrap_or("无"),
        pros_b_section = if req.record_b["pros"].as_str().unwrap_or("").is_empty() { String::new() } else { format!("模型优点：{}", req.record_b["pros"].as_str().unwrap_or("")) },
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build().map_err(|e| e.to_string())?;

    let is_anthropic = cfg.provider == "anthropic";

    let (_url, request) = match cfg.provider.as_str() {
        "azure" => {
            let api_version = if cfg.azure_api_version.is_empty() { "2025-04-01-preview".to_string() } else { cfg.azure_api_version.clone() };
            let url = format!("{}/openai/deployments/{}/chat/completions?api-version={}", cfg.base_url.trim_end_matches('/'), cfg.azure_deployment, api_version);
            let body = serde_json::json!({ "messages": [{"role":"user","content": prompt}], "max_completion_tokens": 512, "temperature": 0.2 });
            let req = client.post(&url).header("api-key", &cfg.api_key).header("Content-Type","application/json").json(&body);
            (url, req)
        }
        "anthropic" => {
            let model = if cfg.model.is_empty() { "claude-sonnet-4-6".to_string() } else { cfg.model.clone() };
            let url = format!("{}/v1/messages", cfg.base_url.trim_end_matches('/'));
            let body = serde_json::json!({ "model": model, "messages": [{"role":"user","content": prompt}], "max_tokens": 512, "temperature": 0.2 });
            let req = client.post(&url).header("x-api-key", &cfg.api_key).header("anthropic-version","2023-06-01").header("Content-Type","application/json").json(&body);
            (url, req)
        }
        _ => {
            let model = if cfg.model.is_empty() { "claude-sonnet-4-6".to_string() } else { cfg.model.clone() };
            let url = format!("{}/v1/chat/completions", cfg.base_url.trim_end_matches('/'));
            let body = serde_json::json!({ "model": model, "messages": [{"role":"user","content": prompt}], "max_tokens": 512, "temperature": 0.2 });
            let req = client.post(&url).header("Authorization", format!("Bearer {}", cfg.api_key)).header("Content-Type","application/json").json(&body);
            (url, req)
        }
    };

    let resp = request.send().await.map_err(|e| format!("请求失败: {}", e))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("API 错误 {}: {}", status, text));
    }

    let resp_json: serde_json::Value = resp.json().await.map_err(|e| format!("解析响应失败: {}", e))?;
    let content = if is_anthropic {
        resp_json["content"][0]["text"].as_str().ok_or("响应格式异常")?
    } else {
        resp_json["choices"][0]["message"]["content"].as_str().ok_or("响应格式异常")?
    };

    let json_str = if let Some(start) = content.find("```json") {
        let s = &content[start + 7..];
        s[..s.find("```").unwrap_or(s.len())].trim()
    } else if let Some(start) = content.find('{') {
        let end = content.rfind('}').map(|i| i + 1).unwrap_or(content.len());
        &content[start..end]
    } else { content.trim() };

    // 清理模型返回里可能夹带的控制字符，避免 JSON 解析失败。
    let cleaned: String = json_str.chars().map(|c| match c {
        '\n' | '\r' => ' ',
        '\t' => ' ',
        c if (c as u32) < 0x20 => ' ',
        c => c,
    }).collect();
    let json_str = cleaned.as_str();

    let result: GsbAnalyzeResult = serde_json::from_str(json_str)
        .map_err(|e| format!("解析结果失败: {}\n原始内容: {}", e, json_str))?;
    Ok(result)
}

// 鈹€鈹€ Trae CN Session ID reader 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub last_seen: String,
}

#[tauri::command]
pub fn get_recent_sessions() -> Result<Vec<SessionInfo>, String> {
    let appdata = std::env::var("APPDATA").map_err(|e| format!("APPDATA 鐜鍙橀噺璇诲彇澶辫触: {}", e))?;
    let log_dir = PathBuf::from(&appdata).join("Trae CN").join("logs");
    if !log_dir.exists() {
        return Err(format!("鏃ュ織鐩綍涓嶅瓨鍦? {}", log_dir.display()));
    }

    let mut dirs: Vec<PathBuf> = fs::read_dir(&log_dir)
        .map_err(|e| format!("璇诲彇鏃ュ織鐩綍澶辫触: {}", e))?
        .filter_map(|e| e.ok()).map(|e| e.path())
        .filter(|p| p.is_dir() && p.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with("2026") || n.starts_with("2025"))
            .unwrap_or(false))
        .collect();
    dirs.sort();
    let latest = dirs.last().ok_or("鏃ュ織鐩綍涓虹┖")?.clone();

    let modular = latest.join("Modular");
    if !modular.exists() {
        return Err(format!("Modular 鐩綍涓嶅瓨鍦? {}", modular.display()));
    }

    let log_file = fs::read_dir(&modular)
        .map_err(|e| format!("璇诲彇 Modular 鐩綍澶辫触: {}", e))?
        .filter_map(|e| e.ok()).map(|e| e.path())
        .find(|p| p.file_name().and_then(|n| n.to_str())
            .map(|n| n.starts_with("ai-agent_") && n.ends_with("_stdout.log"))
            .unwrap_or(false));

    let log_file = log_file.ok_or_else(|| format!("鏈壘鍒?ai-agent stdout 鏃ュ織锛岀洰褰? {}", modular.display()))?;

    let content = fs::read_to_string(&log_file)
        .map_err(|e| format!("璇诲彇鏃ュ織鏂囦欢澶辫触 {}: {}", log_file.display(), e))?;

    let mut seen: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for line in content.lines() {
        if let Some(sid_pos) = line.find("session_id=") {
            let sid = &line[sid_pos + 11..];
            let end = sid.find(|c: char| !c.is_ascii_hexdigit()).unwrap_or(sid.len());
            let sid = &sid[..end];
            if sid.len() == 24 {
                let ts = line.get(..32).unwrap_or("").to_string();
                seen.insert(sid.to_string(), ts);
            }
        }
    }

    if seen.is_empty() {
        return Err(format!("鏃ュ織鏂囦欢涓湭鎵惧埌 session_id锛屾枃浠? {}", log_file.display()));
    }

    let mut results: Vec<SessionInfo> = seen.into_iter()
        .map(|(session_id, last_seen)| SessionInfo { session_id, last_seen })
        .collect();
    results.sort_by(|a, b| b.last_seen.cmp(&a.last_seen));
    results.truncate(10);
    Ok(results)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppData {
    pub questions: Vec<Question>,
    pub records: Vec<TestRecord>,
    pub gsb_records: Vec<GsbRecord>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: String,
    pub title: String,
    pub category: String,
    pub difficulty: u8,
    pub prompt: String,
    pub repo_url: String,
    pub repo_desc: String,
    pub tech_stack: String,
    #[serde(default)]
    pub task_direction: String,
    #[serde(default)]
    pub question_direction: String,
    #[serde(default)]
    pub github_repo_name: String,
    #[serde(default)]
    pub github_owner: String,
    #[serde(default)]
    pub github_ssh_url: String,
    #[serde(default = "default_branch_name")]
    pub default_branch: String,
    #[serde(default)]
    pub branches: Vec<String>,
    #[serde(default)]
    pub is_current_focus: bool,
    #[serde(default)]
    pub requires_long_task_eval: bool,
    #[serde(default)]
    pub requires_frontend_eval: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scores {
    pub ux: u8,
    pub planning: u8,
    pub reasoning: u8,
    pub instruction: u8,
    pub engineering: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestRecord {
    pub id: String,
    pub question_id: String,
    pub model: String,
    #[serde(default)]
    pub branch_name: String,
    pub session_id: String,
    pub pr_url: String,
    #[serde(default = "default_rounds")]
    pub interaction_rounds: u8,
    pub scores: Scores,
    #[serde(default)]
    pub long_task_score: Option<u8>,
    #[serde(default)]
    pub frontend_3d_score: Option<u8>,
    #[serde(default)]
    pub frontend_aesthetic_score: Option<u8>,
    pub issue_types: Vec<String>,
    pub issue_desc: String,
    pub fix_cost: Option<String>,
    pub pros: String,
    #[serde(default)]
    pub analysis_summary: String,
    #[serde(default)]
    pub conversation_text: String,
    #[serde(default)]
    pub conversation_file: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GsbRecord {
    pub id: String,
    pub question_id: String,
    pub model_a: String,
    pub model_b: String,
    pub winner: String,
    #[serde(default)]
    pub good: String,
    #[serde(default)]
    pub bad: String,
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub reason: String,
    pub created_at: String,
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

#[tauri::command]
pub fn export_csv(app: tauri::AppHandle) -> Result<String, String> {
    let data = load_data(&app);
    let mut rows: Vec<String> = vec![
        "棰樼洰ID,棰樼洰鏍囬,妯″瀷,Session ID,PR閾炬帴,鐢ㄦ埛浣撻獙婊℃剰搴?瑙勫垝&鎵ц鍙嶉,鐞嗚В/鎺ㄧ悊鑳藉姏,澶嶆潅鎸囦护閬靛惊,宸ョ▼瀹屽搴?闂绫诲瀷,闂鎻忚堪,淇鎴愭湰,妯″瀷浼樼偣,鍒涘缓鏃堕棿".to_string()
    ];
    for r in &data.records {
        let q_title = data
            .questions
            .iter()
            .find(|q| q.id == r.question_id)
            .map(|q| q.title.clone())
            .unwrap_or_default();
        let issue_types = r.issue_types.join("|");
        let fix_cost = r.fix_cost.clone().unwrap_or_default();
        rows.push(format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            r.question_id,
            q_title,
            r.model,
            r.session_id,
            r.pr_url,
            r.scores.ux,
            r.scores.planning,
            r.scores.reasoning,
            r.scores.instruction,
            r.scores.engineering,
            issue_types,
            r.issue_desc,
            fix_cost,
            r.pros,
            r.created_at
        ));
    }
    Ok(rows.join("\n"))
}

#[tauri::command]
pub fn read_text_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_file_dialog() -> Option<String> {
    use std::process::Command;
    // Use PowerShell to show a file open dialog
    let output = Command::new("powershell")
        .args([
            "-NoProfile", "-NonInteractive", "-Command",
            r#"Add-Type -AssemblyName System.Windows.Forms; $d = New-Object System.Windows.Forms.OpenFileDialog; $d.Filter = 'Text files (*.txt;*.md;*.log)|*.txt;*.md;*.log|All files (*.*)|*.*'; $d.Title = '閫夋嫨瀵硅瘽璁板綍鏂囦欢'; if ($d.ShowDialog() -eq 'OK') { $d.FileName } else { '' }"#,
        ])
        .output()
        .ok()?;
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if path.is_empty() { None } else { Some(path) }
}

fn default_rounds() -> u8 { 1 }
fn default_branch_name() -> String { "main".to_string() }

fn default_data() -> AppData {
    AppData {
        questions: default_questions(),
        records: vec![],
        gsb_records: vec![],
    }
}

fn q(id: &str, title: &str, category: &str, difficulty: u8, prompt: &str, tech_stack: &str) -> Question {
    Question {
        id: id.into(),
        title: title.into(),
        category: category.into(),
        difficulty,
        prompt: prompt.into(),
        repo_url: String::new(),
        repo_desc: String::new(),
        tech_stack: tech_stack.into(),
        task_direction: String::new(),
        question_direction: String::new(),
        github_repo_name: String::new(),
        github_owner: String::new(),
        github_ssh_url: String::new(),
        default_branch: default_branch_name(),
        branches: vec![],
        is_current_focus: false,
        requires_long_task_eval: false,
        requires_frontend_eval: false,
    }
}

fn default_questions() -> Vec<Question> {
    vec![
        q("T01", "SMC 终端地址本查询接口封装", "SMC", 1,
            "实现 TypeScript 模块 smc-client.ts，封装终端地址本查询接口，补齐类型定义、错误处理和基础测试。",
            "TypeScript, Node.js"),
        q("T02", "会议模板查询与即时会议创建", "SMC", 2,
            "在现有客户端上扩展会议模板查询和即时会议创建能力，并补齐参数校验与异常处理。",
            "TypeScript, Vitest"),
        q("T03", "会议控制接口组合调用", "SMC", 2,
            "实现会议呼叫、挂断、主席设置和发言人设置，要求通过组合已有客户端能力完成。",
            "TypeScript"),
        q("T04", "会议状态订阅与告警通知处理", "SMC", 3,
            "实现轮询式会议状态监控器，支持状态变化事件、错误事件和定时清理。",
            "TypeScript, Vitest"),
        q("T05", "调试 SMC 接口 401 鉴权失败", "SMC", 2,
            "分析现有鉴权实现中的潜在问题，修复导致偶发 401 的缺陷，并给出原因说明。",
            "TypeScript"),
        q("T06", "信源列表查询与 RTSP 地址获取", "分布式", 1,
            "实现分布式视频系统的信源查询与 RTSP 地址获取，并提供在线信源聚合能力。",
            "TypeScript"),
        q("T07", "大屏布局模板查询与上屏控制", "分布式", 2,
            "实现布局模板查询、布局应用和生效等待逻辑，保证参数校验与超时处理完整。",
            "TypeScript"),
        q("T08", "矩阵通道查询与输出切换", "分布式", 3,
            "实现矩阵通道状态查询和输出切换，要求支持并发安全和重试策略。",
            "TypeScript, Vitest"),
        q("T09", "重构分布式 API 客户端", "分布式", 2,
            "消除重复请求逻辑，提炼公共请求层，保持现有对外接口签名不变。",
            "TypeScript"),
        q("T10", "调试 RTSP 断流重连机制", "分布式", 3,
            "实现流管理器的自动重连、手动断开和状态事件，补充测试验证退避策略。",
            "TypeScript, Vitest"),
        q("T11", "UDP 音频控制器通信层", "音频", 2,
            "实现基于 UDP 协议的音频控制器通信模块，处理请求响应匹配、超时和校验。",
            "TypeScript, Node.js"),
        q("T12", "音频路径切换与增益控制", "音频", 2,
            "实现音频增益、静音、路径切换和原子化会议音频配置能力。",
            "TypeScript"),
        q("T13", "调试 UDP 乱序导致的状态错误", "音频", 3,
            "修复并发命令和乱序响应下的状态错乱问题，并增加覆盖并发场景的测试。",
            "TypeScript, Vitest"),
        q("T14", "VDC 会议预览流接口封装", "VDC", 1,
            "实现会议预览状态查询、终端预览地址获取和预览流启动接口。",
            "TypeScript"),
        q("T15", "多会场预览状态聚合查询优化", "VDC", 3,
            "将串行预览查询改造为并发实现，控制并发上限，并补充性能与稳定性验证。",
            "TypeScript, Vitest"),
    ]
}
// GitHub API commands 

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
        .build().map_err(|e| e.to_string())?;
    let resp = client.get("https://api.github.com/user")
        .header("Authorization", format!("token {}", cfg.github_token))
        .header("User-Agent", "yingji-tool")
        .send().await.map_err(|e| format!("请求失败: {}", e))?;
    if !resp.status().is_success() {
        return Err(format!("GitHub API 错误 {}", resp.status()));
    }
    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    json["login"].as_str().map(|s| s.to_string())
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
        return Err("璇峰厛鍦ㄨ缃〉閰嶇疆 GitHub Token".to_string());
    }
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build().map_err(|e| e.to_string())?;
    let body = serde_json::json!({
        "name": name,
        "description": description,
        "private": private,
        "auto_init": true,
    });
    let resp = client.post("https://api.github.com/user/repos")
        .header("Authorization", format!("token {}", cfg.github_token))
       .header("User-Agent", "yingji-tool")
        .header("Accept", "application/vnd.github.v3+json")
        .json(&body)
        .send().await.map_err(|e| format!("璇锋眰澶辫触: {}", e))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("鍒涘缓浠撳簱澶辫触 {}: {}", status, text));
    }
    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    Ok(GithubRepo {
        full_name: json["full_name"].as_str().unwrap_or("").to_string(),
        html_url: json["html_url"].as_str().unwrap_or("").to_string(),
        ssh_url: json["ssh_url"].as_str().unwrap_or("").to_string(),
        default_branch: json["default_branch"].as_str().unwrap_or("main").to_string(),
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
        return Err("璇峰厛鍦ㄨ缃〉閰嶇疆 GitHub Token".to_string());
    }
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build().map_err(|e| e.to_string())?;

    // get SHA of from_branch
    let ref_url = format!("https://api.github.com/repos/{}/{}/git/ref/heads/{}", owner, repo, from_branch);
    let ref_resp = client.get(&ref_url)
        .header("Authorization", format!("token {}", cfg.github_token))
        .header("User-Agent", "yingji-tool")
        .send().await.map_err(|e| format!("鑾峰彇鍒嗘敮 SHA 澶辫触: {}", e))?;
    if !ref_resp.status().is_success() {
        return Err(format!("鑾峰彇鍒嗘敮 {} 澶辫触: {}", from_branch, ref_resp.status()));
    }
    let ref_json: serde_json::Value = ref_resp.json().await.map_err(|e| e.to_string())?;
    let sha = ref_json["object"]["sha"].as_str()
        .ok_or_else(|| "鏃犳硶鑾峰彇鍒嗘敮 SHA".to_string())?;

    // create new branch
    let create_url = format!("https://api.github.com/repos/{}/{}/git/refs", owner, repo);
    let body = serde_json::json!({
        "ref": format!("refs/heads/{}", branch),
        "sha": sha,
    });
    let resp = client.post(&create_url)
        .header("Authorization", format!("token {}", cfg.github_token))
        .header("User-Agent", "yingji-tool")
        .header("Accept", "application/vnd.github.v3+json")
        .json(&body)
        .send().await.map_err(|e| format!("璇锋眰澶辫触: {}", e))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("鍒涘缓鍒嗘敮澶辫触 {}: {}", status, text));
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrResult {
    pub url: String,
    pub number: u64,
}

#[tauri::command]
pub async fn github_create_pr(
    app: tauri::AppHandle,
    owner: String,
    repo: String,
    head: String,
    title: String,
    body: String,
    base: String,
) -> Result<PrResult, String> {
    let cfg = load_llm_config(&app);
    if cfg.github_token.is_empty() {
        return Err("璇峰厛鍦ㄨ缃〉閰嶇疆 GitHub Token".to_string());
    }
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build().map_err(|e| e.to_string())?;
    let url = format!("https://api.github.com/repos/{}/{}/pulls", owner, repo);
    let payload = serde_json::json!({
        "title": title,
        "head": head,
        "base": if base.trim().is_empty() { "main" } else { base.trim() },
        "body": body,
    });
    let resp = client.post(&url)
        .header("Authorization", format!("token {}", cfg.github_token))
        .header("User-Agent", "yingji-tool")
        .header("Accept", "application/vnd.github.v3+json")
        .json(&payload)
        .send().await.map_err(|e| format!("璇锋眰澶辫触: {}", e))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
            let list_url = format!(
                "https://api.github.com/repos/{}/{}/pulls?state=open&head={}:{}&base={}",
                owner,
                repo,
                owner,
                head,
                if base.trim().is_empty() { "main" } else { base.trim() }
            );
            let list_resp = client.get(&list_url)
                .header("Authorization", format!("token {}", cfg.github_token))
                .header("User-Agent", "yingji-tool")
                .header("Accept", "application/vnd.github.v3+json")
                .send().await.map_err(|e| format!("鏌ヨ宸叉湁 PR 澶辫触: {}", e))?;
            if list_resp.status().is_success() {
                let list_json: serde_json::Value = list_resp.json().await.map_err(|e| e.to_string())?;
                if let Some(first) = list_json.as_array().and_then(|items| items.first()) {
                    return Ok(PrResult {
                        url: first["html_url"].as_str().unwrap_or("").to_string(),
                        number: first["number"].as_u64().unwrap_or(0),
                    });
                }
            }
        }
        return Err(format!("鍒涘缓 PR 澶辫触 {}: {}", status, text));
    }
  let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    Ok(PrResult {
        url: json["html_url"].as_str().unwrap_or("").to_string(),
        number: json["number"].as_u64().unwrap_or(0),
    })
}

// 鈹€鈹€ xlsx export 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

#[tauri::command]
pub fn export_xlsx_question(app: tauri::AppHandle, question_id: String) -> Result<Vec<u8>, String> {
    let data = load_data(&app);
    let cfg = load_llm_config(&app);
    let filtered = AppData {
        questions: data.questions.iter().filter(|q| q.id == question_id).cloned().collect(),
        records: data.records.iter().filter(|r| r.question_id == question_id).cloned().collect(),
        gsb_records: data.gsb_records.iter().filter(|g| g.question_id == question_id).cloned().collect(),
    };
    build_xlsx_bytes(filtered, cfg)
}

#[tauri::command]
pub fn export_xlsx(app: tauri::AppHandle) -> Result<Vec<u8>, String> {
    let data = load_data(&app);
    let cfg = load_llm_config(&app);
    build_xlsx_bytes(data, cfg)
}

fn build_xlsx_bytes(data: AppData, cfg: LlmConfig) -> Result<Vec<u8>, String> {
    use rust_xlsxwriter::*;
    let configured_models: Vec<(String, String)> = if cfg.models.is_empty() {
        vec![
            ("kimi-k2.5".to_string(), "Kimi K2.5".to_string()),
            ("glm-5.0".to_string(), "GLM 5.0".to_string()),
            ("glm-5.1".to_string(), "GLM 5.1".to_string()),
            ("seed-2.0-pro".to_string(), "seed-2.0-pro-global-minimal".to_string()),
        ]
    } else {
        cfg.models.iter().map(|m| (m.value.clone(), m.label.clone())).collect()
    };

    let mut used_model_values: Vec<String> = vec![];
    for q in &data.questions {
        for branch in &q.branches {
            if !used_model_values.contains(branch) {
                used_model_values.push(branch.clone());
            }
        }
    }
    for r in &data.records {
        if !used_model_values.contains(&r.model) {
            used_model_values.push(r.model.clone());
        }
    }
    for g in &data.gsb_records {
        if !used_model_values.contains(&g.model_a) {
            used_model_values.push(g.model_a.clone());
        }
        if !used_model_values.contains(&g.model_b) {
            used_model_values.push(g.model_b.clone());
        }
    }

    let mut models: Vec<(String, String)> = configured_models
        .iter()
        .filter(|(value, _)| used_model_values.contains(value))
        .cloned()
        .collect();
    for value in used_model_values {
        if !models.iter().any(|(item, _)| item == &value) {
            models.push((value.clone(), value));
        }
    }

    let label_for_model = |value: &str| -> String {
        models.iter()
            .find(|(item, _)| item == value)
            .map(|(_, label)| label.clone())
            .or_else(|| configured_models.iter().find(|(item, _)| item == value).map(|(_, label)| label.clone()))
            .unwrap_or_else(|| value.to_string())
    };
    let model_index = |value: &str| -> usize {
        models.iter().position(|(item, _)| item == value).unwrap_or(usize::MAX)
    };
    let normalize_pair = |model_a: &str, model_b: &str| -> (String, String) {
        if model_index(model_a) <= model_index(model_b) {
            (model_a.to_string(), model_b.to_string())
        } else {
            (model_b.to_string(), model_a.to_string())
        }
    };

    let mut gsb_pairs: Vec<(String, String)> = vec![];
    for g in &data.gsb_records {
        if g.model_a == g.model_b {
            continue;
        }
        let pair = normalize_pair(&g.model_a, &g.model_b);
        if !gsb_pairs.contains(&pair) {
            gsb_pairs.push(pair);
        }
    }
    gsb_pairs.sort_by_key(|(model_a, model_b)| (model_index(model_a), model_index(model_b)));

    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.set_name("数据表").map_err(|e| e.to_string())?;

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

    let num_fmt = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter);

    let star_filled_fmt = Format::new()
        .set_font_color(Color::RGB(0xED7D31))
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter);

    let star_empty_fmt = Format::new()
        .set_font_color(Color::RGB(0xD9D9D9))
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter);

    let fixed_headers = vec![
        "prompt",
        "GitHub 原始 Repo 链接",
        "Repo 介绍",
        "Repo 使用的技术栈",
        "任务方向",
        "题目方向",
    ];
    let model_col_labels = vec![
        "Session ID",
        "GitHub PR链接",
        "交互轮次",
        "用户体验满意度",
        "问题类型",
        "问题描述",
        "修复成本",
        "模型优点",
        "规划&执行反馈",
        "理解/推理能力",
        "复杂指令遵循",
        "工程完备度",
        "长程任务",
        "前端 3D 产物",
        "前端产物美观度",
    ];

    let mut col: u16 = 0;
    for header in &fixed_headers {
        sheet.write_with_format(0, col, *header, &hdr_fmt).map_err(|e| e.to_string())?;
        col += 1;
    }

    let mut model_start_cols: Vec<u16> = vec![];
    for (_, label) in &models {
        model_start_cols.push(col);
        for label_prefix in &model_col_labels {
            let header = format!("{} ({})", label_prefix, label);
            sheet.write_with_format(0, col, header.as_str(), &hdr_fmt).map_err(|e| e.to_string())?;
            col += 1;
        }
    }

    for (model_a, model_b) in &gsb_pairs {
        let pair = format!("{} vs {}", label_for_model(model_a), label_for_model(model_b));
        sheet.write_with_format(0, col, pair.as_str(), &hdr_fmt).map_err(|e| e.to_string())?;
        sheet.write_with_format(0, col + 1, "好的模型好在啥", &hdr_fmt).map_err(|e| e.to_string())?;
        sheet.write_with_format(0, col + 2, "坏的模型坏在啥", &hdr_fmt).map_err(|e| e.to_string())?;
        sheet.write_with_format(0, col + 3, "其他备注", &hdr_fmt).map_err(|e| e.to_string())?;
        sheet.set_column_width(col, 18).map_err(|e| e.to_string())?;
        sheet.set_column_width(col + 1, 26).map_err(|e| e.to_string())?;
        sheet.set_column_width(col + 2, 26).map_err(|e| e.to_string())?;
        sheet.set_column_width(col + 3, 20).map_err(|e| e.to_string())?;
        col += 4;
    }

    sheet.set_column_width(0, 40).map_err(|e| e.to_string())?;
    sheet.set_column_width(1, 30).map_err(|e| e.to_string())?;
    sheet.set_column_width(2, 20).map_err(|e| e.to_string())?;
    sheet.set_column_width(3, 15).map_err(|e| e.to_string())?;
    sheet.set_column_width(4, 12).map_err(|e| e.to_string())?;
    sheet.set_column_width(5, 12).map_err(|e| e.to_string())?;
    for mc in &model_start_cols {
        sheet.set_column_width(*mc, 40).map_err(|e| e.to_string())?;
        sheet.set_column_width(*mc + 1, 30).map_err(|e| e.to_string())?;
        sheet.set_column_width(*mc + 2, 8).map_err(|e| e.to_string())?;
        sheet.set_column_width(*mc + 3, 8).map_err(|e| e.to_string())?;
        sheet.set_column_width(*mc + 4, 15).map_err(|e| e.to_string())?;
        sheet.set_column_width(*mc + 5, 30).map_err(|e| e.to_string())?;
        sheet.set_column_width(*mc + 6, 8).map_err(|e| e.to_string())?;
        sheet.set_column_width(*mc + 7, 30).map_err(|e| e.to_string())?;
        sheet.set_column_width(*mc + 8, 8).map_err(|e| e.to_string())?;
        sheet.set_column_width(*mc + 9, 8).map_err(|e| e.to_string())?;
        sheet.set_column_width(*mc + 10, 8).map_err(|e| e.to_string())?;
        sheet.set_column_width(*mc + 11, 8).map_err(|e| e.to_string())?;
        sheet.set_column_width(*mc + 12, 8).map_err(|e| e.to_string())?;
        sheet.set_column_width(*mc + 13, 8).map_err(|e| e.to_string())?;
        sheet.set_column_width(*mc + 14, 8).map_err(|e| e.to_string())?;
    }

    sheet.set_freeze_panes(1, 0).map_err(|e| e.to_string())?;

    let mut row: u32 = 1;
    for q in &data.questions {
        let mut c: u16 = 0;
        sheet.write_with_format(row, c, q.prompt.as_str(), &cell_fmt).map_err(|e| e.to_string())?;
        c += 1;
        sheet.write_with_format(row, c, q.repo_url.as_str(), &cell_fmt).map_err(|e| e.to_string())?;
        c += 1;
        sheet.write_with_format(row, c, q.repo_desc.as_str(), &cell_fmt).map_err(|e| e.to_string())?;
        c += 1;
        sheet.write_with_format(row, c, q.tech_stack.as_str(), &cell_fmt).map_err(|e| e.to_string())?;
        c += 1;
        sheet.write_with_format(row, c, q.task_direction.as_str(), &cell_fmt).map_err(|e| e.to_string())?;
        c += 1;
        sheet.write_with_format(row, c, q.question_direction.as_str(), &cell_fmt).map_err(|e| e.to_string())?;

        for (mi, (model_value, _)) in models.iter().enumerate() {
            let mc = model_start_cols[mi];
            let rec = data.records.iter().find(|r| r.question_id == q.id && &r.model == model_value);
            if let Some(r) = rec {
                let fix_cost_str = match r.fix_cost.as_deref() {
                    Some("low") => "低",
                    Some("medium") => "中",
                    Some("high") => "高",
                    _ => "",
                };
                sheet.write_with_format(row, mc, r.session_id.as_str(), &cell_fmt).map_err(|e| e.to_string())?;
                sheet.write_with_format(row, mc + 1, r.pr_url.as_str(), &cell_fmt).map_err(|e| e.to_string())?;
                sheet.write_with_format(row, mc + 2, r.interaction_rounds as i32, &num_fmt).map_err(|e| e.to_string())?;
                let filled = "★".repeat(r.scores.ux as usize);
                let empty = "☆".repeat(5 - r.scores.ux as usize);
                if empty.is_empty() {
                    sheet.write_string_with_format(row, mc + 3, &filled, &star_filled_fmt).map_err(|e| e.to_string())?;
                } else {
                    sheet.write_rich_string_with_format(row, mc + 3, &[
                        (&star_filled_fmt, &filled),
                        (&star_empty_fmt, &empty),
                    ], &cell_fmt).map_err(|e| e.to_string())?;
                }
                sheet.write_with_format(row, mc + 4, r.issue_types.join("、").as_str(), &cell_fmt).map_err(|e| e.to_string())?;
                sheet.write_with_format(row, mc + 5, r.issue_desc.as_str(), &cell_fmt).map_err(|e| e.to_string())?;
                sheet.write_with_format(row, mc + 6, fix_cost_str, &cell_fmt).map_err(|e| e.to_string())?;
                sheet.write_with_format(row, mc + 7, r.pros.as_str(), &cell_fmt).map_err(|e| e.to_string())?;
                sheet.write_with_format(row, mc + 8, r.scores.planning as i32, &num_fmt).map_err(|e| e.to_string())?;
                sheet.write_with_format(row, mc + 9, r.scores.reasoning as i32, &num_fmt).map_err(|e| e.to_string())?;
                sheet.write_with_format(row, mc + 10, r.scores.instruction as i32, &num_fmt).map_err(|e| e.to_string())?;
                sheet.write_with_format(row, mc + 11, r.scores.engineering as i32, &num_fmt).map_err(|e| e.to_string())?;
                if let Some(score) = r.long_task_score {
                    sheet.write_with_format(row, mc + 12, score as i32, &num_fmt).map_err(|e| e.to_string())?;
                } else {
                    sheet.write_with_format(row, mc + 12, "", &cell_fmt).map_err(|e| e.to_string())?;
                }
                if let Some(score) = r.frontend_3d_score {
                    sheet.write_with_format(row, mc + 13, score as i32, &num_fmt).map_err(|e| e.to_string())?;
                } else {
                    sheet.write_with_format(row, mc + 13, "", &cell_fmt).map_err(|e| e.to_string())?;
                }
                if let Some(score) = r.frontend_aesthetic_score {
                    sheet.write_with_format(row, mc + 14, score as i32, &num_fmt).map_err(|e| e.to_string())?;
                } else {
                    sheet.write_with_format(row, mc + 14, "", &cell_fmt).map_err(|e| e.to_string())?;
                }
            }
        }

        let mut gc = model_start_cols.last().map(|value| value + 15).unwrap_or(6);
        for (model_a, model_b) in &gsb_pairs {
            let gsb = data.gsb_records.iter().find(|g| {
                if g.question_id != q.id {
                    return false;
                }
                let pair = normalize_pair(&g.model_a, &g.model_b);
                &pair.0 == model_a && &pair.1 == model_b
            });
            if let Some(g) = gsb {
                let winner_str = match g.winner.as_str() {
                    "A" => format!("{} 胜", label_for_model(&g.model_a)),
                    "B" => format!("{} 胜", label_for_model(&g.model_b)),
                    _ => "Same".to_string(),
                };
                let parts: Vec<&str> = g.reason.splitn(3, "\n\n").collect();
                let good = if !g.good.is_empty() { g.good.as_str() } else { parts.get(0).copied().unwrap_or("") };
                let bad = if !g.bad.is_empty() { g.bad.as_str() } else { parts.get(1).copied().unwrap_or("") };
                let note = if !g.note.is_empty() { g.note.as_str() } else { parts.get(2).copied().unwrap_or("") };
                sheet.write_with_format(row, gc, winner_str.as_str(), &cell_fmt).map_err(|e| e.to_string())?;
                sheet.write_with_format(row, gc + 1, good, &cell_fmt).map_err(|e| e.to_string())?;
                sheet.write_with_format(row, gc + 2, bad, &cell_fmt).map_err(|e| e.to_string())?;
                sheet.write_with_format(row, gc + 3, note, &cell_fmt).map_err(|e| e.to_string())?;
            }
            gc += 4;
        }

        sheet.set_row_height(row, 60.0).map_err(|e| e.to_string())?;
        row += 1;
    }

    workbook.save_to_buffer().map_err(|e| e.to_string())
}


