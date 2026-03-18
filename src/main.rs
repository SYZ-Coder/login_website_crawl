use web_crawler::{login, crawler, database, web_server, config};
use std::error::Error;
use tokio;
use log::{info, warn, error};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化日志
    env_logger::init();
    
    info!("Starting authenticated web crawler with enhanced error handling and retry mechanisms");
    
    // 加载配置
    let login_config = config::LoginConfig::default();
    let crawler_config = config::CrawlerConfig::default();
    
    // 配置目标网站和登录凭据
    let base_url = "https://httpbin.org"; // 使用 httpbin 作为测试网站
    let credentials = login::LoginCredentials {
        username: "test".to_string(),
        password: "test".to_string(),
        csrf_token: None,
    };
    
    // 创建模拟数据库
    let database = Arc::new(database::create_mock_database());
    info!("Mock database created");
    
    // 创建登录会话
    let session = match login::create_session_with_login(base_url, &credentials, &login_config).await {
        Ok(session) => {
            info!("Login successful");
            session
        }
        Err(e) => {
            error!("Failed to create session: {}", e);
            return Err(format!("Login failed: {}", e).into());
        }
    };
    
    // 验证登录状态
    if !session.is_logged_in() {
        error!("Session is not logged in after login attempt");
        return Err("Authentication failed".into());
    }
    
    // 创建爬虫
    let crawler = match crawler::create_crawler_with_session(base_url, &session, &crawler_config).await {
        Ok(crawler) => {
            info!("Crawler created with config: {:?}", crawler.get_config());
            crawler
        }
        Err(e) => {
            error!("Failed to create crawler: {}", e);
            return Err(format!("Crawler initialization failed: {}", e).into());
        }
    };
    
    // 开始爬取
    match crawler.crawl().await {
        Ok(_) => info!("Crawling completed"),
        Err(e) => {
            error!("Crawling failed: {}", e);
            return Err(format!("Crawling process failed: {}", e).into());
        }
    };
    
    // 获取页面并存储到数据库
    let pages = crawler.get_pages().await;
    info!("Found {} pages, storing to database...", pages.len());
    
    let mut success_count = 0;
    let mut failure_count = 0;
    
    for page in pages {
        match crawler.get_content(&page).await {
            Ok(content) => {
                if let Err(e) = database.insert(page.clone(), content) {
                    warn!("Failed to store data for {}: {}", page, e);
                    failure_count += 1;
                } else {
                    info!("Stored data for: {}", page);
                    success_count += 1;
                }
            }
            Err(e) => {
                warn!("Failed to fetch content for {}: {}", page, e);
                failure_count += 1;
            }
        }
    }
    
    info!("Data storage completed. Success: {}, Failures: {}", success_count, failure_count);
    
    // 启动Web服务器
    info!("Starting web server on http://127.0.0.1:3030");
    let server = web_server::create_web_server(database);
    
    match server.start("127.0.0.1:3030").await {
        Ok(_) => info!("Web server started successfully"),
        Err(e) => {
            error!("Failed to start web server: {}", e);
            return Err(format!("Web server startup failed: {}", e).into());
        }
    }
    
    Ok(())
}