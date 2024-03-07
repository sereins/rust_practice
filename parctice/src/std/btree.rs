#[cfg(test)]
pub mod tests {
    use std::collections::BTreeSet;

    #[test]
    fn t_btree() {
        let mut b = BTreeSet::new();

        b.insert(4);
        b.insert(3);
        b.insert(2);
        b.insert(1);

        let mut iter = b.iter();
        while let Some(v) = iter.next() {
            println!("value = {:?}", v);
        }
    }
}
