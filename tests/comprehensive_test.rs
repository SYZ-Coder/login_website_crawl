use std::error::Error;
use std::time::Duration;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use log::{info, warn};

// 测试数据结构
#[derive(Debug, Clone)]
struct TestCrawledData {
    pub url: String,
    pub content: String,
    pub timestamp: String,
}

// 测试工具函数
fn create_test_data() -> Vec<TestCrawledData> {
    vec![
        TestCrawledData {
            url: "https://example.com/page1".to_string(),
            content: "This is the first page content with some important keywords".to_string(),
            timestamp: "2023-01-01T12:00:00Z".to_string(),
        },
        TestCrawledData {
            url: "https://example.com/page2".to_string(),
            content: "Second page content with different information".to_string(),
            timestamp: "2023-01-01T12:01:00Z".to_string(),
        },
        TestCrawledData {
            url: "https://example.com/articles/2023/01/01".to_string(),
            content: "Article content with <html> tags and JSON data".to_string(),
            timestamp: "2023-01-01T12:02:00Z".to_string(),
        },
    ]
}

// 导出功能测试
#[test]
fn test_export_functionality() -> Result<(), Box<dyn Error>> {
    info!("Testing export functionality");
    
    let test_data = create_test_data();
    let test_data_ref: Vec<_> = test_data.iter().map(|d| d.clone()).collect();
    
    // 测试JSON导出
    let json_exporter = export::create_exporter(export::ExportFormat::JSON, "test_output.json".to_string());
    json_exporter.export(&test_data_ref)?;
    
    // 测试CSV导出
    let csv_exporter = export::create_exporter(export::ExportFormat::CSV, "test_output.csv".to_string());
    csv_exporter.export(&test_data_ref)?;
    
    // 测试Excel导出（简化为CSV）
    let excel_exporter = export::create_exporter(export::ExportFormat::Excel, "test_output.xlsx".to_string());
    excel_exporter.export(&test_data_ref)?;
    
    info!("Export functionality tests passed");
    Ok(())
}

// 调度任务功能测试
#[test]
fn test_scheduler_functionality() -> Result<(), Box<dyn Error>> {
    info!("Testing scheduler functionality");
    
    let scheduler = scheduler::create_scheduler();
    
    // 添加测试任务
    scheduler.add_task(scheduler::ScheduledTask {
        name: "test_daily_crawl".to_string(),
        url: "https://example.com".to_string(),
        schedule: "0 0 * * *".to_string(), // 每天午夜
        enabled: true,
        last_run: None,
    });
    
    // 获取任务列表
    let tasks = scheduler.get_tasks();
    assert!(!tasks.is_empty());
    assert_eq!(tasks[0].name, "test_daily_crawl");
    
    info!("Scheduler functionality tests passed");
    Ok(())
}

// 数据分析功能测试
#[test]
fn test_analysis_functionality() -> Result<(), Box<dyn Error>> {
    info!("Testing analysis functionality");
    
    let test_data = create_test_data();
    let analyzer = analysis::create_analyzer(test_data);
    
    // 测试内容分析
    let analysis_result = analyzer.analyze_content();
    assert_eq!(analysis_result.total_pages, 3);
    assert!(analysis_result.average_content_length > 0.0);
    assert!(!analysis_result.most_common_domains.is_empty());
    assert!(!analysis_result.content_types.is_empty());
    
    // 测试统计生成
    let stats = analyzer.generate_stats(100, 85, 15, 2.5);
    assert_eq!(stats.total_requests, 100);
    assert_eq!(stats.successful_requests, 85);
    assert_eq!(stats.failed_requests, 15);
    assert!(stats.success_rate > 0.0);
    assert!(stats.average_response_time.as_secs_f64() > 0.0);
    
    info!("Analysis functionality tests passed");
    Ok(())
}

// 爬取规则功能测试
#[test]
fn test_rules_functionality() -> Result<(), Box<dyn Error>> {
    info!("Testing rules functionality");
    
    let mut rules = rules::create_crawl_rules();
    
    // 添加允许的域名
    rules.add_allowed_domain("example.com".to_string());
    
    // 添加被阻止的URL
    rules.add_blocked_url("/admin/".to_string());
    
    // 添加跟随规则
    rules.add_follow_rule("/articles/".to_string(), rules::FollowAction::Allow);
    rules.add_follow_rule("/api/".to_string(), rules::FollowAction::Block);
    
    // 测试URL过滤
    assert!(rules.should_follow("https://example.com/page1"));
    assert!(!rules.should_follow("https://blocked.com/page1"));
    assert!(rules.should_follow("https://example.com/articles/2023/01/01"));
    assert!(!rules.should_follow("https://example.com/api/data"));
    
    info!("Rules functionality tests passed");
    Ok(())
}

// 性能监控功能测试
#[test]
fn test_monitor_functionality() -> Result<(), Box<dyn Error>> {
    info!("Testing monitor functionality");
    
    let monitor = monitor::create_monitor();
    
    // 记录一些请求
    monitor.record_request(Duration::from_secs(1), true);
    monitor.record_request(Duration::from_secs(2), true);
    monitor.record_request(Duration::from_secs(3), false);
    
    // 获取报告
    let report = monitor.get_report();
    assert_eq!(report.metrics.total_requests, 3);
    assert_eq!(report.metrics.successful_requests, 2);
    assert_eq!(report.metrics.error_count, 1);
    assert!(report.metrics.success_rate > 0.0);
    assert!(report.metrics.average_response_time.as_secs_f64() > 0.0);
    
    info!("Monitor functionality tests passed");
    Ok(())
}

