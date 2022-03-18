use crate::domains::models::database::auth;
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use mongodb::Database;
use std::clone::Clone;
use std::error::Error;

pub struct AuthRepository {
    database: Database,
}

impl Clone for AuthRepository {
    fn clone(&self) -> AuthRepository {
        AuthRepository {
            database: self.database.clone(),
        }
    }
}

impl AuthRepository {
    pub fn new(database: Database) -> AuthRepository {
        AuthRepository {
            database: database.clone(),
        }
    }

    pub async fn verify_token(&self, token: String) -> Result<bool, Box<dyn Error>> {
        let typed_collection = self.database.collection::<auth::Auth>("auths");

        let filter = doc! { "auth_token": token };
        let find_options = FindOneOptions::builder().build();
        let result = typed_collection.find_one(filter, find_options).await?;

        Ok(match result {
            Some(_) => true,
            None => false,
        })
    }
}
