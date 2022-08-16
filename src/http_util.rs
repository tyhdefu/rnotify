use std::error::Error;
use serde::Serialize;
use crate::error::MessageSendError;

pub fn post_as_json_to<T: Serialize>(url: &str, payload: &T) -> Result<(), Box<dyn Error>> {
    let response = minreq::post(url)
        .with_json(payload)?
        .with_header("Accept", "application/json")
        .send()?;
    let code = response.status_code;
    if code != 200 && code != 204 {
        let response = response.as_str()?;
        return Err(Box::new(MessageSendError::new(format!("Got response code {}: Response body: {}", code, response))));
    }
    Ok(())
}