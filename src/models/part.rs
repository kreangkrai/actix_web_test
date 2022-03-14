use serde::{Deserialize,Serialize};
#[derive(Deserialize,Serialize)]
pub struct Part{
    pub id:String,
    pub part_type:String,
    pub name:String,
}