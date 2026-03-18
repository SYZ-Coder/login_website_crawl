use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use log::{info, warn};

use crate::database::CrawledData;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    status: String,
    data: Vec<CrawledData>,
}

pub struct WebServer {
    database: Arc<crate::database::MockDatabase>,
}

impl WebServer {
    pub fn new(database: Arc<crate::database::MockDatabase>) -> Self {
        WebServer { database }
    }

    pub async fn start(self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let database = self.database.clone();
        
        // 获取所有数据的API端点
        let get_data = warp::path!("api" / "data")
            .and(warp::get())
            .map(move || {
                let data = database.get_all();
                let response = ApiResponse {
                    status: "success".to_string(),
                    data,
                };
                warp::reply::json(&response)
            });
        
        // 清空数据库的API端点
        let clear_data = warp::path!("api" / "clear")
            .and(warp::post())
            .map(move || {
                database.clear();
                warp::reply::json(&serde_json::json!({
                    "status": "success",
                    "message": "Database cleared"
                }))
            });
        
        // 主页
        let index = warp::path::end()
            .and(warp::get())
            .map(|| {
                warp::reply::html(include_str!("../web/index.html"))
            });
        
        let routes = get_data.or(clear_data).or(index);
        
        info!("Starting web server on {}", addr);
        warp::serve(routes)
            .run(([127, 0, 0, 1], 3030))
            .await;
        
        Ok(())
    }
}

pub fn create_web_server(database: Arc<crate::database::MockDatabase>) -> WebServer {
    WebServer::new(database)
}