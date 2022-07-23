use super::constants::{HTTP_HEADER_CONTENT_TYPE, HTTP_HEADER_CONTENT_TYPE_FORM};

pub async fn post_html_form(endpoint_url: &String, fields: &Vec<(String, String)>) -> bool {
    let mut body_string: String = String::new();
    if fields.len() > 0_usize {
        body_string.push_str(&format!("{}={}", fields[0_usize].0, fields[0_usize].1));

        for entry in fields.iter().skip(1) {
            body_string.push_str(&format!("\n&{}={}", entry.0, entry.1));
        }
    }
    let request_attempt: Result<reqwasm::http::Response, reqwasm::Error> =
        reqwasm::http::Request::post(endpoint_url)
            .header(HTTP_HEADER_CONTENT_TYPE, HTTP_HEADER_CONTENT_TYPE_FORM)
            .body(body_string)
            .send()
            .await;

    match request_attempt {
        Ok(response) => {
            if response.status() != 200 {
                log::info!("{}", response.status_text());
                return false;
            }

            true
        }
        Err(error) => {
            log::error!("{}", error.to_string());
            false
        }
    }
}
