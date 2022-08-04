use perseus::web_log;
use web_sys::RequestMode;

use super::constants::{HTTP_HEADER_CONTENT_TYPE, HTTP_HEADER_CONTENT_TYPE_FORM};

pub async fn post_html_form(
    endpoint_url: &String,
    fields: &Vec<(String, String)>,
) -> Option<String> {
    let mut body_string: String = String::new();
    if fields.len() > 0_usize {
        body_string.push_str(&format!("{}={}", fields[0_usize].0, fields[0_usize].1));

        for entry in fields.iter().skip(1) {
            body_string.push_str(&format!("\n&{}={}", entry.0, entry.1));
        }
    }
    let request_attempt: Result<reqwasm::http::Response, reqwasm::Error> =
        reqwasm::http::Request::post(endpoint_url)
            .mode(RequestMode::SameOrigin)
            .header(HTTP_HEADER_CONTENT_TYPE, HTTP_HEADER_CONTENT_TYPE_FORM)
            .body(body_string)
            .send()
            .await;

    match request_attempt {
        Ok(response) => {
            if response.status() != 200 {
                web_log!("{}", response.status_text());
                return None;
            }

            None
        }
        Err(error) => {
            web_log!("{}", error.to_string());
            None
        }
    }
}

pub async fn get_endpoint(
    endpoint_url: &str,
    query_parameters: Option<&Vec<(String, String)>>,
) -> Option<String> {
    let mut full_request_string: String = String::from(endpoint_url);
    full_request_string.push('?');
    if let Some(parameters) = query_parameters {
        if parameters.len() > 0_usize {
            full_request_string.push_str(&format!(
                "{}={}",
                parameters[0_usize].0, parameters[0_usize].1
            ));

            for parameter in parameters.iter().skip(1) {
                full_request_string.push_str(&format!("&{}={}", parameter.0, parameter.1));
            }
        }
    }

    let request_attempt: Result<reqwasm::http::Response, reqwasm::Error> =
        reqwasm::http::Request::get(&full_request_string)
            .mode(RequestMode::SameOrigin)
            .send()
            .await;
    match request_attempt {
        Ok(response) => {
            if response.status() != 200 {
                log::info!("{}", response.status_text());
                return None;
            }
            match response.text().await {
                Ok(text) => Some(text),
                Err(error) => {
                    log::error!("{}", error.to_string());
                    None
                }
            }
        }
        Err(error) => {
            log::error!("{}", error.to_string());
            None
        }
    }
}
