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
pub struct DataFilter {
    filters: Vec<Box<dyn Filter>>,
}

pub trait Filter {
    fn name(&self) -> &str;
    fn apply(&self, data: &CrawledData) -> bool;
}

impl DataFilter {
    pub fn new() -> Self {
        DataFilter {
            filters: Vec::new(),
        }
    }

    pub fn add_filter(&mut self, filter: Box<dyn Filter>) {
        self.filters.push(filter);
        info!("Added filter: {}", filter.name());
    }

    pub fn filter_data(&self, data: &[CrawledData]) -> Vec<CrawledData> {
        let mut result = Vec::new();
        
        for item in data {
            if self.should_keep(item) {
                result.push(item.clone());
            } else {
                info!("Filtered out data: {}", item.url);
            }
        }
        
        result
    }

    fn should_keep(&self, data: &CrawledData) -> bool {
        for filter in &self.filters {
            if !filter.apply(data) {
                return false;
            }
        }
        true
    }
}

// 内容长度过滤器
pub struct ContentLengthFilter {
    min_length: usize,
    max_length: usize,
}

impl ContentLengthFilter {
    pub fn new(min_length: usize, max_length: usize) -> Self {
        ContentLengthFilter {
            min_length,
            max_length,
        }
    }
}

impl Filter for ContentLengthFilter {
    fn name(&self) -> &str {
        "ContentLengthFilter"
    }

    fn apply(&self, data: &CrawledData) -> bool {
        let length = data.content.len();
        length >= self.min_length && length <= self.max_length
    }
}

// URL过滤器
pub struct UrlFilter {
    allowed_domains: Vec<String>,
    blocked_patterns: Vec<String>,
}

impl UrlFilter {
    pub fn new(allowed_domains: Vec<String>, blocked_patterns: Vec<String>) -> Self {
        UrlFilter {
            allowed_domains,
            blocked_patterns,
        }
    }
}

impl Filter for UrlFilter {
    fn name(&self) -> &str {
        "UrlFilter"
    }

    fn apply(&self, data: &CrawledData) -> bool {
        // 检查域名是否允许
        if !self.is_domain_allowed(&data.url) {
            return false;
        }
        
        // 检查URL是否被阻止
        if self.is_url_blocked(&data.url) {
            return false;
        }
        
        true
    }

    fn is_domain_allowed(&self, url: &str) -> bool {
        if self.allowed_domains.is_empty() {
            return true;
        }
        
        for domain in &self.allowed_domains {
            if url.contains(domain) {
                return true;
            }
        }
        
        false
    }

    fn is_url_blocked(&self, url: &str) -> bool {
        for pattern in &self.blocked_patterns {
            if url.contains(pattern) {
                return true;
            }
        }
        
        false
    }
}

// 时间过滤器
pub struct TimeFilter {
    min_timestamp: String,
    max_timestamp: String,
}

impl TimeFilter {
    pub fn new(min_timestamp: String, max_timestamp: String) -> Self {
        TimeFilter {
            min_timestamp,
            max_timestamp,
        }
    }
}

impl Filter for TimeFilter {
    fn name(&self) -> &str {
        "TimeFilter"
    }

    fn apply(&self, data: &CrawledData) -> bool {
        // 简单的时间比较，实际项目中可以使用chrono等库
        data.timestamp >= self.min_timestamp && data.timestamp <= self.max_timestamp
    }
}

// 内容过滤器
pub struct ContentFilter {
    required_keywords: Vec<String>,
    blocked_keywords: Vec<String>,
}

impl ContentFilter {
    pub fn new(required_keywords: Vec<String>, blocked_keywords: Vec<String>) -> Self {
        ContentFilter {
            required_keywords,
            blocked_keywords,
        }
    }
}

impl Filter for ContentFilter {
    fn name(&self) -> &str {
        "ContentFilter"
    }

    fn apply(&self, data: &CrawledData) -> bool {
        // 检查必需的关键词
        for keyword in &self.required_keywords {
            if !data.content.contains(keyword) {
                return false;
            }
        }
        
        // 检查被阻止的关键词
        for keyword in &self.blocked_keywords {
            if data.content.contains(keyword) {
                return false;
            }
        }
        
        true
    }
}

pub fn create_filter() -> DataFilter {
    DataFilter::new()
}