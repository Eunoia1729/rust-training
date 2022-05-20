

fn main(){
    let nums = vec![13923, 3232, 322233];

    let target = 3232;
    let res = search::linear_search(&nums, &target);
    println!("{:?}", res);

    let res = search::binary_search(&nums, &target);
    println!("{:?}", res);

    let res = search::random_search(&nums, &target);
    println!("{:?}", res);
}