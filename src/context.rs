use std::sync::Arc;

use crate::{model::{ready::User, user::{FetchUser, FetchProfile}}, uri::BASE_URL};
use crate::model::message::Message;
use crate::http::Http;
use futures_util::SinkExt;
use futures_util::stream::{SplitSink};
use serde_json::{json, Error};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message as WsMessage;


#[derive(Clone)]
pub struct Context {
    pub token: String,
    pub http: Http,
    pub json: serde_json::Value,
    writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>>>,
    pub bot: User,
}

impl Context {
    pub fn new(
        token: &str,
        json: &str,
        writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>>>,
        bot: User,
    ) -> Context
    {
        Context  {
            token: token.to_owned(),
            http: Http,
            json: serde_json::from_str(json).unwrap(),
            writer,
            bot,
        }
    }
    pub async fn send_message(&self, message: &str, mention: Option<bool>) {
        let json: Result<Message, Error> = serde_json::from_value(self.json.clone());

        if let Ok(json) = json {
            let _ = Http::post(
                format!("{}/channels/{}/messages", BASE_URL, json.channel).as_str(),
                &json!({
                        "content": message,
                        "nonce": ulid::Ulid::new().to_string(),
                        "replies": [{
                            "id": json._id,
                            "mention": mention.unwrap_or(false),
                        }]
                    }),
                &self.token).await;
        }
    }

    pub async fn reply(&self, message: &str) {
        let json: Result<Message, Error> = serde_json::from_value(self.json.clone());

        if let Ok(json) = json {
            let _ = Http::post(
                format!("{}/channels/{}/messages", BASE_URL, json.channel).as_str(),
                &json!({
                        "content": message,
                        "nonce": ulid::Ulid::new().to_string(),
                        "replies": [{
                            "id": json._id,
                            "mention": true,
                        }]
                    }),
                &self.token).await;
        }
    }

    pub async fn fetch_profile(&self, user_id: &str) -> FetchProfile {
        let resp = Http::get(
            format!("{}/users/{}/profile", BASE_URL, user_id).as_str(),
            &self.token).await;

        resp.unwrap().json().await.unwrap()
    }

    pub async fn fetch_user(&self, user_id: &str) -> FetchUser {
        let resp = Http::get(
            format!("{}/users/{}", BASE_URL, user_id).as_str(),
            &self.token).await;

        resp.unwrap().json().await.unwrap()
    }

    pub async fn begin_typing(&self, channel_id: &str) {
        self.writer.lock().await.send(WsMessage::Text(json!({
            "type": "BeginTyping",
            "channel": channel_id
        }).to_string())).await.unwrap();
    }

    pub async fn end_typing(&self, channel_id: &str) {
        self.writer.lock().await.send(WsMessage::Text(json!({
            "type": "EndTyping",
            "channel": channel_id
        }).to_string())).await.unwrap();
    }
}
