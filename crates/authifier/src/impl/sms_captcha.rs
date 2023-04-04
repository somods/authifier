use chrono::Duration;
use iso8601_timestamp::Timestamp;
use rand::Rng;
use crate::{models::SMSCaptcha, Authifier, Success};
use tencentcloud_sdk_sms::{
    client::{Client, SendSmsRequest},
    credentials::Credential,
};

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

    /// Send sms via tencent
    pub async fn send(&self) -> Success {
        // prepare param
        let secret_id = "AKIDseFcmkV4ScgdR1xiBVo275sVEl9NTP8R";
        let secret_key = "BD3e9VeLttiS1PehaHu6B9zc2MVVM4hE";
        let sms_sdk_app_id = "1400808726";
        let template_id = "1751340";
        let sign_name = "MOD摩兜网";
        let region = "ap-guangzhou";
        let phone_number_set = vec![self.phone_number.to_owned()];
        let template_param_set = vec![self.sms_captcha.to_owned()];
        // build client
        let credential = Credential::new(secret_id, secret_key, None);
        let client = Client::new(credential, region);
        // build request
        let request = SendSmsRequest::new(
            phone_number_set.clone(),
            sms_sdk_app_id,
            template_id,
            sign_name,
            template_param_set,
        );
        // send
        let response = client.send_sms(request).await;
        // check
        match response {
            Ok(res) => {
                phone_number_set.into_iter().for_each(|phone_number| {
                    println!(
                        "send {}: {:?}",
                        phone_number,
                        res.check_is_success(phone_number.to_owned())
                    );
                });
            }
            Err(e) => {
                println!("send error: {}", e);
            }
        }

        Ok(())
    }
}
