use crate::{context::Context, model::message::Message, websocket::WebSocket};

#[async_trait::async_trait]
pub trait EventHandler: Send + Sync + 'static {
    /*async fn error(&self);*/
    async fn authenticated(&self);
    async fn ready(&self, ctx: Context);
    async fn on_message(&self, ctx: Context, message: Message);
    /*async fn message_update(&self);
    async fn message_delete(&self);
    async fn channel_create(&self);
    async fn channel_update(&self);
    async fn channel_delete(&self);
    async fn channel_group_join(&self);
    async fn channel_group_leave(&self);
    async fn channel_start_typing(&self);
    async fn channel_stop_typing(&self);
    async fn channel_ack(&self);
    async fn server_update(&self);
    async fn server_delete(&self);
    async fn server_member_update(&self);
    async fn server_member_join(&self);
    async fn server_member_leave(&self);
    async fn server_role_update(&self);
    async fn server_role_delete(&self);
    async fn user_update(&self);
    async fn user_relationship(&self);*/
}


pub struct Client {
    pub token: String,
    pub websocket: Option<WebSocket>,
}


impl Client {
    pub async fn new(token: String) -> Self {
        Self {
            token,
            websocket: None,
        }
    }


    pub async fn run<S>(&mut self, event_handler: S) where S: EventHandler + Send + Sync + 'static {
        let websocket = WebSocket::new(Box::new(event_handler)).await;
        self.websocket = Some(websocket);
        self.websocket.as_mut().unwrap().connect(self.token.clone()).await;
    }
}