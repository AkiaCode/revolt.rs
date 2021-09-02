use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};
use serde_json::json;
use tokio::{net::TcpStream, spawn, sync::Mutex};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};

use crate::{client::EventHandler, context::Context, model::ready::User};

pub struct WebSocket {
    writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    reader: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    handler: Arc<Box<dyn EventHandler>>
}

impl WebSocket {
    pub async fn new(handler: Box<dyn EventHandler>) -> WebSocket {
        let (ws_stream, _) = connect_async("wss://ws.revolt.chat").await.unwrap();
        let (writer, reader) = ws_stream.split();

        WebSocket {
            writer: Arc::from(Mutex::new(writer)),
            reader: Arc::from(Mutex::new(reader)),
            handler: Arc::from(handler)
        }
    }

    pub async fn connect(&self, token: String) -> &WebSocket {

        self.writer.lock().await.send(Message::Text(json!({
            "type": "Authenticate",
            "token": token
        }).to_string())).await.unwrap();

        let handler_reader = Arc::clone(&self.reader);
        let handler_writer = Arc::clone(&self.writer);
        let arc_token = Arc::clone(&Arc::new(token.to_owned()));
        let arc_handler = Arc::clone(&self.handler);

        spawn(async move {
            crate::websocket::WebSocket::handler(handler_reader, handler_writer, arc_token, arc_handler).await;
        }).await.unwrap();

        self
    }

    pub async fn handler(reader: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
        writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
        token: Arc<String>,
        event: Arc<Box<dyn EventHandler>>)
    {
        let mut bot: Option<User> = None;
        loop {
            while let Some(message) = reader.lock().await.next().await {
                match message {
                    Ok(message) => {
                        if message.is_text() {
                            let json: serde_json::Value = serde_json::from_str(&message.to_string()).unwrap();

                            if let Some(_type) = json["type"].as_str() {
                                if _type == "Ready" {
                                    let ready: crate::model::ready::Ready  = serde_json::from_value(json.clone()).unwrap();
                                    bot = Some(ready.users[0].clone());

                                    let context = Context::new(&token, &message.to_string(), writer.clone(), bot.clone().unwrap());
                                    event.ready(context).await
                                }
                            }
                            
                            if let Some(ref bot) = bot {
                                let context = Context::new(&token, &message.to_string(), writer.clone(), bot.clone());

                                match json["type"].as_str() {
                                    Some("Authenticated") => event.authenticated().await,
                                    Some("Message") => {
                                        let message: Result<crate::model::message::Message, serde_json::Error> = serde_json::from_value(json);

                                        if let Ok(message) = message {
                                            event.on_message(context.to_owned(), message).await;
                                        }
                                    },
                                    Some(&_) => {},
                                    None => {},
                                }
                            }
                        }


                        writer.lock().await.send(Message::Ping(json!({
                            "type": "Ping",
                            "time": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
                        }).to_string()
                        .as_bytes()
                        .to_vec())).await.unwrap();

                        //tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    }
                    Err(e) => {
                        return eprintln!("{:?}", e);
                    }
                }
            }
        }
    }
}