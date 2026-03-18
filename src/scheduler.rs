use std::error::Error;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};
use cron::Schedule;
use chrono::{DateTime, Utc};
use log::{info, warn};

#[derive(Debug)]
pub struct ScheduledTask {
    pub name: String,
    pub url: String,
    pub schedule: String, // cron表达式
    pub enabled: bool,
    pub last_run: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct Scheduler {
    tasks: Arc<Mutex<Vec<ScheduledTask>>>,
    running: Arc<Mutex<bool>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            tasks: Arc::new(Mutex::new(Vec::new())),
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn add_task(&self, task: ScheduledTask) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(task);
        info!("Added task: {}", task.name);
    }

    pub fn remove_task(&self, name: &str) -> Option<ScheduledTask> {
        let mut tasks = self.tasks.lock().unwrap();
        let index = tasks.iter().position(|t| t.name == name)?;
        Some(tasks.remove(index))
    }

    pub fn enable_task(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(task) = tasks.iter_mut().find(|t| t.name == name) {
            task.enabled = true;
            info!("Enabled task: {}", name);
            Ok(())
        } else {
            Err(format!("Task not found: {}", name).into())
        }
    }

    pub fn disable_task(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(task) = tasks.iter_mut().find(|t| t.name == name) {
            task.enabled = false;
            info!("Disabled task: {}", name);
            Ok(())
        } else {
            Err(format!("Task not found: {}", name).into())
        }
    }

    pub fn run(&self) {
        let running = self.running.clone();
        let tasks = self.tasks.clone();
        
        thread::spawn(move || {
            *running.lock().unwrap() = true;
            info!("Scheduler started");
            
            while *running.lock().unwrap() {
                let now = Utc::now();
                let mut tasks_to_run = Vec::new();
                
                {
                    let tasks = tasks.lock().unwrap();
                    for task in tasks.iter() {
                        if task.enabled {
                            if let Ok(schedule) = Schedule::from_str(&task.schedule) {
                                if let Some(next_run) = schedule.upcoming(Utc).next() {
                                    if next_run <= now {
                                        tasks_to_run.push(task.name.clone());
                                    }
                                }
                            } else {
                                warn!("Invalid cron schedule for task: {}", task.name);
                            }
                        }
                    }
                }
                
                for task_name in tasks_to_run {
                    info!("Running scheduled task: {}", task_name);
                    // 这里应该调用实际的爬取逻辑
                    // self.run_task(&task_name);
                }
                
                thread::sleep(Duration::from_secs(60)); // 每分钟检查一次
            }
            
            info!("Scheduler stopped");
        });
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }

    pub fn get_tasks(&self) -> Vec<ScheduledTask> {
        let tasks = self.tasks.lock().unwrap();
        tasks.clone()
    }
}

pub fn create_scheduler() -> Scheduler {
    Scheduler::new()
}