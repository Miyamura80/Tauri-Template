use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    openai_api_key: Option<String>,
    #[serde(skip_serializing)]
    anthropic_api_key: Option<String>,
    #[serde(skip_serializing)]
    groq_api_key: Option<String>,
    #[serde(skip_serializing)]
    perplexity_api_key: Option<String>,
    #[serde(skip_serializing)]
    gemini_api_key: Option<String>,
}

impl AppConfig {
    pub fn openai_api_key(&self) -> Option<&str> {
        self.openai_api_key.as_deref()
    }
    pub fn anthropic_api_key(&self) -> Option<&str> {
        self.anthropic_api_key.as_deref()
    }
    pub fn groq_api_key(&self) -> Option<&str> {
        self.groq_api_key.as_deref()
    }
    pub fn perplexity_api_key(&self) -> Option<&str> {
        self.perplexity_api_key.as_deref()
    }
    pub fn gemini_api_key(&self) -> Option<&str> {
        self.gemini_api_key.as_deref()
    }
}

/// A sanitized version of the configuration intended for exposure to the frontend.
/// This strictly excludes sensitive information like API keys.
#[derive(Debug, Serialize, Deserialize)]
pub struct FrontendConfig {
    pub model_name: String,
    pub dot_global_config_health_check: bool,
    pub dev_env: String,
    pub example_parent: ExampleParent,
    pub default_llm: DefaultLlm,
    pub llm_config: LlmConfig,
    pub features: HashMap<String, bool>,
}

impl From<&AppConfig> for FrontendConfig {
    fn from(config: &AppConfig) -> Self {
        Self {
            model_name: config.model_name.clone(),
            dot_global_config_health_check: config.dot_global_config_health_check,
            dev_env: config.dev_env.clone(),
            example_parent: config.example_parent.clone(),
            default_llm: config.default_llm.clone(),
            llm_config: config.llm_config.clone(),
            features: config.features.clone(),
        }
    }
}

