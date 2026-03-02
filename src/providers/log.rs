use super::SmsProvider;

pub struct LogProvider;

impl SmsProvider for LogProvider {
    fn send(&self, message: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("[txtme/log] {message}");
        Ok(())
    }
}
