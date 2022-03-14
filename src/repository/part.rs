use tokio;
use crate::databases::db::DB;
use tokio_postgres::{NoTls};
use crate::models::{Part};

pub async fn gets()->Result<Vec<Part>,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut parts :Vec<Part> = vec![];
    let mut id:String;
    let mut part_type:String;
    let mut name:String;
    let rows = client.query("select * from Part", &[]).await?;
    for row in rows{
        id = row.get(0);
        part_type = row.get(1);
        name = row.get(2);
                        
        parts.push(Part{id:id,part_type:part_type,name:name});
    } 
    Ok(parts)      
}

pub async fn get(_id : String)->Result<Vec<Part>,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut parts :Vec<Part> = vec![];
    let mut id:String;
    let mut part_type:String;
    let mut name:String;
    let command = format!("select * from Part Where id = $1");
    let rows = client.query(&command, &[&_id]).await?;
    for row in rows{
        id = row.get(0);
        part_type = row.get(1);
        name = row.get(2);
                        
        parts.push(Part{id:id,part_type:part_type,name:name});
    } 
    Ok(parts)      
}

pub async fn update(p : Part)->Result<u64,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let command = format!("Update Part SET part_type = $1, name= $2 Where id=$3");
    let rows = client.execute(&command, &[&p.part_type,&p.name,&p.id]).await?;
    
    Ok(rows)      
}
pub async fn insert(p : Part)->Result<u64,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let command = format!("INSERT INTO Part (id,part_type,name) Values ($1,$2,$3)");
    let rows = client.execute(&command, &[&p.id,&p.part_type,&p.name]).await?;
    
    Ok(rows)      
}
pub async fn delete(_id :String)->Result<u64,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let command = format!("Delete From Part Where id = $1");
    let rows = client.execute(&command, &[&_id]).await?;
    
    Ok(rows)      
}