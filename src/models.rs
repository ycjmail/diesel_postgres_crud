use diesel::{Queryable, Insertable};
use diesel::prelude::*;
use diesel::pg::Pg;
use crate::schema::users;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub address: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub address: &'a str,
}
