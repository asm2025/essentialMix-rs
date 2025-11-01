use chrono::{DateTime, Utc};
use html_entities::decode_html_entities;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fmt::{Display, Result as DisplayResult};

use crate::Result;
use crate::web::reqwestx::build_client_for_api;
#[cfg(feature = "mail")]
use emix::random;
use emix::{
    Error,
    date::{parse_date, parse_date_ftz, utc_today},
};

const URL_TEMP_MAIL: &str = "https://api.internal.temp-mail.io/api/v3/";
const URL_EMAIL_FAKE: &str = "https://email-fake.com/";
const URL_SEC_MAIL: &str = "https://www.1secmail.com/api/v1/";

static RGX_EMAIL_FAKE_GENERATE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
		r#"(?m)(?s)onchange="change_username\(\)".+?value="(.+?)".+? value="(.+?)" id="domainName2""#,
	)
	.expect("Failed to compile regex")
});
static RGX_EMAIL_FAKE_LINKS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)(?s)<a href=".+?<div class="fem from.+?>(.+?)</div>.+?<div class="fem subj.+?>(.+?)</div>.+?<div class="fem time.+?>(.+?)</div>"#)
        .expect("Failed to compile regex")
});
static RGX_EMAIL_FAKE_MESSAGE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)(?s)<span>From:.+?<span>(.+?)<span.+?<span>Subject:.+?<h1.+?>(.+?)</h1>.+?<span>Received:.+?<span>(.+?)<span.+?<div class="elementToProof".+?>[\s\n]*(.+?)</div>"#).expect("Failed to compile regex")
});

static __HTTP: Lazy<reqwest::Client> = Lazy::new(|| {
    build_client_for_api()
        .build()
        .expect("Failed to create HTTP client")
});

