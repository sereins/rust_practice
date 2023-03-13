pub mod initialize;

#[cfg(test)]
mod tests {
    use mysql::{Opts, TxOpts};
    use mysql::prelude::Queryable;
    use crate::initialize::get_pool;
    use super::*;

    #[test]
    fn it_works() {
        let pool = get_pool();
        let mut con = pool.start_transaction(TxOpts::default()).unwrap();

        /// t 这个类型必须要实现 FromRow
        let a = con.query("select * from user");
        let b = con.query_map();
        let c = con.query_iter();
        let b = con.query_opt();
        let z = con.query_first();

    }
}
