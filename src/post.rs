use serde::{Serialize, Deserialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post
{
    pub uuid:u32,
    pub title:String,
    pub category: String, 
    pub author: String,
    pub tags: Vec<String>,
    pub visibility:bool,
    pub date: SystemTime,
    pub body: Option<String>,
}