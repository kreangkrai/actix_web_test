use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::databases::db::DB;
use tokio_postgres::{NoTls};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub:String,
    pub name: String,
    pub role:String,
    pub iat: i64,
    pub exp:i64,
}
impl Claims{
    pub fn new(sub:String,name:String,role:String,iat:i64,exp:i64)-> Self{
        Claims{sub,name,role,iat,exp}
    }
}

pub fn encoder(c:Claims)->Result<String,jsonwebtoken::errors::Error>{
    let token = encode(&Header::default(),&c , &EncodingKey::from_secret("secret".as_ref()))?;
    Ok(token)
}
pub fn decoder(token:&str) ->Result<Claims,jsonwebtoken::errors::Error>{
    let result = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())?;
    Ok(result.claims)
}

pub async fn validation_token(name:String,sub:String)->Result<bool,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client.query("select from user_management where name=$1 and email=$2", &[&name,&sub]).await?;
    if rows.len() > 0 {
        return Ok(true);
    }
    Ok(false)
}