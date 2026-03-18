use std::error::Error;
use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use log::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct CrawledData {
    pub url: String,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    pub requests_per_second: f64,
    pub average_response_time: String,
    pub success_rate: f64,
    pub error_count: u64,
    pub total_requests: u64,
    pub successful_requests: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub total_pages: usize,
    pub average_content_length: f64,
    pub most_common_domains: Vec<(String, usize)>,
    pub content_types: HashMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceReport {
    pub title: String,
    pub generated_at: String,
    pub summary: Summary,
    pub metrics: Metrics,
    pub analysis: AnalysisResult,
    pub data_samples: Vec<CrawledData>,
    pub validation_results: Vec<ValidationResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    pub target_website: String,
    pub crawl_depth: usize,
    pub total_time: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub url: String,
    pub is_valid: bool,
    pub errors: Vec<String>,
}

#[derive(Debug)]
pub struct ReportGenerator {
    data: Vec<CrawledData>,
    metrics: Metrics,
    analysis: AnalysisResult,
    validation_results: Vec<ValidationResult>,
}

impl ReportGenerator {
    pub fn new(data: Vec<CrawledData>, metrics: Metrics, analysis: AnalysisResult, validation_results: Vec<ValidationResult>) -> Self {
        ReportGenerator {
            data,
            metrics,
            analysis,
            validation_results,
        }
    }

    pub fn generate_html_report(&self) -> Result<String, Box<dyn Error>> {
        let html = self.render_html();
        Ok(html)
    }

    pub fn generate_pdf_report(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        // PDF生成需要额外的依赖，这里返回HTML作为替代
        let html = self.render_html();
        Ok(html.into_bytes())
    }

    pub fn save_report(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let html = self.render_html();
        let mut file = File::create(path)?;
        file.write_all(html.as_bytes())?;
        info!("Report saved to: {}", path);
        Ok(())
    }

    fn render_html(&self) -> String {
        let generated_at = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        format!(r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>爬取报告</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .container {{ max-width: 1200px; margin: 0 auto; }}
        .header {{ text-align: center; margin-bottom: 30px; }}
        .section {{ margin-bottom: 30px; padding: 20px; border: 1px solid #ddd; border-radius: 5px; }}
        .section-title {{ font-size: 1.2em; font-weight: bold; margin-bottom: 15px; }}
        .summary-table {{ width: 100%; border-collapse: collapse; }}
        .summary-table th, .summary-table td {{ padding: 8px; text-align: left; border-bottom: 1px solid #ddd; }}
        .summary-table th {{ background-color: #f2f2f2; }}
        .metrics-table {{ width: 100%; border-collapse: collapse; }}
        .metrics-table th, .metrics-table td {{ padding: 8px; text-align: left; border-bottom: 1px solid #ddd; }}
        .metrics-table th {{ background-color: #f2f2f2; }}
        .data-table {{ width: 100%; border-collapse: collapse; }}
        .data-table th, .data-table td {{ padding: 8px; text-align: left; border-bottom: 1px solid #ddd; }}
        .data-table th {{ background-color: #f2f2f2; }}
        .validation-table {{ width: 100%; border-collapse: collapse; }}
        .validation-table th, .validation-table td {{ padding: 8px; text-align: left; border-bottom: 1px solid #ddd; }}
        .validation-table th {{ background-color: #f2f2f2; }}
        .error {{ color: #d32f2f; }}
        .success {{ color: #388e3c; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>网站爬取报告</h1>
            <p>生成时间: {}</p>
        </div>

        <div class="section">
            <div class="section-title">报告摘要</div>
            <table class="summary-table">
                <tr>
                    <th>目标网站</th>
                    <td>{}</td>
                </tr>
                <tr>
                    <th>爬取深度</th>
                    <td>{}</td>
                </tr>
                <tr>
                    <th>总耗时</th>
                    <td>{}</td>
                </tr>
                <tr>
                    <th>状态</th>
                    <td class="{}">{}</td>
                </tr>
            </table>
        </div>

        <div class="section">
            <div class="section-title">性能指标</div>
            <table class="metrics-table">
                <tr>
                    <th>总请求数</th>
                    <td>{}</td>
                </tr>
                <tr>
                    <th>成功请求数</th>
                    <td>{}</td>
                </tr>
                <tr>
                    <th>失败请求数</th>
                    <td>{}</td>
                </tr>
                <tr>
                    <th>成功率</th>
                    <td>{:.2}%</td>
                </tr>
                <tr>
                    <th>平均响应时间</th>
                    <td>{}</td>
                </tr>
                <tr>
                    <th>每秒请求数</th>
                    <td>{}</td>
                </tr>
            </table>
        </div>

        <div class="section">
            <div class="section-title">数据分析</div>
            <table class="summary-table">
                <tr>
                    <th>总页面数</th>
                    <td>{}</td>
                </tr>
                <tr>
                    <th>平均内容长度</th>
                    <td>{}</td>
                </tr>
                <tr>
                    <th>最常见域名</th>
                    <td>{:?}</td>
                </tr>
                <tr>
                    <th>内容类型分布</th>
                    <td>{:?}</td>
                </tr>
            </table>
        </div>

        <div class="section">
            <div class="section-title">数据样本</div>
            <table class="data-table">
                <tr>
                    <th>URL</th>
                    <th>内容预览</th>
                    <th>时间戳</th>
                </tr>
                {}
            </table>
        </div>

        <div class="section">
            <div class="section-title">验证结果</div>
            <table class="validation-table">
                <tr>
                    <th>URL</th>
                    <th>状态</th>
                    <th>错误信息</th>
                </tr>
                {}
            </table>
        </div>
    </div>
</body>
</html>
"#,
            generated_at,
            "https://example.com", // 目标网站
            3, // 爬取深度
            "00:05:30", // 总耗时
            "success", "完成",
            self.metrics.total_requests,
            self.metrics.successful_requests,
            self.metrics.error_count,
            self.metrics.success_rate,
            self.metrics.average_response_time,
            self.metrics.requests_per_second,
            self.analysis.total_pages,
            self.analysis.average_content_length,
            self.analysis.most_common_domains,
            self.analysis.content_types,
            self.render_data_samples(),
            self.render_validation_results()
        )
    }

    fn render_data_samples(&self) -> String {
        let mut html = String::new();
        for (i, item) in self.data.iter().take(5).enumerate() {
            let content_preview = if item.content.len() > 100 {
                format!("{}...", &item.content[..100])
            } else {
                item.content.clone()
            };
            
            html.push_str(&format!(r#"
                <tr>
                    <td><a href="{}" target="_blank">{}</a></td>
                    <td>{}</td>
                    <td>{}</td>
                </tr>
            "#,
                item.url, item.url, content_preview, item.timestamp));
        }
        html
    }

    fn render_validation_results(&self) -> String {
        let mut html = String::new();
        for result in &self.validation_results {
            let status_class = if result.is_valid { "success" } else { "error" };
            let errors = result.errors.join(", ");
            
            html.push_str(&format!(r#"
                <tr>
                    <td>{}</td>
                    <td class="{}">{}</td>
                    <td>{}</td>
                </tr>
            "#,
                result.url, status_class, if result.is_valid { "有效" } else { "无效" }, errors));
        }
        html
    }
}

pub fn create_report_generator(data: Vec<CrawledData>, metrics: Metrics, analysis: AnalysisResult, validation_results: Vec<ValidationResult>) -> ReportGenerator {
    ReportGenerator::new(data, metrics, analysis, validation_results)
}