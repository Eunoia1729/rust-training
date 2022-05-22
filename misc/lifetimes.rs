struct Human {
    age: u32,
}
impl Human {
    fn get_age(&self) -> u32 {
        self.age
    }

    fn get_name<'a>(&'a self, name: &'a str) -> &'a str {
        let new_name = "jai";
        if new_name.len() > name.len() {
            new_name
        } else {
            name
        }
    }
}
fn main() {
    let aman = Human { age: 12 };

    println!("{}", aman.age);
    println!("{}", aman.get_age());
    let some_name = "meena";
    println!("{}", aman.get_name(some_name));
}
