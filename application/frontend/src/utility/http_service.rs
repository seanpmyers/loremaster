use super::constants::{FORM_CONTENT_TYPE, HTTP_HEADER_CONTENT_TYPE};

pub fn post_html_form(endpoint_url: &String, fields: &Vec<(String, String)>) -> bool {
    let mut body_string: String = String::new();

    if fields.len() > 0_usize {
        body_string.push_str(&format!("{}={}", fields[0_usize].0, fields[0_usize].1));

        for entry in fields.iter().skip(1) {
            body_string.push_str(&format!("\n&{}={}", entry.0, entry.1));
        }
    }

    let mut http_request: ehttp::Request =
        ehttp::Request::post(&format!("{}", endpoint_url), body_string.into_bytes());

    http_request.headers.insert(
        String::from(HTTP_HEADER_CONTENT_TYPE),
        String::from(FORM_CONTENT_TYPE),
    );

    let mut result: bool = false;

    ehttp::fetch(
        http_request,
        move |fetch_result: Result<ehttp::Response, String>| match fetch_result {
            Ok(_response) => result = true,
            Err(_error) => result = false,
        },
    );

    result
}
