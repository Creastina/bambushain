use std::fmt::{Display, Formatter};

#[cfg(feature = "backend")]
use sea_orm::ActiveValue::Set;
#[cfg(feature = "backend")]
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(feature = "backend", derive(DeriveEntityModel), sea_orm(table_name = "user"))]
pub struct Model {
    #[cfg_attr(feature = "backend", sea_orm(primary_key))]
    pub id: i32,
    #[cfg_attr(feature = "backend", sea_orm(unique))]
    pub username: String,
    pub password: String,
    pub is_mod: bool,
    pub gear_level: String,
    pub job: String,
    pub discord_name: String,
}

#[cfg(feature = "backend")]
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::crafter::Entity")]
    Crafter,
    #[sea_orm(has_many = "super::fighter::Entity")]
    Fighter,
    #[sea_orm(has_many = "super::token::Entity")]
    Token,
}

#[cfg(feature = "backend")]
impl Related<super::crafter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Crafter.def()
    }
}

#[cfg(feature = "backend")]
impl Related<super::fighter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Fighter.def()
    }
}

#[cfg(feature = "backend")]
impl Related<super::token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Token.def()
    }
}

#[cfg(feature = "backend")]
impl ActiveModelBehavior for ActiveModel {}

#[cfg(feature = "backend")]
impl ActiveModel {
    pub fn set_password(&mut self, plain_password: &String) -> Result<(), bcrypt::BcryptError> {
        let hashed = bcrypt::hash(plain_password.as_bytes(), 12);
        match hashed {
            Ok(hashed_password) => {
                self.password = Set(hashed_password);
                Ok(())
            }
            Err(err) => Err(err)
        }
    }
}

impl Model {
    pub fn new(username: String, password: String, job: String, gear_level: String, discord_name: String, is_mod: bool) -> Self {
        Self {
            id: i32::default(),
            username,
            password,
            is_mod,
            gear_level,
            job,
            discord_name,
        }
    }

    #[cfg(feature = "backend")]
    pub fn validate_password(&self, password: String) -> bool {
        bcrypt::verify(password, self.password.as_str()).unwrap_or(false)
    }

    pub fn to_web_user(&self) -> WebUser {
        WebUser {
            id: self.id,
            username: self.username.to_string(),
            is_mod: self.is_mod,
            gear_level: self.gear_level.to_string(),
            job: self.job.to_string(),
            discord_name: self.discord_name.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebUser {
    pub id: i32,
    pub username: String,
    pub is_mod: bool,
    pub gear_level: String,
    pub job: String,
    pub discord_name: String,
}

impl Display for WebUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap_or(self.username.clone()).as_str())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfile {
    pub gear_level: String,
    pub job: String,
    pub discord_name: String,
}

impl UpdateProfile {
    pub fn new(job: String, gear_level: String, discord_name: String) -> Self {
        Self {
            job,
            gear_level,
            discord_name,
        }
    }
}
