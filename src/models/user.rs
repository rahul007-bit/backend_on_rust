use diesel::{PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::db::schema::users;

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub roll: String,
    pub department: String,
    pub profile_image: Option<String>,
    pub academic_year: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub roll: String,
    pub department: String,
    pub profile_image: Option<String>,
    pub academic_year: String,
}

impl User {
    pub fn find_all(pool: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
        let users = users::table.load::<User>(pool)?;

        Ok(users)
    }
}
