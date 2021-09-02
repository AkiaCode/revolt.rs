use std::collections::HashMap;

use super::user::{Bot, Metadata};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ready {
    #[serde(rename = "type")]
    pub _type: String,
    pub channels: Vec<Channel>,
    pub members: Vec<Members>,
    pub servers: Vec<serde_json::Value>, // Error
    pub users: Vec<User>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Members {
    pub _id: Member
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Member {
    pub server: String,
    pub user: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    pub channel_type: String,
    pub _id: String,
    pub server: String,
    pub nonce: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<Icon>,
    pub role_permissions: Option<HashMap<String, usize>>,
    pub active: Option<bool>,
    pub recipients: Option<Vec<String>>,
    pub default_permissions: Option<serde_json::Value>, // Error
    pub last_message: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub avatar: Option<Avatar>,
    pub badges: usize,
    pub online: bool,
    pub relationship: Option<String>,
    pub status: Option<Status>,
    pub username: String,
    pub _id: String,
    pub flags: Option<usize>,
    pub bot: Option<Bot>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Avatar {
    pub content_type: String,
    pub filename: String,
    pub metadata: Metadata,
    pub size: usize,
    pub tag: String,
    pub _id: String
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Status {
    pub text: String,
    pub presence: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Icon {
    pub _id: String,
    pub tag: String,
    pub metadata: Metadata,
    pub content_type: String,
    pub size: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub _id: String,
    pub nonce: String,
    pub owner: String,
    pub name: String,
    pub description: String,
    pub channels: Vec<String>,
    pub categories: Vec<Category>,
    pub system_messages: SystemMessages,
    pub default_permissions: Option<serde_json::Value>,
    pub icon: Option<Icon>,
    pub roles: Option<Vec<HashMap<String, Role>>>,
    pub banner: Option<Icon>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Role {
    pub color: String,
    pub hoist: bool,
    pub name: String,
    pub permissions: Vec<usize>,
    pub rank: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemMessages {
    pub user_joined: Option<String>,
    pub user_left: Option<String>,
    pub user_kicked: Option<String>,
    pub user_banned: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: String,
    pub title: String,
    pub channels: Vec<String>,
}