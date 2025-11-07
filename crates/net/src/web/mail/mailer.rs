use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};

use crate::Result;
use emix::Error;

#[derive(Clone)]
#[must_use]
pub struct Mailer {
    smtp: SmtpTransport,
}

impl Mailer {
    pub fn new(host: &str, username: &str, password: &str) -> Result<Self> {
        Ok(Self {
            smtp: SmtpTransport::relay(host)
                .map_err(|e| Error::from_std_error(e))?
                .credentials(Credentials::new(username.to_owned(), password.to_owned()))
                .build(),
        })
    }

    pub fn from(smtp: SmtpTransport) -> Self {
        Mailer { smtp }
    }

    pub fn send(&self, from: &str, to: &str, subject: &str, body: &str) -> Result<()> {
        let email = Message::builder()
            .from(from.parse().map_err(|e| Error::from_std_error(e))?)
            .to(to.parse().map_err(|e| Error::from_std_error(e))?)
            .subject(subject)
            .body(body.to_string())
            .map_err(|e| Error::from_std_error(e))?;
        self.smtp
            .send(&email)
            .map_err(|e| Error::from_std_error(e))?;
        Ok(())
    }
}
