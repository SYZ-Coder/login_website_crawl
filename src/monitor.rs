use std::error::Error;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use log::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    pub requests_per_second: f64,
    pub average_response_time: Duration,
    pub success_rate: f64,
    pub error_count: u64,
    pub total_requests: u64,
    pub successful_requests: u64,
}

#[derive(Debug)]
pub struct PerformanceReport {
    pub metrics: Metrics,
    pub start_time: Instant,
    pub end_time: Instant,
}

#[derive(Debug)]
pub struct PerformanceMonitor {
    metrics: Arc<Mutex<Metrics>>,
    start_time: Instant,
    request_times: Arc<Mutex<Vec<Duration>>>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        PerformanceMonitor {
            metrics: Arc::new(Mutex::new(Metrics {
                requests_per_second: 0.0,
                average_response_time: Duration::from_secs(0),
                success_rate: 0.0,
                error_count: 0,
                total_requests: 0,
                successful_requests: 0,
            })),
            start_time: Instant::now(),
            request_times: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn record_request(&self, duration: Duration, success: bool) {
        let mut metrics = self.metrics.lock().unwrap();
        let mut request_times = self.request_times.lock().unwrap();
        
        metrics.total_requests += 1;
        if success {
            metrics.successful_requests += 1;
        } else {
            metrics.error_count += 1;
        }
        
        request_times.push(duration);
        
        // 更新平均响应时间
        let total_time: Duration = request_times.iter().sum();
        metrics.average_response_time = total_time / request_times.len() as u32;
        
        // 计算成功率
        if metrics.total_requests > 0 {
            metrics.success_rate = metrics.successful_requests as f64 / metrics.total_requests as f64 * 100.0;
        }
        
        // 计算每秒请求数（最近1分钟的平均）
        let now = Instant::now();
        let one_minute_ago = now - Duration::from_secs(60);
        let recent_requests: usize = request_times.iter()
            .filter(|t| self.start_time + **t >= one_minute_ago)
            .count();
        
        if now.duration_since(self.start_time).as_secs() >= 60 {
            metrics.requests_per_second = recent_requests as f64 / 60.0;
        }
    }

    pub fn get_report(&self) -> PerformanceReport {
        let metrics = self.metrics.lock().unwrap();
        let now = Instant::now();
        
        PerformanceReport {
            metrics: metrics.clone(),
            start_time: self.start_time,
            end_time: now,
        }
    }

    pub fn reset(&self) {
        let mut metrics = self.metrics.lock().unwrap();
        let mut request_times = self.request_times.lock().unwrap();
        
        *metrics = Metrics {
            requests_per_second: 0.0,
            average_response_time: Duration::from_secs(0),
            success_rate: 0.0,
            error_count: 0,
            total_requests: 0,
            successful_requests: 0,
        };
        
        request_times.clear();
        self.start_time = Instant::now();
    }

    pub fn get_metrics(&self) -> Metrics {
        self.metrics.lock().unwrap().clone()
    }
}

pub fn create_monitor() -> PerformanceMonitor {
    PerformanceMonitor::new()
}