use web_crawler::{login, crawler, database, config};
use std::error::Error;
use tokio;
use log::{info, warn};

#[tokio::test]
async fn test_login_flow() -> Result<(), Box<dyn Error>> {
    // 测试登录功能
    let base_url = "https://httpbin.org";
    let credentials = login::LoginCredentials {
        username: "test".to_string(),
        password: "test".to_string(),
        csrf_token: None,
    };
    
    let login_config = config::LoginConfig::default();
    let session = login::create_session_with_login(base_url, &credentials, &login_config).await?;
    assert!(session.get_cookie("session").is_some());
    info!("Login test passed");
    Ok(())
}

#[tokio::test]
async fn test_crawler_creation() -> Result<(), Box<dyn Error>> {
    // 测试爬虫创建
    let base_url = "https://httpbin.org";
    let credentials = login::LoginCredentials {
        username: "test".to_string(),
        password: "test".to_string(),
        csrf_token: None,
    };
    
    let login_config = config::LoginConfig::default();
    let session = login::create_session_with_login(base_url, &credentials, &login_config).await?;
    let crawler_config = config::CrawlerConfig::default();
    let _crawler = crawler::create_crawler_with_session(base_url, &session, &crawler_config).await?;
    info!("Crawler creation test passed");
    Ok(())
}

#[tokio::test]
async fn test_content_retrieval() -> Result<(), Box<dyn Error>> {
    // 测试内容获取
    let base_url = "https://httpbin.org";
    let credentials = login::LoginCredentials {
        username: "test".to_string(),
        password: "test".to_string(),
        csrf_token: None,
    };
    
    let login_config = config::LoginConfig::default();
    let session = login::create_session_with_login(base_url, &credentials, &login_config).await?;
    let crawler_config = config::CrawlerConfig::default();
    let mut crawler = crawler::create_crawler_with_session(base_url, &session, &crawler_config).await?;
    
    let content = crawler.get_content("https://httpbin.org/html").await?;
    assert!(!content.is_empty());
    info!("Content retrieval test passed");
    Ok(())
}

#[tokio::test]
async fn test_database_operations() -> Result<(), Box<dyn Error>> {
    // 测试数据库操作
    let database = database::create_mock_database();
    
    // 测试插入
    let url = "https://example.com/test".to_string();
    let content = "Test content".to_string();
    database.insert(url.clone(), content.clone())?;
    
    // 测试获取
    let data = database.get_all();
    assert_eq!(data.len(), 1);
    assert_eq!(data[0].url, url);
    assert_eq!(data[0].content, content);
    
    // 测试清空
    database.clear();
    assert_eq!(database.get_all().len(), 0);
    
    info!("Database operations test passed");
    Ok(())
}

#[tokio::test]
async fn test_config_defaults() -> Result<(), Box<dyn Error>> {
    // 测试配置默认值
    let login_config = config::LoginConfig::default();
    let crawler_config = config::CrawlerConfig::default();
    
    assert_eq!(login_config.max_retries, 3);
    assert_eq!(crawler_config.max_depth, 3);
    assert_eq!(crawler_config.concurrency, 4);
    
    info!("Config defaults test passed");
    Ok(())
}