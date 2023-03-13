
#[cfg(test)]
mod tests {

    #[test]
    fn tests(){
        let a = None;
        let b:i8 = a.unwrap_or_default();
        println!("value is {}",b);
    }
}