#[derive(Serialize)]
struct NewNameLength {
    #[serde(rename = "min_name_length")]
    min: usize,
    #[serde(rename = "max_name_length")]
    max: usize,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SecMailDomain {
    #[default]
    SecMailCom,
    SecMailOrg,
    SecMailNet,
    WwjmpCom,
    EsiixCom,
    XojxeCom,
    YoggmCom,
    IcznnCom,
    EzzttCom,
    VjuumCom,
    LaafdCom,
    TxcctCom,
}

impl Display for SecMailDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> DisplayResult {
        match self {
            SecMailDomain::SecMailCom => write!(f, "1secmail.com"),
            SecMailDomain::SecMailNet => write!(f, "1secmail.net"),
            SecMailDomain::SecMailOrg => write!(f, "1secmail.org"),
            SecMailDomain::EsiixCom => write!(f, "esiix.com"),
            SecMailDomain::EzzttCom => write!(f, "ezztt.com"),
            SecMailDomain::IcznnCom => write!(f, "icznn.com"),
            SecMailDomain::LaafdCom => write!(f, "laafd.com"),
            SecMailDomain::TxcctCom => write!(f, "txcct.com"),
            SecMailDomain::VjuumCom => write!(f, "vjuum.com"),
            SecMailDomain::WwjmpCom => write!(f, "wwjmp.com"),
            SecMailDomain::XojxeCom => write!(f, "xojxe.com"),
            SecMailDomain::YoggmCom => write!(f, "yoggm.com"),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TempMailProvider {
    #[default]
    Tempmail,
    EmailFake,
    SecMail(SecMailDomain),
}

#[derive(Deserialize)]
struct TempMailMessage {
    id: String,
    from: String,
    subject: String,
    created_at: String,
}

#[derive(Deserialize)]
struct SecMailMessage {
    id: String,
    from: String,
    subject: String,
    date: String,
}

#[derive(Clone)]
pub struct TempMail {
    provider: TempMailProvider,
    username: String,
    domain: String,
}

impl TempMail {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn address(&self) -> String {
        format!("{}@{}", self.username, self.domain)
    }

    pub fn new(provider: TempMailProvider, username: &str, domain: &str) -> Self {
        if username.is_empty() {
            panic!("username is empty");
        }

        if domain.is_empty() {
            panic!("domain is empty");
        }

        TempMail {
            provider: provider.clone(),
            username: username.to_owned(),
            domain: domain.to_owned(),
        }
    }

    pub fn from(email: &TempMail) -> Self {
        TempMail {
            provider: email.provider.clone(),
            username: email.username.to_string(),
            domain: email.domain.to_string(),
        }
    }

    #[cfg(feature = "mail")]
    pub async fn random() -> Result<Self> {
        let provider = match random::numeric(0..100) {
            0..=33 => TempMailProvider::Tempmail,
            34..=66 => TempMailProvider::EmailFake,
            _ => {
                let domain = match random::numeric(0..100) {
                    0..=10 => SecMailDomain::SecMailCom,
                    11..=20 => SecMailDomain::SecMailOrg,
                    21..=30 => SecMailDomain::SecMailNet,
                    31..=40 => SecMailDomain::WwjmpCom,
                    41..=50 => SecMailDomain::EsiixCom,
                    51..=60 => SecMailDomain::XojxeCom,
                    61..=70 => SecMailDomain::YoggmCom,
                    71..=80 => SecMailDomain::IcznnCom,
                    81..=90 => SecMailDomain::EzzttCom,
                    _ => SecMailDomain::VjuumCom,
                };
                TempMailProvider::SecMail(domain)
            }
        };
        Self::generate(provider).await
    }

    pub async fn generate(provider: TempMailProvider) -> Result<Self> {
        match provider {
            TempMailProvider::Tempmail => Self::temp_mail_generate().await,
            TempMailProvider::EmailFake => Self::email_fake_generate().await,
            TempMailProvider::SecMail(domain) => Ok(Self::sec_mail_generate(domain)),
        }
    }

    async fn temp_mail_generate() -> Result<Self> {
        let url = format!("{}email/new", URL_TEMP_MAIL);
        let json: Value = __HTTP
            .post(&url)
            .json(&json!(NewNameLength { min: 4, max: 32 }))
            .send()
            .await
            .map_err(|e| Error::from_std_error(e))?
            .json()
            .await
            .map_err(|e| Error::from_std_error(e))?;
        match json {
            Value::Object(map) => {
                let email = map
                    .get("email")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| Error::Parse("Missing or invalid email field".to_string()))?;
                Ok(Self::parse(TempMailProvider::Tempmail, email))
            }
            _ => Err(Error::Parse("Invalid response format".to_string())),
        }
    }

    async fn email_fake_generate() -> Result<Self> {
        let body = Self::email_fake_get_content(URL_EMAIL_FAKE)
            .await
            .map_err(|e| Error::from_std_error(e))?;

        if body.is_empty() {
            return Err(Error::NoInput);
        }

        let start = match body.find("fem coserch") {
            Some(index) => index,
            None => return Err(Error::NotFound("coserch".to_string())),
        };
        let body = &body[start..];
        let end = match body.find("fem dropselect") {
            Some(index) => index,
            None => return Err(Error::NotFound("dropselect".to_string())),
        };
        let body = &body[..end];
        let captures = match RGX_EMAIL_FAKE_GENERATE.captures(&body) {
            Some(captures) => captures,
            None => return Err(Error::NotFound("username and domain".to_string())),
        };
        let username = captures
            .get(1)
            .map(|c| c.as_str())
            .ok_or_else(|| Error::Parse("Missing username in response".to_string()))?;
        let domain = captures
            .get(2)
            .map(|c| c.as_str())
            .ok_or_else(|| Error::Parse("Missing domain in response".to_string()))?;
        Ok(TempMail {
            provider: TempMailProvider::EmailFake,
            username: username.to_string(),
            domain: domain.to_string(),
        })
    }

    #[cfg(feature = "mail")]
    fn sec_mail_generate(domain: SecMailDomain) -> Self {
        let len = random::numeric(4..32);
        let username = random::string(len);
        TempMail {
            provider: TempMailProvider::SecMail(domain.clone()),
            username,
            domain: domain.to_string(),
        }
    }

    pub fn parse(provider: TempMailProvider, email: &str) -> Self {
        if email.is_empty() {
            panic!("email is empty");
        }

        if let Some(index) = email.find('@') {
            let (username, domain) = email.split_at(index);
            TempMail {
                provider: provider.clone(),
                username: username.to_string(),
                domain: domain[1..].to_string(),
            }
        } else {
            panic!("email is invalid");
        }
    }

    pub async fn find_string(
        &self,
        from: Option<&str>,
        subject: Option<&str>,
        date: Option<DateTime<Utc>>,
        expected: &str,
        size: usize,
    ) -> Result<String> {
        if expected.is_empty() {
            panic!("Expected is empty");
        }

        if size == 0 {
            panic!("Size is zero");
        }

        match &self.provider {
            TempMailProvider::Tempmail => {
                self.temp_mail_find_string(from, subject, date, expected, size)
                    .await
            }
            TempMailProvider::EmailFake => {
                self.email_fake_find_string(from, subject, date, expected, size)
                    .await
            }
            TempMailProvider::SecMail(_) => {
                self.sec_mail_find_string(from, subject, date, expected, size)
                    .await
            }
        }
    }

    fn extract_value(body: &str, expected: &str, size: usize) -> String {
        if body.is_empty() {
            return "".to_string();
        }

        if let Some(index) = &body.find(expected) {
            let index = index + expected.len();
            let size = if index + size < body.len() {
                size
            } else {
                body.len() - index
            };

            if size == 0 {
                return "".to_string();
            }

            let text = &body[index..];
            let text = text.chars().take(size).collect::<String>();
            return text;
        }

        "".to_string()
    }

    async fn temp_mail_find_string(
        &self,
        from: Option<&str>,
        subject: Option<&str>,
        date: Option<DateTime<Utc>>,
        expected: &str,
        size: usize,
    ) -> Result<String> {
        let from = match from {
            Some(from) => from.to_lowercase(),
            None => "".to_owned(),
        };
        let subject = match subject {
            Some(subject) => subject.to_lowercase(),
            None => "".to_owned(),
        };
        let date_min = date.unwrap_or_else(|| utc_today());
        let url = format!("{}email/{}/messages", URL_TEMP_MAIL, self.address());
        let messages: Vec<TempMailMessage> = __HTTP
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::from_std_error(e))?
            .json()
            .await
            .map_err(|e| Error::from_std_error(e))?;

        if let Some(message) = messages.iter().rev().find(|e| {
            (from.is_empty() || e.from.contains(&from))
                && (subject.is_empty() || e.subject.contains(&subject))
                && parse_date_ftz(&e.created_at)
                    .map(|d| d > date_min)
                    .unwrap_or(false)
        }) {
            let url = format!("{}message/{}", URL_TEMP_MAIL, message.id);
            let json: Value = __HTTP
                .get(&url)
                .send()
                .await
                .map_err(|e| Error::from_std_error(e))?
                .json()
                .await
                .map_err(|e| Error::from_std_error(e))?;
            let body = json["body_text"].as_str().unwrap_or_default();
            return Ok(Self::extract_value(&body, expected, size));
        }

        Ok("".to_string())
    }

