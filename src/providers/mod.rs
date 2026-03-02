pub mod email_sms;
pub mod log;

pub trait SmsProvider: Send + Sync {
    fn send(&self, message: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
