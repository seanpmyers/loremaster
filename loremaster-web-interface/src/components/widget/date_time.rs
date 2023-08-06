use futures_util::{future::ready, stream::StreamExt};
use gloo_timers::future::IntervalStream;
use js_sys::{Array, Object};
use perseus::prelude::spawn_local_scoped;
use sycamore::prelude::*;
use time::{macros::format_description, OffsetDateTime};
use wasm_bindgen::JsValue;

const ONE_SECOND_IN_MILLISECONDS: u32 = 1_000;

#[component]
pub fn DateTime<G: Html>(context: Scope) -> View<G> {
    let short_date: &Signal<String> = create_signal(context, String::new());
    let day_month: &Signal<String> = create_signal(context, String::new());
    let time: &Signal<String> = create_signal(context, String::new());
    let time_zone: &Signal<String> = create_signal(context, String::new());

    if G::IS_BROWSER {
        let short_format = format_description!(
            "[year]/[month]/[day] [hour repr:12 padding:space]:[minute]:[second] [period]"
        );
        let time_format = format_description!("[hour repr:12 padding:space]:[minute] [period]");
        let rust_time: OffsetDateTime = time::OffsetDateTime::now_local().unwrap();
        short_date.set(rust_time.format(short_format).unwrap());
        time.set(rust_time.format(time_format).unwrap());
        day_month.set(format!(
            "{}, {} {}",
            rust_time.weekday(),
            rust_time.month(),
            rust_time.date().day()
        ));
        spawn_local_scoped(context, async move {
            let options =
                js_sys::Intl::DateTimeFormat::new(&Array::new(), &Object::new()).resolved_options();
            time_zone.set(
                js_sys::Reflect::get(&options, &JsValue::from("timeZone"))
                    .unwrap()
                    .as_string()
                    .unwrap(),
            );

            IntervalStream::new(ONE_SECOND_IN_MILLISECONDS)
                .for_each(|_| {
                    let rust_time = time::OffsetDateTime::now_local().unwrap();
                    short_date.set(rust_time.format(short_format).unwrap());
                    time.set(rust_time.format(time_format).unwrap());
                    day_month.set(format!(
                        "{}, {} {}",
                        rust_time.weekday(),
                        rust_time.month(),
                        rust_time.date().day()
                    ));
                    ready(())
                })
                .await;
        });
    }
    let widget_classes = "date-time-widget";
    view! {context,
        section(class=widget_classes) {
            div() { (day_month.get()) }
            div() { (short_date.get()) }
            div() { (time_zone.get()) }
        }
    }
}
