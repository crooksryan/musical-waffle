use crate::dataobj::{
        User,
        Post
};


use rocket::{
    fs::{NamedFile, relative},
    serde::json::Json
};


#[post("/user", data="<user>")]
pub async fn user(user: Json<User<'_>>) -> Json<User>{
    if user.uuid.unwrap() <= 100{
        return Json(User{ username: None, uuid: None, followers: None, followees: None })
    }
    Json(User { username: user.username, uuid: user.uuid, followers: None, followees: None })
}


#[post("/post", data="<post>")]
pub async fn post(post: Json<Post<'_>>) -> Json<Post>{
    Json(Post { username: post.username, upid: post.upid, content: post.content, likes: 0})
}


#[get("/")]
pub async fn index() -> Option<NamedFile>{
    NamedFile::open(relative!("/front-end/dist/index.html")).await.ok()
}


#[get("/name/<name>")]
pub async fn hello(name: &str) -> String{
    format!("Hello, {}", name)
}
