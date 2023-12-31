use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::entities::user::User;
use crate::models::access_token::AccessTokenModel;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessToken {
    pub id: String,
    pub name: String,
    pub key: String,
    pub created_date: NaiveDateTime,
    pub created_by: Option<Box<User>>,
    pub expiration: Option<NaiveDateTime>,
}

impl From<AccessTokenModel> for AccessToken {
    fn from(model: AccessTokenModel) -> Self {
        AccessToken {
            id: model.id,
            name: model.name,
            key: model.key,
            created_date: model.created_date,
            expiration: model.expiration,
            created_by: None,
        }
    }
}

impl AccessToken {
    pub fn from_vec_model(model: Vec<AccessTokenModel>) -> Vec<Self> {
        model.iter().map(|x| x.clone().into()).collect()
    }

    pub fn is_expired(&self) -> bool {
        self.expiration.map(|x| x < Utc::now().naive_utc()).unwrap_or(false)
    }
}