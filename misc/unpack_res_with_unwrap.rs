use std::io::{Error, ErrorKind};

fn halves_if_even(i: i32) -> Result<i32, Error> {
    if i % 2 == 0 {
        Ok(i / 2)
    } else {
        Err( Error::new(ErrorKind::Other, "oh no!") )
    }
}

fn do_the_thing(i: i32) -> Result<i32, Error> {
    // let i = match halves_if_even(i) {
    //     Ok(i) => i,
    //     Err(e) => return Err(e),
    // };
    // Ok(i)
    let x = halves_if_even(i).unwrap();
    Ok(x)
}
fn main() {
    let res = do_the_thing(5);
    match res {
        Ok(i) => println!("yayy it can be halved to {}", i),
        Err(e) => println!("in main: {}", e),
    }
    
}