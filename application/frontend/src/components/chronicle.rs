use dioxus::{events::FormEvent, prelude::*};

pub fn chronicle(context: Scope) -> Element {
    // 	let onsubmit = move |event: FormEvent| {
    //     context.spawn(async move {
    //         let response: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
    //             .post(API_REGISTER_URL)
    //             .form(&[
    //                 ("email_address", &event.values["username"]),
    //                 ("password", &event.values["password"]),
    //             ])
    //             .send()
    //             .await;

    //         match response {
    //             // Parse data from here, such as storing a response token
    //             Ok(_data) => println!("Registration successful!"),

    //             //Handle any errors from the fetch here
    //             Err(_err) => {
    //                 println!(
    //                     "Registration failed - you need a login server running on localhost:8000."
    //                 )
    //             }
    //         }
    //     });
    // };

    context.render(rsx! {
					div { class: "d-flex flex-column",
							h1 { class: "font-bold text-decoration-underline text-center", "loremaster" }
							div {}
					}
	})
}