    async fn email_fake_find_string(
        &self,
        from: Option<&str>,
        subject: Option<&str>,
        date: Option<DateTime<Utc>>,
        expected: &str,
        size: usize,
    ) -> Result<String> {
        if expected.is_empty() {
            panic!("Expected is empty");
        }

        if size == 0 {
            panic!("Size is zero");
        }

        let mut content =
            Self::email_fake_get_content(&format!("{}{}", URL_EMAIL_FAKE, self.address()))
                .await
                .map_err(|e| Error::from_std_error(e))?;
        let mut body = Self::email_fake_get_email_table(&content);

        if body.is_empty() {
            return Ok("".to_string());
        }

        let from = match from {
            Some(from) => from.to_lowercase(),
            None => "".to_owned(),
        };
        let subject = match subject {
            Some(subject) => subject.to_lowercase(),
            None => "".to_owned(),
        };
        let date_min = date.unwrap_or_else(|| utc_today());
        let mut links = Vec::new();
        for c in RGX_EMAIL_FAKE_LINKS.captures_iter(&body) {
            let date_str = c
                .get(4)
                .map(|m| m.as_str())
                .ok_or_else(|| Error::Parse("Missing date".to_string()))?;
            let date = parse_date(date_str).map_err(|e| Error::from_std_error(e))?;
            links.push((
                c.get(1).map(|m| m.as_str()).unwrap_or("").to_string(),
                c.get(2).map(|m| m.as_str()).unwrap_or("").to_lowercase(),
                c.get(3).map(|m| m.as_str()).unwrap_or("").to_lowercase(),
                date,
            ));
        }

        if !links.is_empty() {
            let target = match links.iter().find(|(_, f, s, d)| {
                (from.is_empty() || f.to_lowercase().contains(&from))
                    && (subject.is_empty() || s.to_lowercase().contains(&subject))
                    && d >= &date_min
            }) {
                Some(item) => item.0.to_owned(),
                None => return Ok("".to_string()),
            };
            content = Self::email_fake_get_content(&format!("{}{}", URL_EMAIL_FAKE, target))
                .await
                .map_err(|e| Error::from_std_error(e))?;
            body = Self::email_fake_get_email_table(&content);

            if body.is_empty() {
                return Ok("".to_string());
            }
        }

        let body = match decode_html_entities(&body) {
            Ok(b) => b,
            Err(e) => {
                return Err(Error::from_other_error(format!(
                    "Failed to decode HTML entities: {:?}",
                    e
                )));
            }
        };

        if let Some(matches) = RGX_EMAIL_FAKE_MESSAGE.captures(&body) {
            let f = matches
                .get(2)
                .map(|m| m.as_str())
                .ok_or_else(|| Error::Parse("Missing from field".to_string()))?;
            let s = matches
                .get(3)
                .map(|m| m.as_str())
                .ok_or_else(|| Error::Parse("Missing subject field".to_string()))?;
            let d = matches
                .get(4)
                .map(|m| m.as_str())
                .ok_or_else(|| Error::Parse("Missing date field".to_string()))
                .and_then(|s| parse_date(s).map_err(|e| Error::from_std_error(e)))?;

            if (!f.is_empty() && !f.contains(&from))
                || (!s.is_empty() && !s.contains(&from))
                || d < date_min
            {
                return Ok("".to_string());
            }

            let text = matches
                .get(5)
                .map(|m| m.as_str())
                .ok_or_else(|| Error::Parse("Missing text field".to_string()))?;
            return Ok(Self::extract_value(text, expected, size));
        }

        Ok("".to_string())
    }

