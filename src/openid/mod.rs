use lazy_static::lazy_static;
use regex::Regex;
use url::Url;
use utoipa::{IntoParams, ToSchema};

use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct SteamOpenId {
    return_to: String,
    redirect_url: String,
}

impl SteamOpenId {
    pub fn new(realm: &str, path: &str) -> std::result::Result<Self, NewError> {
        let return_to = Url::parse(realm).map_err(|_| NewError::BadUrl)?;
        let return_to = return_to
            .join(path)
            .map_err(|_| NewError::BadUrl)?
            .to_string();

        let json = RedirectForm {
            ns: "http://specs.openid.net/auth/2.0",
            identity: "http://specs.openid.net/auth/2.0/identifier_select",
            claimed_id: "http://specs.openid.net/auth/2.0/identifier_select",
            mode: "checkid_setup",
            realm,
            return_to: &return_to,
        };
        let form_str = serde_urlencoded::to_string(json).map_err(|_| NewError::BadUrl)?;

        let mut redirect_url = Url::parse("https://steamcommunity.com/openid/login").unwrap();
        redirect_url.set_query(Some(&form_str));

        Ok(Self {
            return_to,
            redirect_url: redirect_url.to_string(),
        })
    }

    pub fn get_redirect_url(&self) -> &str {
        &self.redirect_url
    }

    pub async fn verify(&self, mut json: VerifyForm) -> std::result::Result<i64, VerifyError> {
        lazy_static! {
            static ref STEAMID_REGEX: Regex =
                Regex::new("^https://steamcommunity.com/openid/id/([0-9]{17})$").unwrap();
        }

        if json.return_to != self.return_to {
            return Err(VerifyError::Denied);
        }

        json.mode = "check_authentication".to_owned();
        let form_str = serde_urlencoded::to_string(&json).map_err(|_| VerifyError::BadQuery)?;

        let client = reqwest::Client::new();

        let response = client
            .post("https://steamcommunity.com/openid/login")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_str)
            .send()
            .await
            .map_err(|_| VerifyError::RequestFailed)?
            .text()
            .await
            .map_err(|_| VerifyError::BadResponse)?;

        let is_valid = response.split('\n').any(|line| line == "is_valid:true");

        if !is_valid {
            return Err(VerifyError::Denied);
        }

        let captures = STEAMID_REGEX
            .captures(&json.claimed_id)
            .ok_or(VerifyError::BadQuery)?;

        let steamid64_str = captures.get(1).ok_or(VerifyError::BadQuery)?.as_str();

        let steamid64 = steamid64_str
            .parse::<i64>()
            .map_err(|_| VerifyError::BadQuery)?;

        Ok(steamid64)
    }
}

#[derive(Debug)]
pub enum NewError {
    BadUrl,
}

#[derive(Debug)]
pub enum VerifyError {
    Denied,
    BadQuery,
    RequestFailed,
    BadResponse,
}

impl From<VerifyError> for AppError {
    fn from(value: VerifyError) -> Self {
        match value {
            VerifyError::Denied => AppError::AuthUserDenied,
            VerifyError::BadQuery => AppError::BadAuthQuery,
            VerifyError::RequestFailed => AppError::AuthRequestFailed,
            VerifyError::BadResponse => AppError::AuthBadResponse,
        }
    }
}

#[derive(serde::Serialize)]
pub struct RedirectForm<'a> {
    #[serde(rename = "openid.ns")]
    ns: &'static str,
    #[serde(rename = "openid.identity")]
    identity: &'static str,
    #[serde(rename = "openid.claimed_id")]
    claimed_id: &'static str,
    #[serde(rename = "openid.mode")]
    mode: &'static str,
    #[serde(rename = "openid.return_to")]
    return_to: &'a str,
    #[serde(rename = "openid.realm")]
    realm: &'a str,
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema, IntoParams, Debug)]
pub struct VerifyForm {
    #[serde(rename = "openid.ns")]
    ns: String,
    #[serde(rename = "openid.mode")]
    mode: String,
    #[serde(rename = "openid.op_endpoint")]
    op_endpoint: String,
    #[serde(rename = "openid.claimed_id")]
    claimed_id: String,
    #[serde(rename = "openid.identity")]
    identity: Option<String>,
    #[serde(rename = "openid.return_to")]
    return_to: String,
    #[serde(rename = "openid.response_nonce")]
    response_nonce: String,
    #[serde(rename = "openid.invalidate_handle")]
    invalidate_handle: Option<String>,
    #[serde(rename = "openid.assoc_handle")]
    assoc_handle: String,
    #[serde(rename = "openid.signed")]
    signed: String,
    #[serde(rename = "openid.sig")]
    sig: String,
}
