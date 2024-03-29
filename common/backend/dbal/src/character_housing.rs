use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{IntoActiveModel, NotSet, QueryOrder};

use bamboo_common_core::entities::*;
use bamboo_common_core::entities::{character, character_housing};
use bamboo_common_core::error::*;

pub async fn get_character_housings(
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<Vec<CharacterHousing>> {
    character_housing::Entity::find()
        .filter(character_housing::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .order_by_asc(character_housing::Column::District)
        .order_by_asc(character_housing::Column::Ward)
        .order_by_asc(character_housing::Column::Plot)
        .all(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("character_housing", "Failed to load character housings")
        })
}

pub async fn get_character_housing(
    id: i32,
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooResult<CharacterHousing> {
    character_housing::Entity::find_by_id(id)
        .filter(character_housing::Column::CharacterId.eq(character_id))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .one(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("character_housing", "Failed to load character housing")
        })
        .map(|res| {
            if let Some(res) = res {
                Ok(res)
            } else {
                Err(BambooError::not_found(
                    "character_housing",
                    "The character housing was not found",
                ))
            }
        })?
}

async fn character_housing_exists_by_id(
    id: i32,
    user_id: i32,
    character_id: i32,
    district: HousingDistrict,
    ward: i16,
    plot: i16,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    character_housing::Entity::find()
        .filter(character_housing::Column::Id.ne(id))
        .filter(character_housing::Column::CharacterId.eq(character_id))
        .filter(character_housing::Column::District.eq(district))
        .filter(character_housing::Column::Ward.eq(ward))
        .filter(character_housing::Column::Plot.eq(plot))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("Failed to load character housing {err}");
            BambooError::database("character_housing", "Failed to load the character housings")
        })
}

async fn character_housing_exists_by_fields(
    user_id: i32,
    character_id: i32,
    district: HousingDistrict,
    ward: i16,
    plot: i16,
    db: &DatabaseConnection,
) -> BambooResult<bool> {
    character_housing::Entity::find()
        .filter(character_housing::Column::CharacterId.eq(character_id))
        .filter(character_housing::Column::District.eq(district))
        .filter(character_housing::Column::Ward.eq(ward))
        .filter(character_housing::Column::Plot.eq(plot))
        .filter(character::Column::UserId.eq(user_id))
        .inner_join(character::Entity)
        .count(db)
        .await
        .map(|count| count > 0)
        .map_err(|err| {
            log::error!("Failed to load character housing {err}");
            BambooError::database("character_housing", "Failed to load the character housings")
        })
}

pub async fn create_character_housing(
    user_id: i32,
    character_id: i32,
    housing: CharacterHousing,
    db: &DatabaseConnection,
) -> BambooResult<CharacterHousing> {
    if character_housing_exists_by_fields(
        user_id,
        character_id,
        housing.district,
        housing.ward,
        housing.plot,
        db,
    )
    .await?
    {
        return Err(BambooError::exists_already(
            "character_housing",
            "A character housing with that address exists already",
        ));
    }

    let mut model = housing.into_active_model();
    model.character_id = Set(character_id);
    model.id = NotSet;

    model.insert(db).await.map_err(|err| {
        log::error!("{err}");
        BambooError::database("character_housing", "Failed to create character housing")
    })
}

pub async fn update_character_housing(
    id: i32,
    user_id: i32,
    character_id: i32,
    housing: CharacterHousing,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    if character_housing_exists_by_id(
        id,
        user_id,
        character_id,
        housing.district,
        housing.ward,
        housing.plot,
        db,
    )
    .await?
    {
        return Err(BambooError::exists_already(
            "character_housing",
            "A character housing with that address exists already",
        ));
    }

    let mut active_housing = get_character_housing(id, user_id, character_id, db)
        .await?
        .into_active_model();
    active_housing.district = Set(housing.district);
    active_housing.housing_type = Set(housing.housing_type);
    active_housing.ward = Set(housing.ward);
    active_housing.plot = Set(housing.plot);

    active_housing
        .update(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("character_housing", "Failed to update housing")
        })
        .map(|_| ())
}

pub async fn delete_character_housing(
    id: i32,
    user_id: i32,
    character_id: i32,
    db: &DatabaseConnection,
) -> BambooErrorResult {
    get_character_housing(id, user_id, character_id, db)
        .await?
        .delete(db)
        .await
        .map_err(|err| {
            log::error!("{err}");
            BambooError::database("character_housing", "Failed to delete character housing")
        })
        .map(|_| ())
}
