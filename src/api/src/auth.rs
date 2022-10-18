use actix_web::{http::header, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::{Deserialize, Serialize};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    user_name: String,
}

pub async fn index(user: Option<Identity>) -> impl Responder {
    let html = IndexTemplate {
        user_name: if let Some(user) = user {
            let user_data: UserData = serde_json::from_str(&user.id().unwrap()).unwrap();
            user_data.user_name
        } else {
            "".to_string()
        }
    };
    let view = html.render().expect("failed to render html");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(view)
}

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    user_id: String,
    user_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    user_id: String,
    user_name: String,
}

// post params
#[derive(Serialize, Deserialize)]
pub struct LoginParams {
    user_name: String,
}

pub async fn login(db: web::Data<db::Pool>, request: HttpRequest, params: web::Form<LoginParams>) -> impl Responder {
    // login data
    let user_name = &params.user_name;

    // user存在チェック
    if true {
        // ユーザーdbを参照
        let user_id = &params.user_name;
        // println!("user_id: {}", user_id);

        // get user data
        let _user_data = get_user_by_user_name(db, (&user_name).to_string());
        println!("user_data: {:?}", &_user_data.await.username);

        // session
        // session data
        let _session_data = SessionData {
            user_id: user_id.clone(),
            user_name: user_name.clone(),
        };
        // session 情報をredisに登録
        let session_data_str = serde_json::to_string(&_session_data).unwrap();
        let user = Identity::login(&request.extensions(), session_data_str.into()).unwrap();

        let user_data: UserData = serde_json::from_str(&user.id().unwrap()).unwrap();
        // user_name = &user_data.user_name;
        println!("User {:?} login", &user_data.user_name);

    } else { /* ユーザーが存在しない時の処理 */ }

    HttpResponse::SeeOther().append_header((header::LOCATION, "/")).finish()
}

pub async fn logout(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        println!("User {:?} logout", &user.id());
        user.logout();
    }
    HttpResponse::SeeOther().append_header((header::LOCATION, "/")).finish()
}

#[derive(Serialize)]
pub struct UserResponse {
    user_name: String,
}

pub async fn get_user_info(user: Option<Identity>) -> web::Json<UserResponse> {
    let user_name;
    if let Some(user) = user {
        let user_data: UserData = serde_json::from_str(&user.id().unwrap()).unwrap();
        user_name = user_data.user_name;
        println!("user_name {}", &user_name);
    } else {
        user_name = "unknown".to_string();
    }
    web::Json(UserResponse { user_name: user_name })
}


// get user data from session params
use crate::db;
use crate::model::User;
use diesel::prelude::*;
use diesel::RunQueryDsl;
pub async fn get_user_by_user_name(db: web::Data<db::Pool>, target_username: String) -> User {
    let mut conn = db.get().unwrap();
    use crate::schema::users::dsl::*;
    users
        .filter(username.eq(&target_username))
        .first(&mut conn)
        .expect("Error loading posts")
}

