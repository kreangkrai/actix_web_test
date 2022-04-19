use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Debug)]
pub struct User{
    pub id:i32,
    pub name:String,
    pub password:String,
    pub email:String,
}