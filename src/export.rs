use std::error::Error;
use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};
use serde_json;
use csv::Writer;

#[derive(Serialize, Deserialize, Debug)]
pub struct CrawledData {
    pub url: String,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug)]
pub enum ExportFormat {
    JSON,
    CSV,
    Excel,
}

#[derive(Debug)]
pub struct DataExporter {
    format: ExportFormat,
    output_path: String,
}

impl DataExporter {
    pub fn new(format: ExportFormat, output_path: String) -> Self {
        DataExporter {
            format,
            output_path,
        }
    }

    pub fn export(&self, data: &[CrawledData]) -> Result<(), Box<dyn Error>> {
        match self.format {
            ExportFormat::JSON => self.export_json(data),
            ExportFormat::CSV => self.export_csv(data),
            ExportFormat::Excel => self.export_excel(data),
        }
    }

    fn export_json(&self, data: &[CrawledData]) -> Result<(), Box<dyn Error>> {
        let json_data = serde_json::to_string_pretty(data)?;
        let mut file = File::create(&self.output_path)?;
        file.write_all(json_data.as_bytes())?;
        Ok(())
    }

    fn export_csv(&self, data: &[CrawledData]) -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path(&self.output_path)?;
        wtr.write_record(&["url", "content", "timestamp"])?;
        
        for item in data {
            wtr.write_record(&[
                &item.url,
                &item.content,
                &item.timestamp,
            ])?;
        }
        
        wtr.flush()?;
        Ok(())
    }

    fn export_excel(&self, data: &[CrawledData]) -> Result<(), Box<dyn Error>> {
        // Excel导出需要额外的依赖，这里简化处理
        // 实际项目中可以使用calamine或rust_xlsxwriter等库
        self.export_csv(data) // 临时使用CSV作为替代
    }
}

pub fn create_exporter(format: ExportFormat, output_path: String) -> DataExporter {
    DataExporter::new(format, output_path)
}