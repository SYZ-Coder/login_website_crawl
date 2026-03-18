use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_factor: f32,
}

impl Default for RetryConfig {
    fn default() -> Self {
        RetryConfig {
            max_retries: 3,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
            backoff_factor: 2.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CrawlerConfig {
    pub max_depth: usize,
    pub concurrency: usize,
    pub request_timeout: Duration,
    pub user_agent: String,
    pub retry_config: RetryConfig,
}

impl Default for CrawlerConfig {
    fn default() -> Self {
        CrawlerConfig {
            max_depth: 3,
            concurrency: 4,
            request_timeout: Duration::from_secs(30),
            user_agent: "Mozilla/5.0 (compatible; WebCrawler/1.0; +https://example.com/bot)".to_string(),
            retry_config: RetryConfig::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoginConfig {
    pub max_retries: u32,
    pub timeout: Duration,
    pub retry_config: RetryConfig,
}

impl Default for LoginConfig {
    fn default() -> Self {
        LoginConfig {
            max_retries: 3,
            timeout: Duration::from_secs(30),
            retry_config: RetryConfig::default(),
        }
    }
}