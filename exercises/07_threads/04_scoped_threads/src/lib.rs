// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    if v.is_empty() {
        return 0;
    }

    let mid = v.len() / 2;
    let (left, right) = v.split_at(mid);

    thread::scope(|scope| {
        let (left_handle, right_handle) = (
            scope.spawn(|| left.iter().sum::<i32>()),
            scope.spawn(|| right.iter().sum::<i32>()),
        );

        left_handle.join().unwrap() + right_handle.join().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[test]
    fn test() {
        let data = RefCell::new(5);

        // 不変な借用（読み取り）
        let borrowed = data.borrow();
        assert_eq!(5, *borrowed);

        // 可変な借用（書き込み）
        let mut borrowed_mut = data.borrow_mut();
        *borrowed_mut += 10;
        assert_eq!(15, *borrowed_mut);

        assert_eq!(15, *data.borrow());
    }

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
