use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Contact {
    pub id: Id,
    pub name: Name,
    pub phone: Phone,
    pub selected: Selected,
    pub success: Success,
    pub fail: Fail,
}

impl Contact {
    pub fn new(id: &str, name: &str, phone: &str) -> Self {
        Contact {
            id: Id(id.to_string()),
            name: Name(name.to_string()),
            phone: Phone(phone.to_string()),
            selected: Selected(false),
            success: Success(0),
            fail: Fail(0),
        }
    }
}
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Id(String);

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Selected(bool);
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Name(String);
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Phone(String);
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Success(i32);
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Fail(i32);

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Contact2 {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub selected: bool,
    pub success: i32,
    pub fail: i32,
}

impl Contact2 {
    pub fn new(id: &str, name: &str, phone: &str) -> Self {
        Contact2 {
            id: id.to_string(),
            name: name.to_string(),
            phone: phone.to_string(),
            selected: false,
            success: 0,
            fail: 0,
        }
    }
}
