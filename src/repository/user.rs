use tokio;
use crate::databases::db::DB;
use tokio_postgres::{NoTls};
use crate::models::{User};
use sha2::{Sha256,Digest};
use crate::auth::{tokens,Claims};
use chrono::{Duration,Utc};
use serde_json::json;

pub async fn gets()->Result<Vec<User>,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut parts :Vec<User> = vec![];
    let mut id:i32;
    let mut name:String;
    let mut password:String;
    let mut email:String;
    let rows = client.query("select * from user_management", &[]).await?;
    for row in rows{
        id = row.get(0);
        name = row.get(1);
        password = row.get(2);
        email = row.get(3);
                        
        parts.push(User{id:id,name:name,password:password,email:email});
    } 
    Ok(parts)      
}

pub async fn get(_id : String)->Result<Vec<User>,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut parts :Vec<User> = vec![];
    let mut id:i32;
    let mut name:String;
    let mut password:String;
    let mut email:String;
    let command = format!("select * from user_management Where id = $1");
    let rows = client.query(&command, &[&_id]).await?;
    for row in rows{
        id = row.get(0);
        name = row.get(1);
        password = row.get(2);
        email = row.get(3);
                        
        parts.push(User{id:id,name:name,password:password,email:email});
    } 
    Ok(parts)      
}

pub async fn update(p : User)->Result<u64,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let command = format!("Update user_management SET name = $1, password= $2, email = $3 Where id=$4");
    let rows = client.execute(&command, &[&p.name,&p.password,&p.email,&p.id]).await?;
    
    Ok(rows)      
}
pub async fn insert(p : User)->Result<serde_json::Value,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let command = format!("INSERT INTO user_management (id,name,password,email,token) Values ($1,$2,$3,$4,$5)");

    let mut hasher = Sha256::new();
    hasher.update(&p.password);
    let result = format!("{:x}", hasher.finalize());

    // token
    let issue_date = Utc::now().timestamp_millis();
    let expire_date = (Utc::now() + Duration::days(30)).timestamp_millis();
    let role = String::from("Admin");
    let name = p.name.clone();
    let sub = p.email.clone();
    let cliame:Claims = Claims::new(sub,name,role,issue_date,expire_date);
    let token = tokens::encoder(cliame);

    let rows = client.execute(&command, &[&p.id,&p.name,&result,&p.email,&token.clone().unwrap()]).await?;
    
    if rows == 1{
        let json = json!({
            "token": token.unwrap(),
        });
        return Ok(json);
    }else{
        let json = json!({
            "token": "No Auth",
        });
        return Ok(json);
    }
}
pub async fn delete(_id :i32)->Result<u64,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let command = format!("Delete From user_management Where id = $1");
    let rows = client.execute(&command, &[&_id]).await?;
    
    Ok(rows)      
}