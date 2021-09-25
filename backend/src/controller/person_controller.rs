
use rocket::{State, fs::NamedFile, get, http::{
      Cookie, 
      CookieJar
   }, post, response::{
      Flash, 
      Redirect
   }};



/// Retrieve the user's ID, if any.
#[get("/user_id")]
fn user_id(cookies: &CookieJar<'_>) -> Option<String> {
    cookies.get_private("user_id")
        .map(|crumb| format!("User ID: {}", crumb.value()))
}

/// Remove the `user_id` cookie.
#[post("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}
