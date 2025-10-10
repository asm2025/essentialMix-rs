use lettre::{
    Message, SmtpTransport, Transport,
    transport::smtp::{authentication::Credentials, response::Response},
};

use crate::Result;

#[derive(Clone)]
pub struct Mailer {
    smtp: SmtpTransport,
}

impl Mailer {
    pub fn new(host: &str, username: &str, password: &str) -> Result<Self> {
        Ok(Self {
            smtp: SmtpTransport::relay(host)?
                .credentials(Credentials::new(username.to_owned(), password.to_owned()))
                .build(),
        })
    }

    pub fn from(smtp: SmtpTransport) -> Self {
        Mailer { smtp }
    }

    pub fn send(&self, from: &str, to: &str, subject: &str, body: &str) -> Result<Response> {
        let email = Message::builder()
            .from(from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .body(body.to_string())?;
        self.smtp.send(&email)?
    }
}
