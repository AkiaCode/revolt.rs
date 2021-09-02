
pub const BASE_URL: &str = "https://api.revolt.chat";

pub enum Authentication {
    UserID,
    SessionToken,
    BotToken(String),
}

impl Authentication {
    pub fn to_string(&self) -> String {
        match self {
            Authentication::UserID => "/auth/login".to_string(),
            Authentication::SessionToken => "/auth/login".to_string(),
            Authentication::BotToken(id) => format!("/bots/{}", id),
        }
    }
}

pub enum ChatPlatForm {
    Core,
    CheckOnboardingStatus,
    CompleteOnboarding,
}

pub enum Auth {
    CreateAccount
}

pub enum User {
    FetchUserProfile
}