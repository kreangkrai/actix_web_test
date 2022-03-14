use tokio;
use crate::databases::db::DB;
use tokio_postgres::{NoTls};
use crate::common::{Product};

pub async fn gets()->Result<Vec<Product>,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut products :Vec<Product> = vec![];
    let mut id:String;
    let mut product_type:String;
    let mut name:String;
    let rows = client.query("select * from Product", &[]).await?;
    for row in rows{
        id = row.get(0);
        product_type = row.get(1);
        name = row.get(2);
                        
        products.push(Product{id:id,product_type:product_type,name:name});
    } 
    Ok(products)      
}

pub async fn get(_id : String)->Result<Vec<Product>,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut products :Vec<Product> = vec![];
    let mut id:String;
    let mut product_type:String;
    let mut name:String;
    let command = format!("select * from Product Where id = $1");
    let rows = client.query(&command, &[&_id]).await?;
    for row in rows{
        id = row.get(0);
        product_type = row.get(1);
        name = row.get(2);
                        
        products.push(Product{id:id,product_type:product_type,name:name});
    } 
    Ok(products)      
}

pub async fn update(p : Product)->Result<u64,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let command = format!("Update Product SET product_type = $1, name= $2 Where id=$3");
    let rows = client.execute(&command, &[&p.product_type,&p.name,&p.id]).await?;
    
    Ok(rows)      
}
pub async fn insert(p : Product)->Result<u64,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let command = format!("INSERT INTO Product (id,product_type,name) Values ($1,$2,$3)");
    let rows = client.execute(&command, &[&p.id,&p.product_type,&p.name]).await?;
    
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
    let command = format!("Delete From Product Where id = $1");
    let rows = client.execute(&command, &[&_id]).await?;
    
    Ok(rows)      
}