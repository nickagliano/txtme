use lettre::{
    transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport,
};

use super::SmsProvider;

pub struct EmailSmsProvider {
    mailer:  SmtpTransport,
    from:    String,
    gateway: String,
}

impl EmailSmsProvider {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let host = std::env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".into());
        let user = std::env::var("SMTP_USER")?;
        let pass = std::env::var("SMTP_PASS")?;
        let gateway = std::env::var("SMS_GATEWAY")?;

        let creds  = Credentials::new(user.clone(), pass);
        let mailer = SmtpTransport::starttls_relay(&host)?
            .credentials(creds)
            .build();

        Ok(Self { mailer, from: user, gateway })
    }
}

impl SmsProvider for EmailSmsProvider {
    fn send(&self, message: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let email = Message::builder()
            .from(self.from.parse()?)
            .to(self.gateway.parse()?)
            .subject("")
            .body(message.to_string())?;

        self.mailer.send(&email)?;
        Ok(())
    }
}
