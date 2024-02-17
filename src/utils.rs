use std::fmt::Display;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    error_code: String,
    message: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("error code: {}\n", self.error_code))?;
        f.write_str(&format!("message: {}", self.message))?;
        Ok(())
    }
}

impl std::error::Error for ErrorResponse {}

pub(crate) fn check_for_error_response(response: &str) -> Result<(), ErrorResponse> {
    match serde_json::from_str::<ErrorResponse>(response) {
        Ok(e) => Err(e),
        Err(_) => Ok(()),
    }
}

pub(crate) fn checked_get_request<I: Serialize + ?Sized, O: DeserializeOwned>(
    path: &str,
    input: &I,
) -> Result<O, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(path)
        .json(input)
        .send()?
        .error_for_status()?
        .text()?;

    check_for_error_response(&response)?;

    Ok(serde_json::from_str(&response)?)
}

pub(crate) fn checked_post_request<I: Serialize + ?Sized, O: DeserializeOwned>(
    path: &str,
    input: &I,
) -> Result<O, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .post(path)
        .json(input)
        .send()?
        .error_for_status()?
        .text()?;

    check_for_error_response(&response)?;

    Ok(serde_json::from_str(&response)?)
}
