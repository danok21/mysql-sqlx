 use sqlx::mysql::{MySqlPool,MySqlConnection};
 use sqlx::prelude::*;
 use std::env;
 //use structopt::StructOpt;

#[async_std::main]
// or #[tokio::main]
async fn main()-> anyhow::Result<()> {

    //单个连接
    let mut pool = MySqlConnection::connect(&env::var("DATABASE_URL")?).await?;


    let sql="INSERT INTO student (id,name,age,hobby)VALUES ( 4,'Bob',12,'cycle')";
    let count=sqlx::query(sql).execute(&mut pool).await?;
    println!("{}",count);


    /*
    let sql=  "DELETE FROM student WHERE  id = 3";
    let count=sqlx::query(sql).execute(&mut pool).await?;
    println!("{}",count);
    */


    /*
    let sql=  "UPDATE student SET age = 7 WHERE id = 3 ";
    let count=sqlx::query(sql).bind(1).execute(&mut pool).await?;
    println!("{}",count);
    */

    let sql = "SELECT * FROM student ORDER BY id";
    let mut cursor = sqlx::query(sql).fetch(&mut pool);
    while let Some(row) = cursor.next().await? {

        let id: i32 = row.get(0);
        let name: String = row.get("name");
        let age: i32 = row.get(2);
        let hobby: String = row.get(3);
        println!("{}, {}，{}, {}",id,name,age,hobby);
    }

    Ok(())
 }