    async fn email_fake_get_content(url: &str) -> Result<String> {
        let response = __HTTP
            .get(url)
            .send()
            .await
            .map_err(|e| Error::from_std_error(e))?;
        response
            .error_for_status_ref()
            .map_err(|e| Error::from_std_error(e))?;
        Ok(response
            .text()
            .await
            .map_err(|e| Error::from_std_error(e))?)
    }

    fn email_fake_get_email_table(body: &str) -> &str {
        if body.is_empty() {
            return "";
        }

        let start = match body.find("email-table") {
            Some(index) => index,
            None => return "",
        };
        let body = &body[start..];
        let end = match body.find(r#"<script src="https://cdn.jsdelivr.net/"#) {
            Some(index) => index,
            None => return "",
        };
        &body[..end]
    }

    async fn sec_mail_find_string(
        &self,
        from: Option<&str>,
        subject: Option<&str>,
        date: Option<DateTime<Utc>>,
        expected: &str,
        size: usize,
    ) -> Result<String> {
        let from = match from {
            Some(from) => from.to_lowercase(),
            None => "".to_owned(),
        };
        let subject = match subject {
            Some(subject) => subject.to_lowercase(),
            None => "".to_owned(),
        };
        let date_min = date.unwrap_or_else(|| utc_today());
        let domain = match &self.provider {
            TempMailProvider::SecMail(domain) => domain,
            _ => panic!("Invalid provider"),
        };

        let url = format!(
            "{}?action=getMessages&login={}&domain={}",
            URL_SEC_MAIL, self.username, domain
        );
        let messages: Vec<SecMailMessage> = __HTTP
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::from_std_error(e))?
            .json()
            .await
            .map_err(|e| Error::from_std_error(e))?;

        if let Some(message) = messages.iter().rev().find(|e| {
            (from.is_empty() || e.from.contains(&from))
                && (subject.is_empty() || e.subject.contains(&subject))
                && parse_date(&e.date).map(|d| d > date_min).unwrap_or(false)
        }) {
            let url = format!(
                "{}?action=readMessage&login={}&domain={}&id={}",
                URL_SEC_MAIL,
                self.username(),
                self.domain,
                message.id
            );
            let json: Value = __HTTP
                .get(&url)
                .send()
                .await
                .map_err(|e| Error::from_std_error(e))?
                .json()
                .await
                .map_err(|e| Error::from_std_error(e))?;
            let body = json["html_body"].as_str().unwrap_or("");
            return Ok(Self::extract_value(body, expected, size));
        }

        Ok("".to_string())
    }
}
