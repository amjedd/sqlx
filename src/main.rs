#![allow(unused)]

use sqlx::{FromRow,Row,postgres::{PgPoolOptions,PgRow}};

#[tokio::main]
async fn main() -> Result<(),sqlx::Error>{
   //1- Create connection
   let pool =PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:12345@localhost/actix")
        .await?;

   //2- Create table Ticket if it not exist
   sqlx::query(
      r#"
      CREATE TABLE IF NOT EXISTS ticket(
         id bigserial,
         name text
      );
      "#,
   )
   .execute(&pool)
   .await?;
   //3- Insert a new ticket 

   //4- Select all tickets

   //5- Select query with map()

   //6- Select query_as 


    Ok(())
}
