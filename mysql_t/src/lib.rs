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


    }
}
