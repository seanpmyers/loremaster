use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{components::combobox::ComboBoxOption, templates::design_system::ComboBoxDatum};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub id: Uuid,
    pub name: String,
}

impl ComboBoxDatum for Action {
    fn to_combobox_option(self) -> ComboBoxOption {
        ComboBoxOption {
            id: self.id,
            display_text: self.name,
            description: String::new(),
        }
    }
}
