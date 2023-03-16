use iso8601_timestamp::Timestamp;

/// SMS captcha
#[derive(Debug, Serialize, Deserialize)]
pub struct SMSCaptcha {
    /// Id
    #[serde(rename = "_id")]
    pub id: String,
    /// User's phone number
    pub phone_number: String,
    /// SMS captcha
    pub sms_captcha: String,
    /// Time at which this captcha expires
    pub expiry: Timestamp,
}
