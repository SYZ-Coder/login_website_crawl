use std::error::Error;
use regex::Regex;
use log::{info, warn};

#[derive(Debug)]
pub struct CrawlRules {
    pub allowed_domains: Vec<String>,
    pub blocked_urls: Vec<String>,
    pub follow_rules: Vec<FollowRule>,
    pub extract_rules: Vec<ExtractRule>,
}

#[derive(Debug)]
pub struct FollowRule {
    pub pattern: String,
    pub action: FollowAction,
}

#[derive(Debug)]
pub enum FollowAction {
    Allow,
    Block,
    Conditional(Box<dyn Fn(&str) -> bool>),
}

#[derive(Debug)]
pub struct ExtractRule {
    pub selector: String,
    pub field: String,
    pub extractor: Box<dyn Fn(&str) -> Option<String>>,
}

impl CrawlRules {
    pub fn new() -> Self {
        CrawlRules {
            allowed_domains: Vec::new(),
            blocked_urls: Vec::new(),
            follow_rules: Vec::new(),
            extract_rules: Vec::new(),
        }
    }

    pub fn add_allowed_domain(&mut self, domain: String) {
        self.allowed_domains.push(domain);
        info!("Added allowed domain: {}", domain);
    }

    pub fn add_blocked_url(&mut self, url: String) {
        self.blocked_urls.push(url);
        info!("Added blocked URL: {}", url);
    }

    pub fn add_follow_rule(&mut self, pattern: String, action: FollowAction) {
        self.follow_rules.push(FollowRule { pattern, action });
        info!("Added follow rule: {}", pattern);
    }

    pub fn add_extract_rule(&mut self, selector: String, field: String, extractor: Box<dyn Fn(&str) -> Option<String>>) {
        self.extract_rules.push(ExtractRule { selector, field, extractor });
        info!("Added extract rule for field: {}", field);
    }

    pub fn should_follow(&self, url: &str) -> bool {
        // 检查域名是否允许
        if !self.is_domain_allowed(url) {
            return false;
        }
        
        // 检查URL是否被阻止
        if self.is_url_blocked(url) {
            return false;
        }
        
        // 检查跟随规则
        for rule in &self.follow_rules {
            if self.matches_pattern(url, &rule.pattern) {
                match &rule.action {
                    FollowAction::Allow => return true,
                    FollowAction::Block => return false,
                    FollowAction::Conditional(func) => return func(url),
                }
            }
        }
        
        // 默认允许
        true
    }

    pub fn extract_data(&self, content: &str) -> HashMap<String, String> {
        let mut result = HashMap::new();
        
        for rule in &self.extract_rules {
            if let Some(extracted) = (rule.extractor)(content) {
                result.insert(rule.field.clone(), extracted);
            }
        }
        
        result
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
        for blocked in &self.blocked_urls {
            if url.contains(blocked) {
                return true;
            }
        }
        
        false
    }

    fn matches_pattern(&self, url: &str, pattern: &str) -> bool {
        // 简单的字符串匹配，实际可以使用正则表达式
        url.contains(pattern)
    }
}

pub fn create_crawl_rules() -> CrawlRules {
    CrawlRules::new()
}