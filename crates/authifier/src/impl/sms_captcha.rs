use chrono::Duration;
use iso8601_timestamp::Timestamp;
use crate::{models::SMSCaptcha, Authifier, Success};
use alibaba_cloud_sdk_rust::services::dysmsapi;
use gostd::strings;

const ALIYUN_SMS_SERVER_REGION: &str = "cn-hangzhou";
const ALIYUN_SMS_ACCESS_KEY_ID: &str = "LTAI4FwqPxiA111111111";
const ALIYUN_SMS_ACCESS_KEY_SECRET: &str = "ESX1wX11111FJqHTTLwDU2222cP1";

impl SMSCaptcha {
    /// Create a new SMS captcha
    pub fn new(phone_number: String, sms_captcha: String) -> SMSCaptcha {
        SMSCaptcha {
            id: ulid::Ulid::new().to_string(),
            phone_number,
            sms_captcha,
            expiry: Timestamp::from_unix_timestamp_ms(
                chrono::Utc::now()
                    .checked_add_signed(Duration::seconds(180))
                    .expect("failed to checked_add_signed")
                    .timestamp_millis(),
            ),
        }
    }

    /// Save model
    pub async fn save(&self, authifier: &Authifier) -> Success {
        authifier.database.save_sms_captcha(self).await
    }

    /// Send sms via aliyun
    pub async fn send(&self) -> Success {
        let mut client = dysmsapi::Client::NewClientWithAccessKey(
            ALIYUN_SMS_SERVER_REGION,
            ALIYUN_SMS_ACCESS_KEY_ID,
            ALIYUN_SMS_ACCESS_KEY_SECRET,
        ).expect("aliyun service error");
        let mut request = dysmsapi::CreateSendSmsRequest();
        request.PhoneNumbers = strings::Replace(&self.phone_number, "+86", "", -1);
        request.SignName = "阿里云短信测试".to_string();
        request.TemplateCode = "SMS_154950909".to_string();
        request.TemplateParam = json!({"code": &self.sms_captcha}).to_string();
        let response = client.SendSms(&mut request).expect("send sms error");
        println!("{:?}", &response);

        Ok(())
    }
}
