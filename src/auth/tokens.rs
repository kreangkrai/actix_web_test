use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub name: String,
    pub iat: i64,
    pub sub:String,
}
impl Claims{
    pub fn new(name:String,iat:i64,sub:String)-> Self{
        Claims{name,iat,sub}
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
