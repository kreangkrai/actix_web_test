use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Debug)]
pub struct Product{
    pub id:String,
    pub name:String,
    pub product_type:String,
    pub detail:String,
    pub price:f64,
}