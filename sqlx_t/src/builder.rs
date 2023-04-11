use sqlx::{MySql, QueryBuilder};

fn builder() {

    let mut qb = QueryBuilder::<'_, MySql>::new(
        "insert into"
    );
}


mod tests {
    use crate::builder::builder;

    #[test]
    fn sh() {
        builder();
    }
}