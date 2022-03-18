use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub _id: ObjectId,
    pub auth_token: String,
    pub created_at: DateTime,
}
