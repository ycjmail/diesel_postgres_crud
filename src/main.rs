#[macro_use]
extern crate diesel;

mod schema;
mod models;

use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::schema::users::dsl::*;
use crate::models::User;
use crate::models::NewUser;
use diesel::insert_into;
use std::error::Error;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn insert_user(pool: &Pool, new_user: NewUser) -> Result<usize, Box<dyn Error>> {
    let mut db_connection = pool.get()?;
    insert_into(users).values(&new_user).execute(&mut db_connection).map_err(|e| e.into())
}

fn delete_user_by_name(pool: &Pool, user_name: &str) -> Result<usize, Box<dyn Error>> {
    let mut db_connection = pool.get()?;
    diesel::delete(users.filter(name.eq(user_name)))
        .execute(&mut db_connection)
        .map_err(|e| e.into())
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        eprintln!("DATABASE_URL not found");
        std::process::exit(1);
    });
  
    let database_pool = Pool::builder()
        .build(ConnectionManager::new(database_url))
        .expect("Failed to create pool.");
  

    let mut db_connection = database_pool.get().expect("Failed to get a connection from the pool.");

    // add a new user
    let new_user = NewUser {
        name: "Alice",
        address: "222 Avenue",
    };

    match insert_user(&database_pool, new_user) {
        Ok(_) => println!("New user inserted successfully"),
        Err(err) => eprintln!("Error inserting new user: {}", err),
    }

    // query all users
    match users.load::<User>(&mut db_connection) {
        Ok(results) => {
            println!("Displaying {} users", results.len());
            for user in results {
                // match user.id {
                //     Some(user_id) => println!("ID: {}", user_id),  
                //     None => println!("ID: None"),
                // }
                println!("Id: {}", user.id);
                println!("Name: {}", user.name);
                println!("Address: {}", user.address);
                println!("----------\n");
            }
        },
        Err(err) => {
            eprintln!("Error loading users: {}", err);
        },
    }
    // remove a user
    let user_name_to_delete = "Alice";
    match delete_user_by_name(&database_pool, user_name_to_delete) {
        Ok(count) => println!("Deleted {} users with name '{}'", count, user_name_to_delete),
        Err(err) => eprintln!("Error deleting user: {}", err),
    }
    // query all users again
    match users.load::<User>(&mut db_connection) {
        Ok(results) => {
            println!("Displaying {} users", results.len());
            for user in results {
                // match user.id {
                //     Some(user_id) => println!("ID: {}", user_id),  
                //     None => println!("ID: None"),
                // }
                println!("Id: {}", user.id);
                println!("Name: {}", user.name);
                println!("Address: {}", user.address);
                println!("----------\n");
            }
        },
        Err(err) => {
            eprintln!("Error loading users: {}", err);
        },
    }
}
