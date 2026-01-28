use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub model_name: String,
    pub dot_global_config_health_check: bool,
    #[serde(default = "default_dev_env")]
    pub dev_env: String,

    pub example_parent: ExampleParent,
    pub default_llm: DefaultLlm,
    pub llm_config: LlmConfig,
    pub logging: LoggingConfig,
    #[serde(default)]
    pub features: HashMap<String, bool>,

    // Environment variables (optional in config file, usually injected)
    #[serde(skip_serializing)]
    pub openai_api_key: Option<String>,
    #[serde(skip_serializing)]
    pub anthropic_api_key: Option<String>,
    #[serde(skip_serializing)]
    pub groq_api_key: Option<String>,
    #[serde(skip_serializing)]
    pub perplexity_api_key: Option<String>,
    #[serde(skip_serializing)]
    pub gemini_api_key: Option<String>,
}

fn default_dev_env() -> String {
    "dev".to_string()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExampleParent {
    pub example_child: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DefaultLlm {
    pub default_model: String,
    pub fallback_model: Option<String>,
    pub default_temperature: f32,
    pub default_max_tokens: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LlmConfig {
    pub cache_enabled: bool,
    pub retry: RetryConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RetryConfig {
    pub max_attempts: i32,
    pub min_wait_seconds: i32,
    pub max_wait_seconds: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub verbose: bool,
    pub format: LoggingFormatConfig,
    pub levels: LoggingLevelsConfig,
    #[serde(default)]
    pub redaction: RedactionConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoggingFormatConfig {
    pub show_time: bool,
    pub show_session_id: bool,
    pub location: LoggingLocationConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoggingLocationConfig {
    pub enabled: bool,
    pub show_file: bool,
    pub show_function: bool,
    pub show_line: bool,
    pub show_for_info: bool,
    pub show_for_debug: bool,
    pub show_for_warning: bool,
    pub show_for_error: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoggingLevelsConfig {
    pub debug: bool,
    pub info: bool,
    pub warning: bool,
    pub error: bool,
    pub critical: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RedactionConfig {
    #[serde(default = "true_default")]
    pub enabled: bool,
    #[serde(default = "true_default")]
    pub use_default_pii: bool,
    #[serde(default)]
    pub patterns: Vec<RedactionPattern>,
}

fn true_default() -> bool {
    true
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedactionPattern {
    pub name: String,
    pub regex: String,
    pub placeholder: String,
}

static CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn get_config() -> &'static AppConfig {
    CONFIG.get_or_init(|| load_config().expect("Failed to load configuration"))
}

fn load_config() -> Result<AppConfig, ConfigError> {
    let builder = Config::builder()
        // Load default config
        .add_source(File::with_name("global_config.yaml").required(false))
        .add_source(File::with_name("src-tauri/global_config.yaml").required(false))
        // Load production config if in prod
        .add_source(File::with_name("production_config.yaml").required(false))
        .add_source(File::with_name("src-tauri/production_config.yaml").required(false))
        // Load local override
        .add_source(File::with_name(".global_config.yaml").required(false))
        .add_source(File::with_name("src-tauri/.global_config.yaml").required(false))
        // Load environment variables
        .add_source(Environment::default());

    builder.build()?.try_deserialize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        // Ensure the config loads without error
        let config = load_config();
        assert!(config.is_ok(), "Failed to load config: {:?}", config.err());

        let config = config.unwrap();
        // Verify some default values from global_config.yaml
        assert_eq!(
            config.default_llm.default_model,
            "gemini/gemini-3-flash-preview"
        );
        assert_eq!(config.llm_config.retry.max_attempts, 3);
    }

    #[test]
    fn test_default_dev_env() {
        let config = load_config().unwrap();
        // assuming env var isn't set during this test, or defaults to what's in yaml/default
        // The struct default is "dev", but yaml says "dev"
        assert_eq!(config.dev_env, "dev");
    }
}