fn default_dev_env() -> String {
    "dev".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExampleParent {
    pub example_child: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DefaultLlm {
    pub default_model: String,
    pub fallback_model: Option<String>,
    pub default_temperature: f32,
    pub default_max_tokens: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LlmConfig {
    pub cache_enabled: bool,
    pub retry: RetryConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RetryConfig {
    pub max_attempts: i32,
    pub min_wait_seconds: i32,
    pub max_wait_seconds: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingConfig {
    pub verbose: bool,
    pub format: LoggingFormatConfig,
    pub levels: LoggingLevelsConfig,
    #[serde(default)]
    pub redaction: RedactionConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingFormatConfig {
    pub show_time: bool,
    pub show_session_id: bool,
    pub location: LoggingLocationConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingLevelsConfig {
    pub debug: bool,
    pub info: bool,
    pub warning: bool,
    pub error: bool,
    pub critical: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RedactionPattern {
    pub name: String,
    pub regex: String,
    pub placeholder: String,
}

static CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn get_config() -> &'static AppConfig {
    CONFIG.get_or_init(|| {
        load_config().unwrap_or_else(|e| panic!("Failed to load configuration: {}", e))
    })
}

fn load_config() -> Result<AppConfig, ConfigError> {
    let builder = Config::builder()
        // Load default config (mandatory)
        .add_source(File::with_name("src-tauri/global_config.yaml").required(true))
        // Load production config if in prod
        .add_source(File::with_name("src-tauri/production_config.yaml").required(false))
        // Load local override
        .add_source(File::with_name("src-tauri/.global_config.yaml").required(false))
        // Load environment variables
        // Map nested env vars like APP__LOGGING__VERBOSE=true
        .add_source(Environment::with_prefix("APP").separator("__"));

    builder.build()?.try_deserialize()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    struct EnvGuard(&'static str);
    impl EnvGuard {
        fn new(key: &'static str, val: &str) -> Self {
            env::set_var(key, val);
            Self(key)
        }
    }
    impl Drop for EnvGuard {
        fn drop(&mut self) {
            env::remove_var(self.0);
        }
    }

    #[test]
    #[serial]
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
    #[serial]
    fn test_env_var_override_precedence() {
        // YAML value is "gemini/gemini-3-flash-preview"
        let _guard = EnvGuard::new("APP__MODEL_NAME", "override-model");

        let config = load_config().expect("Should load config");
        assert_eq!(config.model_name, "override-model");
    }

    #[test]
    #[serial]
    fn test_type_coercion_boolean() {
        {
            let _guard = EnvGuard::new("APP__LLM_CONFIG__CACHE_ENABLED", "true");
            let config = load_config().expect("Should load config");
            assert_eq!(config.llm_config.cache_enabled, true);
        }

        {
            let _guard = EnvGuard::new("APP__LLM_CONFIG__CACHE_ENABLED", "false");
            let config = load_config().expect("Should load config");
            assert_eq!(config.llm_config.cache_enabled, false);
        }

        // Test boolean coercion from '1' and '0' (porting from Python tests)
        {
            let _guard = EnvGuard::new("APP__LOGGING__FORMAT__LOCATION__ENABLED", "1");
            let config = load_config().expect("Should load config");
            assert_eq!(config.logging.format.location.enabled, true);
        }

        {
            let _guard = EnvGuard::new("APP__LOGGING__FORMAT__LOCATION__ENABLED", "0");
            let config = load_config().expect("Should load config");
            assert_eq!(config.logging.format.location.enabled, false);
        }
    }

    #[test]
    #[serial]
    fn test_type_coercion_numeric() {
        let _guard1 = EnvGuard::new("APP__DEFAULT_LLM__DEFAULT_TEMPERATURE", "0.95");
        let _guard2 = EnvGuard::new("APP__LLM_CONFIG__RETRY__MAX_ATTEMPTS", "10");

        let config = load_config().expect("Should load config");
        assert_eq!(config.default_llm.default_temperature, 0.95);
        assert_eq!(config.llm_config.retry.max_attempts, 10);

        // Test float-to-int coercion if applicable (usually handled by serde)
        // Here we test string-to-numeric coercion specifically.
        let _guard3 = EnvGuard::new("APP__LLM_CONFIG__RETRY__MAX_ATTEMPTS", "5");
        let config = load_config().expect("Should load config");
        assert_eq!(config.llm_config.retry.max_attempts, 5);
    }

    #[test]
    #[serial]
    fn test_dev_env_override() {
        // Field name is lowercase `dev_env`
        let _guard = EnvGuard::new("APP__DEV_ENV", "production");
        let config = load_config().expect("Should load config");
        assert_eq!(config.dev_env, "production");
    }

    #[test]
    #[serial]
    fn test_api_key_loading() {
        // Test that API keys are loaded from environment variables (APP__ prefix)
        let _guard1 = EnvGuard::new("APP__OPENAI_API_KEY", "test-openai-key");
        let _guard2 = EnvGuard::new("APP__ANTHROPIC_API_KEY", "test-anthropic-key");

        let config = load_config().expect("Should load config");
        assert_eq!(config.openai_api_key(), Some("test-openai-key"));
        assert_eq!(config.anthropic_api_key(), Some("test-anthropic-key"));
    }

    #[test]
    #[serial]
    fn test_frontend_config_sanitization() {
        let config = AppConfig {
            model_name: "gpt-4".to_string(),
            dot_global_config_health_check: true,
            dev_env: "dev".to_string(),
            example_parent: ExampleParent {
                example_child: "val".to_string(),
            },
            default_llm: DefaultLlm {
                default_model: "gpt-4".to_string(),
                fallback_model: None,
                default_temperature: 0.7,
                default_max_tokens: 100,
            },
            llm_config: LlmConfig {
                cache_enabled: true,
                retry: RetryConfig {
                    max_attempts: 1,
                    min_wait_seconds: 1,
                    max_wait_seconds: 1,
                },
            },
            logging: LoggingConfig {
                verbose: true,
                format: LoggingFormatConfig {
                    show_time: true,
                    show_session_id: true,
                    location: LoggingLocationConfig {
                        enabled: true,
                        show_file: true,
                        show_function: true,
                        show_line: true,
                        show_for_info: true,
                        show_for_debug: true,
                        show_for_warning: true,
                        show_for_error: true,
                    },
                },
                levels: LoggingLevelsConfig {
                    debug: true,
                    info: true,
                    warning: true,
                    error: true,
                    critical: true,
                },
                redaction: RedactionConfig::default(),
            },
            features: HashMap::new(),
            openai_api_key: Some("secret-key".to_string()),
            anthropic_api_key: None,
            groq_api_key: None,
            perplexity_api_key: None,
            gemini_api_key: None,
        };

        let frontend_config = FrontendConfig::from(&config);
        let json = serde_json::to_string(&frontend_config).unwrap();

        assert!(!json.contains("secret-key"));
        assert!(!json.contains("openai_api_key"));
    }
}
