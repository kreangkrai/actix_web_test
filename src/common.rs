use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Debug)]
pub struct Product{
    pub id:String,
    pub product_type:String,
    pub name:String,
}

#[derive(Deserialize,Serialize)]
pub struct Part{
    pub id:String,
    pub part_type:String,
    pub name:String,
}