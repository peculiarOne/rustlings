// iterators4.rs

pub fn factorial_rec(num: u64) -> u64 {
    // Complete this function to return factorial_rec of num
    // Do not use:
    // - return
    // For extra fun don't use:
    // - imperative style loops (for, while)
    // - additional variables
    // For the most fun don't use:
    // - recursion
    // Scroll down for hints.

    fn inner_fact(num: u64, acc: u64) -> u64 {
        match num {
            0 => acc,
            _ => inner_fact(num -1, num * acc)
        }
    }
    inner_fact(num, 1)
}

pub fn factorial(num: u64) -> u64 {
    (1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial_rec(1));
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial_rec(2));
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial_rec(4));
        assert_eq!(24, factorial(4));
    }
}

























// In an imperative language you might write a for loop to iterate through
// multiply the values into a mutable variable. Or you might write code more
// functionally with recursion and a match clause. But you can also use ranges
// and iterators to solve this in rust.
