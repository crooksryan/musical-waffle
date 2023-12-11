use rocket::serde::{
    Serialize,
    Deserialize
};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User<'r> {
    pub username: Option<&'r str>,
    pub uuid: Option<u32>,
    pub followers: Option<u32>,
    pub followees: Option<u32>,

}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Post<'r>{
    pub username: &'r str,
    pub upid: u32,
    pub content: &'r str,
    pub likes: u32
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Login<'r>{
    pub username: &'r str,
    pub password: &'r str,
}

pub struct Likes<'r>{
    pub ulid: u32,
    pub upid: u32,
    pub username: Option<&'r str>,
}
