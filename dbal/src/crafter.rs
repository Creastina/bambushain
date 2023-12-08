use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{IntoActiveModel, NotSet, QueryOrder, QuerySelect};

use bamboo_entities::prelude::*;
use bamboo_entities::{character, crafter};
use bamboo_error::*;

use crate::prelude::character_exists;

pub async fn get_crafters(
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<Crafter>> {
    crafter::Entity::find()
        .filter(crafter::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .order_by_asc(crafter::Column::Job)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("crafter", "Failed to load crafters")
        })
}

pub async fn get_crafter(
    id: i32,
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Crafter> {
    match crafter::Entity::find_by_id(id)
        .filter(crafter::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .one(db)
        .await
    {
        Ok(Some(res)) => Ok(res),
        Ok(None) => Err(bamboo_not_found_error!(
            "crafter",
            "The crafter was not found"
        )),
        Err(err) => {
            log::error!("{err}");
            Err(bamboo_db_error!(
                "crafter",
                "Failed to execute database query"
            ))
        }
    }
}

pub async fn crafter_exists(
    id: i32,
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> bool {
    crafter::Entity::find_by_id(id)
        .select_only()
        .column(crafter::Column::Id)
        .filter(crafter::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .count(db)
        .await
        .map(|count| count > 0)
        .unwrap_or(false)
}

pub async fn crafter_exists_by_job(
    user_id: i32,
    character_id: i32,
    job: CrafterJob,
    db: &DatabaseConnection,
) -> bool {
    crafter::Entity::find()
        .select_only()
        .column(crafter::Column::Id)
        .filter(crafter::Column::Job.eq(job))
        .filter(crafter::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .count(db)
        .await
        .map(|count| count > 0)
        .unwrap_or(false)
}

pub async fn create_crafter(
    user_id: i32,
    character_id: i32,
    crafter: Crafter,
    db: &DatabaseConnection,
) -> BambooResult<Crafter> {
    if !character_exists(user_id, character_id, db).await {
        return Err(bamboo_not_found_error!(
            "crafter",
            "The character does not exist"
        ));
    }

    let mut model = crafter.into_active_model();
    model.character_id = Set(character_id);
    model.id = NotSet;

    model.insert(db).await.map_err(|err| {
        log::error!("{err}");
        bamboo_db_error!("crafter", "Failed to create crafter")
    })
}

pub async fn update_crafter(
    id: i32,
    crafter: Crafter,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    crafter::Entity::update_many()
        .filter(crafter::Column::Id.eq(id))
        .col_expr(crafter::Column::Level, Expr::value(crafter.level))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("crafter", "Failed to update crafter")
        })
        .map(|_| ())
}

pub async fn delete_crafter(id: i32, db: &DatabaseConnection) -> BambooErrorResult {
    crafter::Entity::delete_many()
        .filter(crafter::Column::Id.eq(id))
        .exec(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            bamboo_db_error!("crafter", "Failed to delete crafter")
        })
        .map(|_| ())
}
