use spider::website::Website;
use spider::tokio::sync::Mutex;
use spider::reqwest::Client as SpiderClient;
use spider::configuration::Configuration;
use std::sync::Arc;
use reqwest::{Client, StatusCode};
use url::Url;
use log::{info, error, warn};
use std::error::Error;
use std::time::Duration;
use crate::config::{CrawlerConfig, RetryConfig};

pub struct AuthenticatedCrawler {
    website: Arc<Mutex<Website>>,
    client: Client,
    base_url: Url,
    config: CrawlerConfig,
}

impl AuthenticatedCrawler {
    pub async fn new(base_url: &str, session: &crate::login::Session, config: &CrawlerConfig) -> Result<Self, Box<dyn Error>> {
        let client = Client::builder()
            .cookie_store(true)
            .timeout(config.request_timeout)
            .build()?;
        
        // 复制会话的 cookies 到爬虫客户端
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(cookie) = session.get_cookie("session_id") {
            headers.insert(
                reqwest::header::COOKIE,
                reqwest::header::HeaderValue::from_str(&cookie)?,
            );
        }
        
        let spider_client = SpiderClient::builder()
            .with_client(client.clone())
            .build()?;
        
        let spider_config = Configuration::new(base_url)
            .with_user_agent(&config.user_agent)
            .with_concurrency(config.concurrency)
            .with_delay(1.0)
            .with_depth(config.max_depth);
        
        let website = Website::new(spider_config);
        website.clone().run(spider_client).await;
        
        Ok(AuthenticatedCrawler {
            website: Arc::new(Mutex::new(website)),
            client,
            base_url: Url::parse(base_url)?,
            config: config.clone(),
        })
    }

    pub async fn crawl(&self) -> Result<(), Box<dyn Error>> {
        let mut website = self.website.lock().await;
        website.crawl().await;
        
        info!("Crawling completed. Found {} pages", website.page_count());
        Ok(())
    }

    pub async fn get_pages(&self) -> Vec<String> {
        let website = self.website.lock().await;
        website.pages().iter()
            .map(|page| page.url().to_string())
            .collect()
    }

    pub async fn get_content(&self, url: &str) -> Result<String, Box<dyn Error>> {
        self.fetch_content_with_retry(url, &self.config.retry_config).await
    }

    async fn fetch_content_with_retry(&self, url: &str, retry_config: &RetryConfig) -> Result<String, Box<dyn Error>> {
        for attempt in 0..retry_config.max_retries {
            match self.try_fetch_content(url).await {
                Ok(content) => return Ok(content),
                Err(e) if attempt < retry_config.max_retries - 1 => {
                    let delay = std::cmp::min(
                        Duration::from_secs_f32(retry_config.initial_delay.as_secs_f32() * retry_config.backoff_factor.powi(attempt as i32)),
                        retry_config.max_delay
                    );
                    warn!("Fetch attempt {} failed for {}: {}. Retrying in {:?}...", attempt + 1, url, e, delay);
                    tokio::time::sleep(delay).await;
                }
                Err(e) => return Err(e),
            }
        }
        
        Err("Max fetch retries exceeded".into())
    }

    async fn try_fetch_content(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let response = self.client.get(url).send().await?;
        
        match response.status() {
            StatusCode::OK => Ok(response.text().await?),
            StatusCode::UNAUTHORIZED => Err("Authentication required or expired".into()),
            StatusCode::FORBIDDEN => Err("Access denied".into()),
            StatusCode::NOT_FOUND => Err("Page not found".into()),
            status => Err(format!("Failed to fetch content from {}: {}", url, status).into()),
        }
    }

    pub fn get_config(&self) -> &CrawlerConfig {
        &self.config
    }
}

pub async fn create_crawler_with_session(
    base_url: &str,
    session: &crate::login::Session,
    config: &CrawlerConfig,
) -> Result<AuthenticatedCrawler, Box<dyn Error>> {
    AuthenticatedCrawler::new(base_url, session, config).await
}