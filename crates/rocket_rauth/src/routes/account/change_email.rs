//! Change account email.
//! PATCH /account/change/email
use rauth::models::Account;
use rauth::{RAuth, Result};
use rocket::serde::json::Json;
use rocket::State;
use rocket_empty::EmptyResponse;

/// # Change Data
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DataChangeEmail {
    /// Valid email address
    pub email: String,
    /// Current password
    pub current_password: String,
}

/// # Change Email
///
/// Change the associated account email.
#[openapi(tag = "Account")]
#[patch("/change/email", data = "<data>")]
pub async fn change_email(
    rauth: &State<RAuth>,
    mut account: Account,
    data: Json<DataChangeEmail>,
) -> Result<EmptyResponse> {
    let data = data.into_inner();

    // Make sure email is valid and not blocked
    rauth.config.email_block_list.validate_email(&data.email)?;

    // Ensure given password is correct
    account.verify_password(&data.current_password)?;

    // Send email verification for new email
    account
        .start_email_move(rauth, data.email)
        .await
        .map(|_| EmptyResponse)
}

#[cfg(test)]
#[cfg(feature = "test")]
mod tests {
    use crate::test::*;

    #[cfg(feature = "async-std-runtime")]
    #[async_std::test]
    async fn success() {
        use rocket::http::Header;

        let (db, auth, session, account) = for_test_authenticated("change_email::success").await;
        let client = bootstrap_rocket_with_auth(
            auth,
            routes![crate::web::account::change_email::change_email],
        )
        .await;

        let res = client
            .patch("/change/email")
            .header(ContentType::JSON)
            .header(Header::new("X-Session-Token", session.token.clone()))
            .body(
                json!({
                    "email": "validexample@valid.com",
                    "current_password": "password"
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::NoContent);

        let account = Account::find_one(&db, doc! { "_id": account.id.unwrap() }, None)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(account.email, "validexample@valid.com");
    }

    #[cfg(feature = "async-std-runtime")]
    #[async_std::test]
    async fn success_smtp() {
        use rocket::http::Header;

        let (db, auth, session, account) = for_test_authenticated_with_config(
            "change_email::success_smtp",
            test_smtp_config().await,
        )
        .await;
        let client = bootstrap_rocket_with_auth(
            auth,
            routes![
                crate::web::account::change_email::change_email,
                crate::web::account::verify_email::verify_email
            ],
        )
        .await;

        let res = client
            .patch("/change/email")
            .header(ContentType::JSON)
            .header(Header::new("X-Session-Token", session.token.clone()))
            .body(
                json!({
                    "email": "change_email@smtp.test",
                    "current_password": "password"
                })
                .to_string(),
            )
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::NoContent);

        let mail = assert_email_sendria("change_email@smtp.test".into()).await;
        let res = client
            .post(format!("/verify/{}", mail.code.expect("`code`")))
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::NoContent);

        let account = Account::find_one(&db, doc! { "_id": account.id.unwrap() }, None)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(account.email, "change_email@smtp.test");
    }
}