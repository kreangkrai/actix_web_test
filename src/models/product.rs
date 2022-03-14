use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Debug)]
pub struct Product{
    pub id:String,
    pub product_type:String,
    pub name:String,
}