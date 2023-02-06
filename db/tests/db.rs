use db::sql::sql_builder;
use db::sql::sql_builder::SqlBuilder;
#[test]
pub fn test_new(){
    let sql_builder = SqlBuilder::new("test".to_string());
    println!("{:?}",sql_builder)
}


#[test]
pub fn get_sql(){
    let mut  sql_builder = SqlBuilder::new("test".to_string());
    sql_builder.table("hh".to_string());
    let sql = sql_builder.build_sql().unwrap();
    println!("sql:{}",sql)
}

#[test]
pub fn fields(){
    let mut sql_builder = SqlBuilder::new("test".to_string());

    sql_builder.field("id,barcode,code");

    let sql = sql_builder.build_sql().unwrap();

    println!("sql:{}",sql)
}

#[test]
pub fn fields_arr(){
    let mut sql_builder = SqlBuilder::new("test".to_string());

    sql_builder.field_arr(vec!["id","barcode","code"]);

    let sql = sql_builder.build_sql().unwrap();

    println!("sql={}",sql)
}

#[test]
pub fn tes(){
    let  a = "da";

    let b = a + "23";
    println!("{}",b)
}