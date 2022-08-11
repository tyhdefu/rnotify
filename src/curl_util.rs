use std::error::Error;
use curl::easy::{Easy, List};
use crate::error::MessageSendError;

pub fn post_json_to(url: &str, payload: &str) -> Result<(), Box<dyn Error>> {
    let payload_bytes = payload.as_bytes();

    let mut easy = Easy::new();
    easy.url(&url)?;

    let mut headers = List::new();
    headers.append("Accept: application/json")?;
    headers.append("Content-Type:application/json")?;
    easy.http_headers(headers)?;

    easy.post(true)?;
    easy.post_field_size(payload_bytes.len() as u64)?;
    easy.post_fields_copy(payload_bytes)?;
    let mut response_buf = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|buf| {
            response_buf.extend_from_slice(buf);
            Ok(buf.len())
        })?;
        transfer.perform()?;
    }
    let code = easy.response_code()?;
    if code != 200 && code != 204 {
        let response = String::from_utf8(response_buf)?;
        return Err(Box::new(MessageSendError::new(format!("Got response code {}: Response body: {}", code, response))));
    }
    Ok(())
}