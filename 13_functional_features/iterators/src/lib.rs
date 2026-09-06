fn _intro() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for n in v1_iter {
        println!("{n}");
    }
}

// What Iterator trait's definition is like:
pub trait MyIterator {
    type Item; // defines a type to this trait

    // To implement, only need to have "next" method
    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations...
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intro() {
        _intro();
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();
        // let mut _ownership_iter = v1.into_iter();
        // let mut _mutable_iter = v1.iter_mut();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        // sum here takes ownership of iterator (can't use v1_iter again)
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    // TODO: left off https://doc.rust-lang.org/book/ch13-02-iterators.html#methods-that-produce-other-iterators

}
