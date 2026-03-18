use reqwest::{CookieStore, Client, Url, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use crate::config::LoginConfig;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
}

#[derive(Debug)]
pub struct Session {
    client: Client,
    cookies: CookieStore,
    base_url: Url,
}

impl Session {
    pub async fn new(base_url: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::builder()
            .cookie_store(true)
            .timeout(Duration::from_secs(30))
            .build()?;
        
        let cookies = CookieStore::default();
        
        Ok(Session {
            client,
            cookies,
            base_url: Url::parse(base_url)?,
        })
    }

    pub async fn login(&self, credentials: &LoginCredentials, config: &LoginConfig) -> Result<(), Box<dyn Error>> {
        let login_url = self.base_url.join("login")?;
        
        let mut params = HashMap::new();
        params.insert("username", &credentials.username);
        params.insert("password", &credentials.password);
        
        if let Some(csrf_token) = &credentials.csrf_token {
            params.insert("csrf_token", csrf_token);
        }
        
        for attempt in 0..config.max_retries {
            match self.try_login(&login_url, &params).await {
                Ok(_) => return Ok(()),
                Err(e) if attempt < config.max_retries - 1 => {
                    let delay = std::cmp::min(
                        Duration::from_secs_f32(config.retry_config.initial_delay.as_secs_f32() * config.retry_config.backoff_factor.powi(attempt as i32)),
                        config.retry_config.max_delay
                    );
                    log::warn!("Login attempt {} failed: {}. Retrying in {:?}...", attempt + 1, e, delay);
                    tokio::time::sleep(delay).await;
                }
                Err(e) => return Err(e),
            }
        }
        
        Err("Max login retries exceeded".into())
    }

    async fn try_login(&self, login_url: &Url, params: &HashMap<&str, &str>) -> Result<(), Box<dyn Error>> {
        let response = self.client
            .post(login_url.clone())
            .form(params)
            .send()
            .await?;
        
        match response.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => Ok(()),
            StatusCode::UNAUTHORIZED => Err("Invalid credentials".into()),
            StatusCode::FORBIDDEN => Err("Access denied".into()),
            status => Err(format!("Login failed with status: {}", status).into()),
        }
    }

    pub fn get_cookie(&self, name: &str) -> Option<String> {
        self.cookies.cookies(&self.base_url).find_map(|cookie| {
            if cookie.name() == name {
                Some(cookie.value().to_string())
            } else {
                None
            }
        })
    }

    pub fn is_logged_in(&self) -> bool {
        self.get_cookie("session_id").is_some()
    }
}

pub async fn create_session_with_login(
    base_url: &str,
    credentials: &LoginCredentials,
    config: &LoginConfig,
) -> Result<Session, Box<dyn Error>> {
    let mut session = Session::new(base_url).await?;
    
    if !session.is_logged_in() {
        session.login(credentials, config).await?;
    }
    
    Ok(session)
}