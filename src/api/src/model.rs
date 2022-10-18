use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NewUser {
//     pub name: String,
// }
