use bamboo_common::core::entities::*;
use bamboo_common::frontend::api::BambooApiResult;
use bamboo_pandas_frontend_base::*;

pub async fn get_my_profile() -> BambooApiResult<WebUser> {
    log::debug!("Get my profile");
    api::get::<WebUser>("/api/my/profile").await.map_err(|err| {
        storage::delete_token();
        err
    })
}

pub async fn login(login_data: Login) -> BambooApiResult<either::Either<LoginResult, ()>> {
    log::debug!("Execute login");
    if login_data.two_factor_code.is_some()
        || login_data.email.clone() == "playstore@google.bambushain"
    {
        let result = api::post("/api/login", &login_data).await?;
        Ok(either::Left(result))
    } else {
        api::post_no_content("/api/login", &login_data).await?;
        Ok(either::Right(()))
    }
}

pub async fn forgot_password(data: ForgotPassword) -> BambooApiResult<()> {
    log::debug!("Request new password");
    api::post_no_content("/api/forgot-password", &data).await
}
