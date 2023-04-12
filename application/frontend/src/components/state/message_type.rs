use serde::{Deserialize, Serialize};

use crate::components::icon::{
    ALERT_OCTAGON_SVG_HTML, ALERT_TRIANGLE_SVG_HTML, CHECK_SVG_HTML, INFO_SVG_HTML,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum MessageType {
    Information,
    Warning,
    Error,
    Success,
}

pub fn get_message_type_icon(message_type: &MessageType) -> &'static str {
    match message_type {
        MessageType::Information => INFO_SVG_HTML,
        MessageType::Warning => ALERT_TRIANGLE_SVG_HTML,
        MessageType::Error => ALERT_OCTAGON_SVG_HTML,
        MessageType::Success => CHECK_SVG_HTML,
    }
}
