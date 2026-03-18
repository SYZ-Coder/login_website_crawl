use std::error::Error;
use std::collections::HashMap;
use log::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct CrawledData {
    pub url: String,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug)]
pub struct AnalysisResult {
    pub total_pages: usize,
    pub average_content_length: f64,
    pub most_common_domains: Vec<(String, usize)>,
    pub content_types: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct Statistics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub success_rate: f64,
}

#[derive(Debug)]
pub struct DataAnalyzer {
    data: Vec<CrawledData>,
}

impl DataAnalyzer {
    pub fn new(data: Vec<CrawledData>) -> Self {
        DataAnalyzer { data }
    }

    pub fn analyze_content(&self) -> AnalysisResult {
        let total_pages = self.data.len();
        
        let total_length: usize = self.data.iter()
            .map(|d| d.content.len())
            .sum();
        let average_content_length = if total_pages > 0 {
            total_length as f64 / total_pages as f64
        } else {
            0.0
        };
        
        let mut domain_counts = HashMap::new();
        for item in &self.data {
            if let Some(domain) = extract_domain(&item.url) {
                *domain_counts.entry(domain).or_insert(0) += 1;
            }
        }
        
        let mut sorted_domains: Vec<_> = domain_counts.into_iter().collect();
        sorted_domains.sort_by(|a, b| b.1.cmp(&a.1));
        let most_common_domains = sorted_domains.into_iter().take(5).collect();
        
        let mut content_types = HashMap::new();
        for item in &self.data {
            let content_type = detect_content_type(&item.content);
            *content_types.entry(content_type).or_insert(0) += 1;
        }
        
        AnalysisResult {
            total_pages,
            average_content_length,
            most_common_domains,
            content_types,
        }
    }

    pub fn generate_stats(&self, total_requests: u64, successful_requests: u64, failed_requests: u64, 
                        average_response_time: f64) -> Statistics {
        let success_rate = if total_requests > 0 {
            successful_requests as f64 / total_requests as f64 * 100.0
        } else {
            0.0
        };
        
        Statistics {
            total_requests,
            successful_requests,
            failed_requests,
            average_response_time,
            success_rate,
        }
    }

    pub fn get_data(&self) -> &[CrawledData] {
        &self.data
    }
}

fn extract_domain(url: &str) -> Option<String> {
    // 简单的域名提取逻辑
    if let Some(domain_start) = url.find("://") {
        let rest = &url[domain_start + 3..];
        if let Some(domain_end) = rest.find('/') {
            Some(rest[..domain_end].to_string())
        } else {
            Some(rest.to_string())
        }
    } else {
        None
    }
}

fn detect_content_type(content: &str) -> String {
    // 简单的内容类型检测
    if content.contains("<html") || content.contains("<body") {
        "text/html".to_string()
    } else if content.starts_with("{") && content.ends_with("}") {
        "application/json".to_string()
    } else if content.contains("DOCTYPE") {
        "text/xml".to_string()
    } else {
        "text/plain".to_string()
    }
}

pub fn create_analyzer(data: Vec<CrawledData>) -> DataAnalyzer {
    DataAnalyzer::new(data)
}