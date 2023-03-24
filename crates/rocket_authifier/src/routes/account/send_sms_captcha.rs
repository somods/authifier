//! Send sms captcha
//! POST /account/reset_password
use authifier::{Authifier, Result};
use rocket::serde::json::Json;
use rocket::State;
use rocket_empty::EmptyResponse;
use authifier::models::SMSCaptcha;

/// # SMS Captcha Information
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DataSendSmsCaptcha {
    /// Phone Number
    pub phone_number: String,
}

/// # Send Sms Captcha
///
/// Send sms captcha.
#[openapi(tag = "Account")]
#[post("/sms_captcha", data = "<data>")]
pub async fn send_sms_captcha(
    authifier: &State<Authifier>,
    data: Json<DataSendSmsCaptcha>,
) -> Result<EmptyResponse> {
    let data = data.into_inner();
    let phone_number = data.phone_number;

    let sms_captcha = SMSCaptcha::new(phone_number);
    sms_captcha.save(authifier).await?;
    sms_captcha.send().await?;

    // Never fail this route, (except for db error)
    // You may open the application to email enumeration otherwise.
    Ok(EmptyResponse)
}
