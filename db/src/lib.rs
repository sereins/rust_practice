pub mod sql;

pub fn li() {
    let a = Box::new(2);
    let c = *a;

    println!("{}", c)
}

#[test]
pub fn a(){
    li()
}
