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

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

// into_iter takes ownership over the shoes Vector
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
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

    #[test]
    fn iterator_adapters() {
        // Don't consume the iterator
        // Produce other iterators - change something w/ original iterator
        // ex: map

        let v1: Vec<i32> = vec![1, 2, 3];

        // .map does not consume the iterator
        // .collect here consumes the iterator and puts it into a vector
        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("Nike"),
            },
            Shoe {
                size: 13,
                style: String::from("Reebok"),
            },
            Shoe {
                size: 10,
                style: String::from("Adidas"),
            },
        ];

        let my_shoes = shoes_in_size(shoes, 10);

        assert_eq!(
            my_shoes,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("Nike"),
                },
                Shoe {
                    size: 10,
                    style: String::from("Adidas"),
                }
            ]
        );

        // println!("{shoes:?}"); // can't use shoes here. shoes_in_size took ownership
    }
}
