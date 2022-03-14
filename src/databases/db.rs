pub struct DB{
    pub url: &'static str
}
impl DB{
    pub fn url() -> Self{
        DB {url:"postgres://jbguoexu:zPB_3MdrZxzpfh7fkoXw8GEQ0iMv6CbF@ziggy.db.elephantsql.com/jbguoexu"}
    }
}