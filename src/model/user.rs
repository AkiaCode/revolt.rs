use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct FetchProfile {
    pub background: Option<Background>,
    pub content: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Background {
    pub content_type: String,
    pub filename: String,
    pub metadata: Metadata,
    pub size: usize,
    pub tag: String,
    pub _id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub height: usize,
    #[serde(rename = "type")]
    pub _type: String,
    pub width: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchUser {
    pub _id: String,
    pub username: String,
    pub avatar: Option<Avatar>,
    pub relations: Option<Vec<Relation>>,
    pub badges: usize,
    pub status: Option<Status>,
    pub relationship: String,
    pub online: bool,
    pub flags: Option<usize>,
    pub bot: Option<Bot>
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bot {
    pub owner: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub text: Option<String>,
    pub presence: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Avatar {
    pub _id: String,
    pub tag: String,
    pub size: usize,
    pub filename: String,
    pub metadata: Metadata,
    pub content_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    pub status: String,
    pub _id: String,
}