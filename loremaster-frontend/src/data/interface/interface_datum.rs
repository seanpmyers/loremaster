use uuid::Uuid;

pub trait InterfaceDatum {
    type InterfaceId;
    type InterfaceDisplayText;
    type InterfaceDescription;
}

pub type InterfaceDisplayText = String;
pub type InterfaceId = Uuid;
pub type InterfaceDescription = String;
