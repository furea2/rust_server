// crud
use actix_web::web;
use serde::Serialize;
use crate::db;
use crate::model::User;
use diesel::RunQueryDsl;

#[derive(Serialize)]
pub struct UserListResponse {
    results: Vec<User>,
}
pub async fn get_user_list(db: web::Data<db::Pool>) -> web::Json<UserListResponse> {
    let mut conn = db.get().unwrap();
    use crate::schema::users::dsl::*;
    let results = users
        .load::<User>(&mut conn)
        .expect("Error loading posts");
    // println!("users: {:?}", results);
    web::Json(UserListResponse { results: results })
}




// use diesel::prelude::*;
// // use uuid::Uuid;

// use crate::models;

// type DbError = Box<dyn std::error::Error + Send + Sync>;

// /// Run query using Diesel to find user by uid and return it.
// pub fn find_user_by_uid(
//     conn: &mut SqliteConnection,
//     uid: Uuid,
// ) -> Result<Option<models::User>, DbError> {
//     use crate::schema::users::dsl::*;

//     let user = users
//         .filter(id.eq(uid.to_string()))
//         .first::<models::User>(conn)
//         .optional()?;

//     Ok(user)
// }


// /// Run query using Diesel to insert a new database row and return the result.
// pub fn insert_new_user(
//     conn: &mut SqliteConnection,
//     nm: &str, // prevent collision with `name` column imported inside the function
// ) -> Result<models::User, DbError> {
//     // It is common when using Diesel with Actix Web to import schema-related
//     // modules inside a function's scope (rather than the normal module's scope)
//     // to prevent import collisions and namespace pollution.
//     use crate::schema::users::dsl::*;

//     let new_user = models::User {
//         id: Uuid::new_v4().to_string(),
//         name: nm.to_owned(),
//     };

//     diesel::insert_into(users).values(&new_user).execute(conn)?;

//     Ok(new_user)
// }
