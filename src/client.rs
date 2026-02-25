use log::{debug, error};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::Fio;

fn map_status_error(status: StatusCode) -> Option<Error> {
    match status {
        StatusCode::CONFLICT => Some(Error::Limit),
        StatusCode::INTERNAL_SERVER_ERROR => Some(Error::Malformed),
        StatusCode::PAYLOAD_TOO_LARGE => Some(Error::TooLarge),
        StatusCode::NOT_FOUND => Some(Error::Token),
        StatusCode::UNPROCESSABLE_ENTITY => Some(Error::HistoricalDataLocked),
        _ => None,
    }
}

impl Fio {
    pub(crate) async fn api_get<T: DeserializeOwned>(&self, rest_method: &str) -> Result<T, Error> {
        match self.api_get_text(rest_method).await {
            Ok(v) => {
                let de: Result<T, _> = serde_json::from_str(&v);
                match de {
                    Ok(reply) => Ok(reply),
                    Err(e) => {
                        error!("Couldn't parse reply for {} call: {}", rest_method, e);
                        debug!("Source JSON: {}", v);
                        Err(e.into())
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn api_get_text(&self, rest_method: &str) -> Result<String, Error> {
        match reqwest::get(format!("https://fioapi.fio.cz/v1/rest/{rest_method}")).await {
            Ok(resp) => {
                if let Some(e) = map_status_error(resp.status()) {
                    return Err(e);
                }
                match resp.text().await {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e.into()),
                }
            }
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn api_post(
        &self,
        rest_method: &str,
        import_type: &str,
        body: String,
    ) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let form = reqwest::multipart::Form::new()
            .text("token", self.token.clone())
            .text("type", import_type.to_string())
            .part(
                "file",
                match reqwest::multipart::Part::text(body)
                    .file_name("import.xml")
                    .mime_str("application/xml")
                {
                    Ok(file) => file,
                    Err(e) => {
                        return Err(e.into());
                    }
                },
            );
        match client
            .post(format!("https://fioapi.fio.cz/v1/rest/{rest_method}"))
            .multipart(form)
            .send()
            .await
        {
            Ok(resp) => {
                if let Some(e) = map_status_error(resp.status()) {
                    return Err(e);
                }
                match resp.text().await {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e.into()),
                }
            }
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn api_get_empty(&self, rest_method: &str) -> Result<(), Error> {
        match self.api_get_text(rest_method).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
