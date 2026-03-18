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
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
}

#[derive(Debug)]
pub struct DataValidator {
    validators: Vec<Box<dyn Validator>>,
}

pub trait Validator {
    fn name(&self) -> &str;
    fn validate(&self, data: &CrawledData) -> ValidationResult;
}

impl DataValidator {
    pub fn new() -> Self {
        DataValidator {
            validators: Vec::new(),
        }
    }

    pub fn add_validator(&mut self, validator: Box<dyn Validator>) {
        self.validators.push(validator);
        info!("Added validator: {}", validator.name());
    }

    pub fn validate_data(&self, data: &CrawledData) -> ValidationResult {
        let mut errors = Vec::new();
        let mut is_valid = true;
        
        for validator in &self.validators {
            let result = validator.validate(data);
            if !result.is_valid {
                is_valid = false;
                errors.extend(result.errors);
            }
        }
        
        ValidationResult {
            is_valid,
            errors,
        }
    }

    pub fn validate_batch(&self, data: &[CrawledData]) -> Vec<ValidationResult> {
        data.iter().map(|d| self.validate_data(d)).collect()
    }
}

// URL验证器
pub struct UrlValidator {
    required_scheme: Option<String>,
    allowed_domains: Vec<String>,
}

impl UrlValidator {
    pub fn new(required_scheme: Option<String>, allowed_domains: Vec<String>) -> Self {
        UrlValidator {
            required_scheme,
            allowed_domains,
        }
    }
}

impl Validator for UrlValidator {
    fn name(&self) -> &str {
        "UrlValidator"
    }

    fn validate(&self, data: &CrawledData) -> ValidationResult {
        let mut errors = Vec::new();
        let mut is_valid = true;
        
        // 检查URL格式
        if data.url.is_empty() {
            errors.push("URL is empty".to_string());
            is_valid = false;
        }
        
        // 检查协议
        if let Some(scheme) = &self.required_scheme {
            if !data.url.starts_with(scheme) {
                errors.push(format!("URL must start with {}", scheme));
                is_valid = false;
            }
        }
        
        // 检查域名
        if !self.allowed_domains.is_empty() {
            let is_domain_allowed = self.allowed_domains.iter().any(|domain| data.url.contains(domain));
            if !is_domain_allowed {
                errors.push("URL domain not allowed".to_string());
                is_valid = false;
            }
        }
        
        ValidationResult {
            is_valid,
            errors,
        }
    }
}

// 内容验证器
pub struct ContentValidator {
    min_length: Option<usize>,
    max_length: Option<usize>,
    required_keywords: Vec<String>,
    blocked_keywords: Vec<String>,
}

impl ContentValidator {
    pub fn new(min_length: Option<usize>, max_length: Option<usize>, 
              required_keywords: Vec<String>, blocked_keywords: Vec<String>) -> Self {
        ContentValidator {
            min_length,
            max_length,
            required_keywords,
            blocked_keywords,
        }
    }
}

impl Validator for ContentValidator {
    fn name(&self) -> &str {
        "ContentValidator"
    }

    fn validate(&self, data: &CrawledData) -> ValidationResult {
        let mut errors = Vec::new();
        let mut is_valid = true;
        
        // 检查内容长度
        if let Some(min_length) = self.min_length {
            if data.content.len() < min_length {
                errors.push(format!("Content too short (min {} chars)", min_length));
                is_valid = false;
            }
        }
        
        if let Some(max_length) = self.max_length {
            if data.content.len() > max_length {
                errors.push(format!("Content too long (max {} chars)", max_length));
                is_valid = false;
            }
        }
        
        // 检查必需的关键词
        for keyword in &self.required_keywords {
            if !data.content.contains(keyword) {
                errors.push(format!("Missing required keyword: {}", keyword));
                is_valid = false;
            }
        }
        
        // 检查被阻止的关键词
        for keyword in &self.blocked_keywords {
            if data.content.contains(keyword) {
                errors.push(format!("Contains blocked keyword: {}", keyword));
                is_valid = false;
            }
        }
        
        ValidationResult {
            is_valid,
            errors,
        }
    }
}

// 时间戳验证器
pub struct TimestampValidator {
    required_format: Option<String>,
    min_timestamp: Option<String>,
    max_timestamp: Option<String>,
}

impl TimestampValidator {
    pub fn new(required_format: Option<String>, min_timestamp: Option<String>, max_timestamp: Option<String>) -> Self {
        TimestampValidator {
            required_format,
            min_timestamp,
            max_timestamp,
        }
    }
}

impl Validator for TimestampValidator {
    fn name(&self) -> &str {
        "TimestampValidator"
    }

    fn validate(&self, data: &CrawledData) -> ValidationResult {
        let mut errors = Vec::new();
        let mut is_valid = true;
        
        // 检查时间戳格式
        if let Some(format) = &self.required_format {
            // 简单的格式检查，实际项目中可以使用chrono等库
            if !data.timestamp.contains(format) {
                errors.push(format!("Timestamp format mismatch: expected {}", format));
                is_valid = false;
            }
        }
        
        // 检查最小时间戳
        if let Some(min_timestamp) = &self.min_timestamp {
            if data.timestamp < *min_timestamp {
                errors.push(format!("Timestamp too early: {}", data.timestamp));
                is_valid = false;
            }
        }
        
        // 检查最大时间戳
        if let Some(max_timestamp) = &self.max_timestamp {
            if data.timestamp > *max_timestamp {
                errors.push(format!("Timestamp too late: {}", data.timestamp));
                is_valid = false;
            }
        }
        
        ValidationResult {
            is_valid,
            errors,
        }
    }
}

// 综合验证器
pub struct CompositeValidator {
    validators: Vec<Box<dyn Validator>>,
}

impl CompositeValidator {
    pub fn new(validators: Vec<Box<dyn Validator>>) -> Self {
        CompositeValidator { validators }
    }
}

impl Validator for CompositeValidator {
    fn name(&self) -> &str {
        "CompositeValidator"
    }

    fn validate(&self, data: &CrawledData) -> ValidationResult {
        let mut errors = Vec::new();
        let mut is_valid = true;
        
        for validator in &self.validators {
            let result = validator.validate(data);
            if !result.is_valid {
                is_valid = false;
                errors.extend(result.errors);
            }
        }
        
        ValidationResult {
            is_valid,
            errors,
        }
    }
}

pub fn create_validator() -> DataValidator {
    DataValidator::new()
}