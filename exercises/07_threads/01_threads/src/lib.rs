// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let len = v.len();
    let half_one: Vec<_> = v.iter().take(len / 2).copied().collect();
    let half_two: Vec<_> = v.iter().skip(len / 2).copied().collect();

    let handle_half_one = thread::spawn(|| {
        let mut sum_half_one = 0;
        for n in half_one {
            sum_half_one += n;
        }
        sum_half_one
    });

    let handle_half_two = thread::spawn(|| {
        let mut sum_half_two = 0;
        for n in half_two {
            sum_half_two += n;
        }
        sum_half_two
    });

    let sum_one = handle_half_one.join().unwrap();
    let sum_two = handle_half_two.join().unwrap();

    sum_one + sum_two
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
