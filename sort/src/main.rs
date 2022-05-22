use std::cmp::*;
fn partition<T: PartialOrd>(data: &mut [T], l: usize, r: usize) -> usize {
    let mid_index = l + (r - l) / 2;
    
    let mut cur_i = l;
    data.swap(r, mid_index);
    for i in l..r {
        if data[i] < data[r] {
            data.swap(i, cur_i);
            cur_i += 1; 
        }
    }
    data.swap(cur_i, r);
    cur_i
}

fn quick_sort<T: PartialOrd>(data: &mut [T], l: usize, r: usize) {
    let pivot_index;
    if l < r  { 
        pivot_index = partition(data, l, r);
        if pivot_index > 0 {
            quick_sort(data, l, pivot_index - 1);
        }
        quick_sort(data, pivot_index + 1, r);
    }
}
fn sort<T: PartialOrd>(data: &mut [T]) {
    if data.is_empty() {
        ()
    }
    quick_sort(data, 0, data.len() - 1);
}
fn main() {
    let mut nums = vec![4, 2, 66, 1];
    sort(&mut nums);
    println!("{:?}", nums);
}
