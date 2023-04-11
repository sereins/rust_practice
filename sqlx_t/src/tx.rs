


#[cfg(test)]
mod tests{
    use sqlx::{MySqlPool, Pool};
    use sqlx::mysql::MySqlPoolOptions;

    #[tokio::test]
    async fn hello(){
        let pool = MySqlPoolOptions::new()
            .connect("mysql://mysql:password@localhost/test")
            .await
            .unwrap();

        let mut tx = pool.begin().await.unwrap();
        let sql = "SELECT * FROM table_name"; // 修改为合法的 SQL 语句

        let res = sqlx::query(sql)
            .execute(&mut tx)
            .await
            .unwrap();

        let sql = "INSERT INTO table_name (column1, column2) VALUES (?, ?)"; // 修改为合法的 SQL 语句

        let res = sqlx::query(sql)
            .bind("value1")
            .bind("value2")
            .execute(&mut tx)
            .await
            .unwrap();

        tx.commit().await.unwrap(); // 提交事务
    }
}