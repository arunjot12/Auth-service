use diesel::{prelude::*, };
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users{
    pub id: Uuid,
    pub email : String,
    pub password_hash: String
}