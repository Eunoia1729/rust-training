#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct User {
    height: u32,
    weight: u32,
    name: String,
}

fn main(){
    let nums = vec![13923, 3232, 322233];

    let target = 3232;
    let res = search::linear_search(&nums, &target);
    println!("{:?}", res);

    let res = search::binary_search(&nums, &target);
    println!("{:?}", res);

    let res = search::random_search(&nums, &target);
    println!("{:?}", res);

    let users = vec![
        User{
            height: 160,
            weight: 70,
            name: String::from("prince"),
        },
        User{
            height: 150,
            weight: 60,
            name: String::from("henry"),
        },
        User{
            height: 180,
            weight: 66,
            name: String::from("alice"),
        },
    ];
    let target = User{
        height: 1600,
        weight: 70,
        name: String::from("prince"),
    };
    let res = search::linear_search(&users, &target);
    match res {
        Some(i) => println!("{}", users[i].name),
        None => println!("No match found!"),
    }
}