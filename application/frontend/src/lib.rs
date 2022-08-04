pub mod components;
pub mod error_pages;
pub mod data;
pub mod templates;
pub mod utility;
pub mod global_state;

use perseus::{Html, PerseusApp, PerseusRoot};

#[perseus::main]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .template(crate::templates::about::get_template)
        .template(crate::templates::login::get_template)
        .template(crate::templates::chronicle::get_template)
        .template(crate::templates::registration::get_template)
        .error_pages(crate::error_pages::get_error_pages)
        .index_view(|| {
            sycamore::view! {
                // We don't need a `<!DOCTYPE html>`, that's added automatically by Perseus (though that can be overriden if you really want by using `.index_view_str()`)
                // We need a `<head>` and a `<body>` at the absolute minimum for Perseus to work properly (otherwise certain script injections will fail)
                link(rel="icon", type="image/x-icon", href="/.perseus/static/favicon_io/favicon.ico") {}
                link(rel="stylesheet", href="/.perseus/static/bootstrap-5.2.0-dist/css/bootstrap.css"){}
                link(rel="stylesheet", href="/.perseus/static/styles/loremaster/index.css"){}
                link(rel="stylesheet", href="https://fonts.googleapis.com/css2?family=Fira+Mono:wght@400;500;700&family=Fira+Sans:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;0,900;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800;1,900&display=swap")
                head {
                    
                }
                body() {
                    // This creates an element into which our app will be interpolated
                    // This uses a few tricks internally beyond the classic `<div id="root">`, so we use this wrapper for convenience
                    PerseusRoot()
                    // Note that elements in here can't be selectively removed from one page, it's all-or-nothing in the index view (it wraps your whole app)
                    // Note also that this won't be reloaded, even when the user switches pages
                }
            }
        })
        .global_state_creator(crate::global_state::get_global_state_creator())
}
