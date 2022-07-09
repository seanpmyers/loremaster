use std::path::PathBuf;

use anyhow::anyhow;
use rocket::{fs::NamedFile, get, response::content::RawHtml, routes};
use sycamore::view;
use tokio::fs;

use crate::{
    api::response::ApiError,
    utility::constants::files::{FAVICON_PATH, INDEX_PATH},
};

#[macro_export]
macro_rules! home_uri {
    ($($t:tt)*) => (rocket::uri!("/", $crate::controller:: $($t)*))
}

#[get("/favicon.ico")]
async fn favicon() -> Result<Option<NamedFile>, ApiError> {
    let favicon_file: NamedFile = NamedFile::open(FAVICON_PATH)
        .await
        .map_err(|error| anyhow!("{}", error))?;

    Ok(Some(favicon_file))
}

#[get("/", rank = 3)]
async fn index() -> Result<RawHtml<String>, ApiError> {
    let index_html: String = String::from_utf8(
        fs::read(INDEX_PATH)
            .await
            .map_err(|error| anyhow!("{}", error))?,
    )
    .map_err(|error| anyhow!("{}", error))?;

    let rendered: String = sycamore::render_to_string(|| {
        view! {
            frontend::App(Some(String::from("/")))
        }
    });

    let index_html: String = index_html.replace("%sycamore.body", &rendered);

    Ok(RawHtml(index_html))
}

#[get("/<path>", rank = 2)]
async fn path(path: PathBuf) -> Result<RawHtml<String>, ApiError> {
    let index_html: String = String::from_utf8(
        fs::read(INDEX_PATH)
            .await
            .map_err(|error| anyhow!("{}", error))?,
    )
    .map_err(|error| anyhow!("{}", error))?;

    let mut pathname = String::new();
    for segment in &path {
        pathname += match segment.to_str() {
            Some(string) => string,
            None => "",
        };
        pathname += "/";
    }

    let rendered: String = sycamore::render_to_string(|| {
        view! {
            frontend::App(Some(String::from(pathname)))
        }
    });

    let index_html: String = index_html.replace("%sycamore.body", &rendered);

    Ok(RawHtml(index_html))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, path, favicon]
}
