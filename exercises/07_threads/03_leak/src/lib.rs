// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let len = v.len();
    let leaked = v.leak();

    let half_one = &leaked[..(len / 2)];
    let half_two = &leaked[(len / 2)..];

    let half_one_handle = thread::spawn(move || {
        let mut sum = 0;
        for n in half_one {
            sum += n;
        }
        sum
    });

    let half_two_handle = thread::spawn(move || {
        let mut sum = 0;
        for n in half_two {
            sum += n;
        }
        sum
    });

    let half_one_sum = half_one_handle.join().unwrap();
    let half_two_sum = half_two_handle.join().unwrap();

    half_one_sum + half_two_sum
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
