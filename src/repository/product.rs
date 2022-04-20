use tokio;
use crate::databases::db::DB;
use tokio_postgres::{NoTls};
use crate::models::{Product};
use serde_json::json;

pub async fn gets()->Result<serde_json::Value,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut products :Vec<Product> = vec![];
    let mut id:String;
    let mut name:String;
    let mut product_type:String;
    let mut detail:String;
    let mut price:f64;
    let rows = client.query("select * from products", &[]).await?;
    for row in rows{
        id = row.get(0);
        name = row.get(1);
        product_type = row.get(2);
        detail = row.get(3);
        price = row.get(4);
                        
        products.push(Product{id:id,name:name,product_type:product_type,detail:detail,price:price});
    } 
    Ok(json!({"data":products}))   
}

pub async fn insert(p : Product)->Result<serde_json::Value,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let command = format!("INSERT INTO products (id,name,product_type,detail,price) Values ($1,$2,$3,$4,$5)");

    let rows = client.execute(&command, &[&p.id,&p.name,&p.product_type,&p.detail,&p.price]).await?;
    
    if rows == 1{
        let json = json!({
            "data": serde_json::to_value(&p).unwrap()
        });
        return Ok(json);
    }else{
        let json = json!({
            "Error": "No Data",
        });
        return Ok(json);
    }
}
