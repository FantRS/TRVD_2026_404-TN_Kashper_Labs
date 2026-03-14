use std::fmt::{Display, Formatter, Result};
use uuid::Uuid;

pub enum RedisKey {
    WhiteList(String),
    UserTokens(Uuid),
}

impl Display for RedisKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            RedisKey::WhiteList(jti) => write!(f, "wl:{}", jti),
            RedisKey::UserTokens(uid) => write!(f, "ut:{}", uid),
        }
    }
}
