// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let v = v.leak();

    if v.is_empty() {
        return 0;
    }

    let mid = v.len() / 2;
    let (left, right) = v.split_at(mid);

    let (left_handle, right_handle) = (
        thread::spawn(move || left.iter().sum::<i32>()),
        thread::spawn(move || right.iter().sum::<i32>()),
    );

    let (left_sum, right_sum) = (left_handle.join().unwrap(), right_handle.join().unwrap());

    left_sum + right_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
