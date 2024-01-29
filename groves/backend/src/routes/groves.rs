use actix_web::{get, post, web};

use bamboo_common::backend::response::{check_missing_fields, created, list};
use bamboo_common::backend::services::{DbConnection, EnvService};
use bamboo_common::backend::utils::get_random_password;
use bamboo_common::backend::{dbal, mailing};
use bamboo_common::core::entities::grove::CreateGroveRequest;
use bamboo_common::core::entities::{Grove, User};
use bamboo_common::core::error::{BambooApiResponseResult, BambooApiResult};

use crate::middleware::authenticate_user::{authenticate, Username};

#[get("/api/grove", wrap = "authenticate!()")]
pub async fn get_groves(db: DbConnection) -> BambooApiResponseResult {
    dbal::get_groves(&db).await.map(|data| list!(data))
}

#[post("/api/grove", wrap = "authenticate!()")]
pub async fn create_grove(
    create_grove: Option<web::Json<CreateGroveRequest>>,
    db: DbConnection,
    env_service: EnvService,
    username: web::ReqData<Username>,
) -> BambooApiResult<Grove> {
    let create_grove = check_missing_fields!(create_grove, "grove")?;
    let grove = dbal::create_grove(create_grove.grove_name.clone(), &db).await?;
    let password = get_random_password();
    let created_mod = dbal::create_user(
        grove.id,
        User::new(
            create_grove.mod_email.clone(),
            create_grove.mod_name.clone(),
            "".into(),
            true,
        ),
        password.clone(),
        &db,
    )
    .await?;
    mailing::user::send_user_created(
        created_mod.display_name.clone(),
        format!("{} (Bambushainteam)", username.into_inner()),
        created_mod.email.clone(),
        password,
        env_service,
    )
    .await?;

    Ok(created!(grove))
}
