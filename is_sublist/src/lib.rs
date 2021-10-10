use crate::Comparison::{Sublist, Other, Superlist, Equal};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Comparison {
    Equal, // список `a` равен списку `b`
    Sublist, // список `a` является подсписком `b`
    Superlist, // список `b` является подсписком `a`
    Other, // списки не равны и не являются подсписками друг друга
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() > b.len() {
        return false;
    }
    (0..=b.len() - a.len()).any(|index| &b[index..index + a.len()] == a)
}

fn compare<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if is_sublist(a, b) {
        if a.len() == b.len() { Equal } else { Sublist }
    } else if is_sublist(b, a) {
        Superlist
    } else {
        Other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        assert_eq!(Equal, compare::<i32>(&[], &[]));
        assert_eq!(Equal, compare(&[1], &[1]));
        assert_eq!(Equal, compare(&[1, 2, 3], &[1, 2, 3]));
    }

    #[test]
    fn test_sublist() {
        assert_eq!(Sublist, compare(&[], &[1]));
        assert_eq!(Sublist, compare(&[], &[1, 2, 3]));
        assert_eq!(Sublist, compare(&[1], &[1, 2, 3]));
        assert_eq!(Sublist, compare(&[2], &[1, 2, 3]));
        assert_eq!(Sublist, compare(&[3], &[1, 2, 3]));
        assert_eq!(Sublist, compare(&[1, 2], &[1, 2, 3, 4, 5]));
        assert_eq!(Sublist, compare(&[2, 3, 4], &[1, 2, 3, 4, 5]));
        assert_eq!(Sublist, compare(&[4, 5], &[1, 2, 3, 4, 5]));
        assert_eq!(Sublist, compare(&[1, 2, 3], &[1, 2, 2, 1, 2, 3, 1, 2, 4]));
        assert_eq!(Sublist, compare(&[1, 1, 1], &[2, 1, 1, 1, 3]));
    }

    #[test]
    fn test_superlist() {
        assert_eq!(Superlist, compare(&[1], &[]));
        assert_eq!(Superlist, compare(&[1, 2, 3], &[]));
        assert_eq!(Superlist, compare(&[1, 2, 3], &[1]));
        assert_eq!(Superlist, compare(&[1, 2, 3], &[2]));
        assert_eq!(Superlist, compare(&[1, 2, 3], &[3]));
        assert_eq!(Superlist, compare(&[1, 2, 3, 4, 5], &[1, 2]));
        assert_eq!(Superlist, compare(&[1, 2, 3, 4, 5], &[2, 3, 4]));
        assert_eq!(Superlist, compare(&[1, 2, 3, 4, 5], &[4, 5]));
        assert_eq!(Superlist, compare(&[1, 2, 2, 1, 2, 3, 1, 2 ,4], &[1, 2, 3]));
        assert_eq!(Superlist, compare(&[2, 1, 1, 1, 3], &[1, 1, 1]));
    }

    #[test]
    fn test_other() {
        assert_eq!(Other, compare(&[1], &[2]));
        assert_eq!(Other, compare(&[1, 2, 4], &[1, 4]));
        assert_eq!(Other, compare(&[1, 4], &[1, 2, 4]));
        assert_eq!(Other, compare(&[1, 2, 3], &[3, 2, 1]));
        assert_eq!(Other, compare(&[f32::NAN], &[f32::NAN]));
    }
}
