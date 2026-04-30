use {
    crate::{
        app::kurv_ui::KurvAppContext,
        common::err::{self, AppErr},
        db::models::user,
    },
    argon2::{
        Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
        password_hash::{SaltString, rand_core::OsRng},
    },
    axum::{
        extract::{FromRef, FromRequestParts},
        http::request::Parts,
    },
    sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set},
};

pub struct UsersService {
    db: sea_orm::DatabaseConnection,
}

impl FromRequestParts<KurvAppContext> for UsersService {
    type Rejection = String;
    async fn from_request_parts(
        _: &mut Parts,
        state: &KurvAppContext,
    ) -> Result<Self, Self::Rejection> {
        let db = KurvAppContext::from_ref(state).db;
        Ok(UsersService { db })
    }
}

impl UsersService {
    pub async fn get_by_username(&self, username: &str) -> Result<Option<user::Model>, AppErr> {
        let user = user::Entity::find()
            .filter(user::Column::Username.eq(username))
            .one(&self.db)
            .await
            .map_err(|e| err::internal_error(format!("Database query failed: {}", e)))?;

        Ok(user)
    }

    pub async fn create(&self, username: &str, password: &str) -> Result<user::Model, AppErr> {
        // Check if the username already exists
        if self.get_by_username(username).await?.is_some() {
            return Err(err::bad_request("Username already exists"));
        }

        let hash = hash_password(password)?;

        // Create the user in the database
        let new_user = user::ActiveModel {
            username: Set(username.into()),
            password_hash: Set(hash),
        };

        let user = new_user
            .insert(&self.db)
            .await
            .map_err(|e| err::internal_error(format!("Database insert failed: {}", e)))?;

        Ok(user)
    }

    pub async fn verify(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<user::Model>, AppErr> {
        let user = match self.get_by_username(username).await? {
            Some(user) => user,
            None => return Ok(None), // User doesn't exist
        };

        if verify_password(password, &user.password_hash).is_ok() {
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
}

fn hash_password(password: &str) -> Result<String, AppErr> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| err::internal_error(format!("Password hashing failed: {}", e)))
        .map(|hash| hash.to_string())
}

fn verify_password(password: &str, password_hash: &str) -> Result<(), AppErr> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| err::internal_error(format!("Failed to parse password hash: {}", e)))?;

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|e| err::internal_error(format!("Password verification failed: {}", e)))
}
