use ws::listen;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn lister_ws(){

    let a =  listen(([127.0.0, 1], 3307), handel).unwrap();
}

fn handel(){

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
