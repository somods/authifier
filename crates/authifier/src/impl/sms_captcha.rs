use chrono::Duration;
use iso8601_timestamp::Timestamp;
use rand::Rng;
use crate::{models::SMSCaptcha, Authifier, Success};
use sms::aliyun::Aliyun;

impl SMSCaptcha {
    /// Create a new SMS captcha
    pub fn new(phone_number: String) -> SMSCaptcha {
        let mut rng = rand::thread_rng();
        let sms_captcha = rng.gen_range(1000..9999).to_string();

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
        let aliyun = Aliyun::new(
            "LTAI5tCoAXkPkTXdHZrFBXqU",
            "od8cOiHpDbgrWC61sgLdjq501mURk3"
        );
        let response = aliyun.send_sms(
            &self.phone_number,
            "阿里云短信测试",
            "SMS_154950909",
            &json!({"code": &self.sms_captcha}).to_string(),
        ).await;
        println!("{:?}", &response);

        Ok(())
    }
}
