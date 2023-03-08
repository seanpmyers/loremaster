use futures_util::{future::ready, stream::StreamExt};
use gloo_timers::future::IntervalStream;
use js_sys::{Array, Object};
use sycamore::prelude::*;
use time::macros::format_description;
use wasm_bindgen::JsValue;

const ONE_SECOND_IN_MILLISECONDS: u32 = 1_000;

#[component(DateTime<G>)]
pub fn date_time() -> View<G> {
    let short_date: Signal<String> = Signal::new(String::from(""));
    let day_month: Signal<String> = Signal::new(String::from(""));
    let time: Signal<String> = Signal::new(String::from(""));
    let time_zone: Signal<String> = Signal::new(String::from(""));

    if G::IS_BROWSER {
        let short_format =
            format_description!("[year]/[month]/[day] [hour repr:12]:[minute]:[second] [period]");
        let time_format = format_description!("[hour repr:12]:[minute] [period]");
        let rust_time = time::OffsetDateTime::now_local().unwrap();
        short_date.set(rust_time.format(short_format).unwrap());
        time.set(rust_time.format(time_format).unwrap());
        day_month.set(format!(
            "{}, {} {}",
            rust_time.weekday().to_string(),
            rust_time.month().to_string(),
            rust_time.date().day().to_string()
        ));
        perseus::spawn_local(
            cloned!((short_date, day_month, time, time_zone) => async move {
                let options = js_sys::Intl::DateTimeFormat::new(&Array::new(), &Object::new()).resolved_options();
                time_zone.set(js_sys::Reflect::get(&options, &JsValue::from("timeZone")).unwrap().as_string().unwrap());

                IntervalStream::new(ONE_SECOND_IN_MILLISECONDS).for_each(|_| {
                    let rust_time = time::OffsetDateTime::now_local().unwrap();
                    short_date.set(rust_time.format(short_format).unwrap());
                    time.set(rust_time.format(time_format).unwrap());
                    day_month.set(format!(
                        "{}, {} {}",
                        rust_time.weekday().to_string(),
                        rust_time.month().to_string(),
                        rust_time.date().day().to_string()
                    ));
                    ready(())
                }).await;
            }),
        );
    }
    let widget_classes = "date-time-widget";
    let time_classes = "";
    let date_classes = "";
    let time_zone_classes = "";
    view! {

        section(class=widget_classes) {
            div(class=time_classes) { (short_date.get()) }
            div(class=date_classes) { (day_month.get()) }
            div(class=time_classes) { (time.get()) }
            div(class=time_zone_classes) { (time_zone.get()) }
        }
    }
}
