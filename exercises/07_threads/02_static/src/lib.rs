// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    let half_one = &slice[..(slice.len() / 2)];
    let half_two = &slice[(slice.len() / 2)..];

    let handle_half_one = thread::spawn(move || {
        let mut sum = 0;
        for n in half_one {
            sum += n;
        }
        sum
    });

    let handle_half_two = thread::spawn(move || {
        let mut sum = 0;
        for n in half_two {
            sum += n;
        }
        sum
    });

    let sum_half_one = handle_half_one.join().unwrap();
    let sum_half_two = handle_half_two.join().unwrap();

    sum_half_one + sum_half_two
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
