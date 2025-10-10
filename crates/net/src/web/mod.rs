#[cfg(feature = "mail")]
pub mod mail;
pub mod reqwest;

use ::reqwest::{Client, blocking::Client as BlockingClient};
use url::{ParseError, Url};
use urlencoding::{decode, encode};

use crate::{Error, Result};

pub const REMOTE_IP_URL: &'static str = "https://api.ipify.org";

pub fn url_encode<T: AsRef<str>>(value: T) -> String {
    encode(value.as_ref()).to_string()
}

pub fn url_decode<T: AsRef<str>>(value: T) -> Result<String> {
    Ok(decode(value.as_ref())
        .map_err(|e| Error::from_std_error(e))?
        .to_string())
}

pub fn to_url<T: AsRef<str>>(value: T) -> Result<Url> {
    const LOCALHOST: &str = "https://localhost";

    let value = value.as_ref();

    if value.is_empty() {
        return Ok(Url::parse(LOCALHOST).map_err(Error::from_std_error)?);
    }

    match Url::parse(value) {
        Ok(it) => Ok(it.into()),
        Err(ParseError::RelativeUrlWithoutBase) => Ok(Url::parse(LOCALHOST)
            .map_err(Error::from_std_error)?
            .join(value)
            .map_err(Error::from_std_error)?),
        Err(_) => Ok(Url::parse(&url_encode(value)).map_err(Error::from_std_error)?),
    }
}

fn append_if_not_empty<T: AsRef<str>>(base: &Url, component: T) -> Result<Url> {
    let component = component.as_ref();

    if component.is_empty() {
        return Ok(base.clone());
    }
    Ok(base.join(component).map_err(Error::from_std_error)?)
}

pub trait AsUrl<T> {
    fn as_url(&self) -> Result<Url>;
}

impl<T: AsRef<str>> AsUrl<T> for T {
    fn as_url(&self) -> Result<Url> {
        to_url(self)
    }
}

impl<T: AsRef<str>> AsUrl<T> for (T, T) {
    fn as_url(&self) -> Result<Url> {
        let base = to_url(&self.0)?;
        append_if_not_empty(&base, &self.1)
    }
}

impl<T: AsRef<str>> AsUrl<T> for (T, T, T) {
    fn as_url(&self) -> Result<Url> {
        let url = to_url(&self.0)?;
        let url = append_if_not_empty(&url, &self.1)?;
        let url = append_if_not_empty(&url, &self.2)?;
        Ok(url)
    }
}

impl<T: AsRef<str>> AsUrl<T> for (T, T, T, T) {
    fn as_url(&self) -> Result<Url> {
        let url = to_url(&self.0)?;
        let url = append_if_not_empty(&url, &self.1)?;
        let url = append_if_not_empty(&url, &self.2)?;
        let url = append_if_not_empty(&url, &self.3)?;
        Ok(url)
    }
}

impl<T: AsRef<str>> AsUrl<T> for (T, T, T, T, T) {
    fn as_url(&self) -> Result<Url> {
        let url = to_url(&self.0)?;
        let url = append_if_not_empty(&url, &self.1)?;
        let url = append_if_not_empty(&url, &self.2)?;
        let url = append_if_not_empty(&url, &self.3)?;
        let url = append_if_not_empty(&url, &self.4)?;
        Ok(url)
    }
}

impl<T: AsRef<str>, const N: usize> AsUrl<T> for [T; N] {
    fn as_url(&self) -> Result<Url> {
        self.iter().try_fold(to_url(&self[0])?, |url, component| {
            append_if_not_empty(&url, component)
        })
    }
}

impl<T: AsRef<str>> AsUrl<T> for Vec<T> {
    fn as_url(&self) -> Result<Url> {
        self.iter().try_fold(to_url(&self[0])?, |url, component| {
            append_if_not_empty(&url, component)
        })
    }
}

pub fn remove<T: AsRef<str>>(url: &mut Url, value: T) {
    let value = value.as_ref();

    if value.is_empty() {
        return;
    }
    url.set_path(&url.path().replace(value, ""));
}

pub fn get_public_ip(client: &BlockingClient) -> Result<String> {
    let response = client
        .get(REMOTE_IP_URL)
        .send()
        .map_err(Error::from_std_error)?;

    if !response.status().is_success() {
        return Err(Error::from_std_error(
            response.error_for_status().unwrap_err(),
        ));
    }

    let text = response.text().map_err(Error::from_std_error)?;
    Ok(text)
}

pub async fn get_public_ip_async(client: &Client) -> Result<String> {
    let response = client
        .get(REMOTE_IP_URL)
        .send()
        .await
        .map_err(Error::from_std_error)?;

    if !response.status().is_success() {
        return Err(Error::from_std_error(
            response.error_for_status().unwrap_err(),
        ));
    }

    let text = response.text().await.map_err(Error::from_std_error)?;
    Ok(text)
}
