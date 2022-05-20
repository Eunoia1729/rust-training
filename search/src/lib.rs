use std::cmp::*;
use rand::Rng;

pub fn linear_search<T: Eq>(data: &Vec<T>, target: &T) -> Option<usize> {
    for (i, val) in data.iter().enumerate() {
        if val == target {
            return Some(i)
        }
    }
    None
}

pub fn binary_search<T: Ord>(data: &Vec<T>, target: &T) -> Option<usize> {
    if data.is_empty() {
        return None;
    }
    let mut l = 0;
    let mut r = data.len() - 1;
    let mut mid;

    while l <= r {
        mid = l + (r - l) / 2;
        match target.cmp(&data[mid]) {
            Ordering::Less =>  r = mid - 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => l = mid + 1,
        }
    }
    None
}

pub fn random_search<T: Ord>(data: &Vec<T>, target: &T) -> Option<usize> {
    if data.is_empty() {
        return None;
    }
    let mut rng = rand::thread_rng();
    let mut i;
    const MAX_TRIES: i32 = 100000;
    let mut tries = 0;
    while tries <=  MAX_TRIES {
        i = rng.gen_range(0..data.len());        
        if &data[i] == target {
            return Some(i)
        }
        tries += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn linear_search_for_empty_list() {
        let res = linear_search(&vec![], &5);
        assert!(res.is_none());
    }

    #[test]
    fn binary_search_for_empty_list() {
        let res = binary_search(&vec![], &5);
        assert!(res.is_none());
    }

    #[test]
    fn random_search_for_empty_list() {
        let res = random_search(&vec![], &5);
        assert!(res.is_none());
    }

    #[test]
    fn linear_search_for_list_with_target_present() {
        let res = linear_search(&vec![2, 5, 22, 56, 100], &22);
        assert_eq!(res, Some(2));
    }

    #[test]
    fn binary_search_for_list_with_target_present() {
        let res = binary_search(&vec![2, 5, 22, 56, 100], &22);
        assert_eq!(res, Some(2));
    }

    #[test]
    fn random_search_for_list_with_target_present() {
        let res = random_search(&vec![2, 5, 22, 56, 100], &22);
        assert_eq!(res, Some(2));
    }

    #[test]
    fn linear_search_for_list_with_target_absent() {
        let res = linear_search(&vec![2, 5, 21, 56, 100], &22);
        assert!(res.is_none());
    }

    #[test]
    fn binary_search_for_list_with_target_absent() {
        let res = binary_search(&vec![2, 5, 21, 56, 100], &22);
        assert!(res.is_none());
    }

    #[test]
    fn random_search_for_list_with_target_absent() {
        let res = random_search(&vec![2, 5, 21, 56, 100], &22);
        assert!(res.is_none());
    }

    #[test]
    fn linear_search_for_strings_list() {
        let target = "cat";
        let res = random_search(&vec!["tia", "tamera", "cat"], &target);
        assert_eq!(res, Some(2));
    }
}

/*
TODO: Tests to add for:
1. multiple targets exists
2. string match
*/