// 数据过滤功能测试
#[test]
fn test_filter_functionality() -> Result<(), Box<dyn Error>> {
    info!("Testing filter functionality");
    
    let filter = filter::create_filter();
    
    // 添加内容长度过滤器
    filter.add_filter(Box::new(filter::ContentLengthFilter::new(10, 100)));
    
    // 添加URL过滤器
    filter.add_filter(Box::new(filter::UrlFilter::new(
        vec!["example.com".to_string()],
        vec!["/admin/".to_string()],
    )));
    
    // 添加内容过滤器
    filter.add_filter(Box::new(filter::ContentFilter::new(
        vec!["important".to_string()],
        vec!["blocked".to_string()],
    )));
    
    let test_data = create_test_data();
    let filtered_data = filter.filter_data(&test_data);
    
    // 验证过滤结果
    assert!(!filtered_data.is_empty());
    for item in &filtered_data {
        assert!(item.content.len() >= 10 && item.content.len() <= 100);
        assert!(item.url.contains("example.com"));
        assert!(!item.url.contains("/admin/"));
        assert!(item.content.contains("important"));
        assert!(!item.content.contains("blocked"));
    }
    
    info!("Filter functionality tests passed");
    Ok(())
}

// 数据验证功能测试
#[test]
fn test_validation_functionality() -> Result<(), Box<dyn Error>> {
    info!("Testing validation functionality");
    
    let validator = validation::create_validator();
    
    // 添加URL验证器
    validator.add_validator(Box::new(validation::UrlValidator::new(
        Some("https".to_string()),
        vec!["example.com".to_string()],
    )));
    
    // 添加内容验证器
    validator.add_validator(Box::new(validation::ContentValidator::new(
        Some(10),
        Some(100),
        vec!["important".to_string()],
        vec!["blocked".to_string()],
    )));
    
    // 添加时间戳验证器
    validator.add_validator(Box::new(validation::TimestampValidator::new(
        Some("%Y-%m-%dT%H:%M:%SZ".to_string()),
        Some("2023-01-01T00:00:00Z".to_string()),
        Some("2023-12-31T23:59:59Z".to_string()),
    )));
    
    let test_data = create_test_data();
    let validation_results = validator.validate_batch(&test_data);
    
    // 验证结果
    for result in &validation_results {
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }
    
    info!("Validation functionality tests passed");
    Ok(())
}

// 报告生成功能测试
#[test]
fn test_report_functionality() -> Result<(), Box<dyn Error>> {
    info!("Testing report functionality");
    
    let test_data = create_test_data();
    
    // 创建模拟的指标和分析结果
    let metrics = monitor::Metrics {
        requests_per_second: 10.5,
        average_response_time: Duration::from_secs(2),
        success_rate: 85.0,
        error_count: 15,
        total_requests: 100,
        successful_requests: 85,
    };
    
    let analysis_result = analysis::AnalysisResult {
        total_pages: 3,
        average_content_length: 50.0,
        most_common_domains: vec![("example.com".to_string(), 3)],
        content_types: HashMap::from([("text/html".to_string(), 2), ("application/json".to_string(), 1)]),
    };
    
    let validation_results = vec![
        validation::ValidationResult {
            url: "https://example.com/page1".to_string(),
            is_valid: true,
            errors: Vec::new(),
        },
        validation::ValidationResult {
            url: "https://example.com/page2".to_string(),
            is_valid: true,
            errors: Vec::new(),
        },
    ];
    
    let generator = report::create_report_generator(test_data, metrics, analysis_result, validation_results);
    
    // 测试HTML报告生成
    let html_report = generator.generate_html_report()?;
    assert!(!html_report.is_empty());
    assert!(html_report.contains("爬取报告"));
    
    // 测试报告保存
    generator.save_report("test_report.html")?;
    
    info!("Report functionality tests passed");
    Ok(())
}

// 集成测试：完整流程
#[test]
fn test_complete_workflow() -> Result<(), Box<dyn Error>> {
    info!("Testing complete workflow");
    
    // 1. 创建测试数据
    let test_data = create_test_data();
    
    // 2. 使用过滤器
    let filter = filter::create_filter();
    filter.add_filter(Box::new(filter::ContentLengthFilter::new(10, 100)));
    let filtered_data = filter.filter_data(&test_data);
    
    // 3. 使用验证器
    let validator = validation::create_validator();
    validator.add_validator(Box::new(validation::UrlValidator::new(
        Some("https".to_string()),
        vec!["example.com".to_string()],
    )));
    let validation_results = validator.validate_batch(&filtered_data);
    
    // 4. 使用分析器
    let analyzer = analysis::create_analyzer(filtered_data.clone());
    let analysis_result = analyzer.analyze_content();
    
    // 5. 使用监控器
    let monitor = monitor::create_monitor();
    monitor.record_request(Duration::from_secs(1), true);
    
    // 6. 使用报告生成器
    let metrics = monitor.get_metrics();
    let generator = report::create_report_generator(
        filtered_data,
        metrics,
        analysis_result,
        validation_results,
    );
    generator.save_report("workflow_report.html")?;
    
    info!("Complete workflow test passed");
    Ok(())
}