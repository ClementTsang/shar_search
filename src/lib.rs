//! Implements Shar's algorithm in Rust for a theoretically branchless binary search. Inspired by
//! [Beautiful Branchless Binary Search](https://probablydance.com/2023/04/27/beautiful-branchless-binary-search/) and
//! [Beautiful Binary Search in D](https://muscar.eu/shar-binary-search-meta.html).

#![deny(missing_docs)]

use std::cmp::Ordering;

/// Trait for using Shar's binary search.
pub trait SharBinarySearch<T> {
    /// Binary searches this slice with a comparator function. Note it is assumed that the slice it is sorted.
    ///
    /// Note that if there are multiple matches, then the *first*
    /// match will be returned. This is different from how
    /// [`binary_search_by`](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search_by)
    /// works!
    fn bl_binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        T: 'a,
        F: FnMut(&'a T) -> Ordering;

    /// Binary searches this slice for a given element. Note it is assumed that the slice it is sorted.
    ///
    /// Note that if there are multiple matches, then the *first*
    /// match will be returned. This is different from how
    /// [`binary_search`](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search)
    /// works!
    #[inline]
    fn bl_binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.bl_binary_search_by(|p| p.cmp(x))
    }

    /// Binary searches this slice with a key extraction function. Note it is assumed that the slice it is sorted.
    ///
    /// Note that if there are multiple matches, then the *first*
    /// match will be returned. This is different from how
    /// [`binary_search_by_key`](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search_by_key)
    /// works!
    #[inline]
    fn bl_binary_search_by_key<'a, B, F>(&'a self, b: &B, mut f: F) -> Result<usize, usize>
    where
        T: 'a,
        F: FnMut(&'a T) -> B,
        B: Ord,
    {
        self.bl_binary_search_by(|k| f(k).cmp(b))
    }
}

/// Note: this cannot be called with `length = 0`!
#[inline]
const fn bit_floor(length: usize) -> usize {
    1_usize << (usize::BITS - length.leading_zeros() - 1)
}

impl<T> SharBinarySearch<T> for [T] {
    #[inline]
    fn bl_binary_search_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> Ordering,
    {
        if self.is_empty() {
            return Err(0);
        }

        let mut length = self.len();

        let mut left = 0;
        let right = length;

        let mut step = bit_floor(length);

        if step != length && f(unsafe { self.get_unchecked(step) }).is_lt() {
            length -= step + 1;

            if length == 0 {
                return Err(right);
            }

            step = length.next_power_of_two();
            left = right - step;
        }

        // TODO: This needs to loop unroll... bleh.
        loop {
            step /= 2;
            if step == 0 {
                break;
            } else if f(unsafe { self.get_unchecked(left + step) }).is_lt() {
                left += step;
            }
        }

        match f(unsafe { self.get_unchecked(left) }) {
            Ordering::Less => {
                if left + 1 >= self.len() {
                    Err(left + 1)
                } else {
                    match f(unsafe { self.get_unchecked(left + 1) }) {
                        Ordering::Less => Err(left + 1),
                        Ordering::Equal => Ok(left + 1),
                        Ordering::Greater => Err(left + 1),
                    }
                }
            }
            Ordering::Equal => Ok(left),
            Ordering::Greater => Err(left),
        }
    }
}

/// Tests taken from std.
#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use crate::{bit_floor, SharBinarySearch};

    #[test]
    fn test_bit_floor() {
        assert_eq!(bit_floor(1), 1);
        assert_eq!(bit_floor(2), 2);
        assert_eq!(bit_floor(3), 2);
        assert_eq!(bit_floor(4), 4);
        assert_eq!(bit_floor(5), 4);
        assert_eq!(bit_floor(20), 16);
        assert_eq!(bit_floor(100), 64);
        assert_eq!(bit_floor(255), 128);
        assert_eq!(bit_floor(256), 256);
        assert_eq!(bit_floor(257), 256);
        assert_eq!(bit_floor(1000), 512);
        assert_eq!(bit_floor(1024), 1024);
        assert_eq!(bit_floor(1025), 1024);
    }

    #[test]
    fn test_binary_search() {
        let b: [i32; 0] = [];
        assert_eq!(b.bl_binary_search(&5), Err(0));

        let b = [4];
        assert_eq!(b.bl_binary_search(&3), Err(0));
        assert_eq!(b.bl_binary_search(&4), Ok(0));
        assert_eq!(b.bl_binary_search(&5), Err(1));

        let b = [1, 2, 4, 6, 8, 9];
        assert_eq!(b.bl_binary_search(&5), Err(3));
        assert_eq!(b.bl_binary_search(&6), Ok(3));
        assert_eq!(b.bl_binary_search(&7), Err(4));
        assert_eq!(b.bl_binary_search(&8), Ok(4));

        let b = [1, 2, 4, 5, 6, 8];
        assert_eq!(b.bl_binary_search(&9), Err(6));

        let b = [1, 2, 4, 6, 7, 8, 9];
        assert_eq!(b.bl_binary_search(&6), Ok(3));
        assert_eq!(b.bl_binary_search(&5), Err(3));
        assert_eq!(b.bl_binary_search(&8), Ok(5));

        let b = [1, 2, 4, 5, 6, 8, 9];
        assert_eq!(b.bl_binary_search(&7), Err(5));
        assert_eq!(b.bl_binary_search(&0), Err(0));

        let b = [1, 3, 3, 3, 7];
        assert_eq!(b.bl_binary_search(&0), Err(0));
        assert_eq!(b.bl_binary_search(&1), Ok(0));
        assert_eq!(b.bl_binary_search(&2), Err(1));
        assert!(match b.bl_binary_search(&3) {
            Ok(1..=3) => true,
            _ => false,
        });
        assert!(match b.bl_binary_search(&3) {
            Ok(1..=3) => true,
            _ => false,
        });
        assert_eq!(b.bl_binary_search(&4), Err(4));
        assert_eq!(b.bl_binary_search(&5), Err(4));
        assert_eq!(b.bl_binary_search(&6), Err(4));
        assert_eq!(b.bl_binary_search(&7), Ok(4));
        assert_eq!(b.bl_binary_search(&8), Err(5));

        let b = [(); usize::MAX];
        assert_eq!(b.bl_binary_search(&()), Ok(0));
    }

    #[test]
    fn test_binary_search_by_overflow() {
        let b = [(); usize::MAX];
        assert_eq!(b.bl_binary_search_by(|_| Ordering::Equal), Ok(0));
        assert_eq!(b.bl_binary_search_by(|_| Ordering::Greater), Err(0));
        assert_eq!(b.bl_binary_search_by(|_| Ordering::Less), Err(usize::MAX));
    }

    #[test]
    // Test implementation specific behavior when finding equivalent elements.
    fn test_binary_search_implementation_details() {
        let b = [1, 1, 2, 2, 3, 3, 3];
        assert_eq!(b.bl_binary_search(&1), Ok(0));
        assert_eq!(b.bl_binary_search(&2), Ok(2));
        assert_eq!(b.bl_binary_search(&3), Ok(4));
        let b = [1, 1, 1, 1, 1, 3, 3, 3, 3];
        assert_eq!(b.bl_binary_search(&1), Ok(0));
        assert_eq!(b.bl_binary_search(&3), Ok(5));
        let b = [1, 1, 1, 1, 3, 3, 3, 3, 3];
        assert_eq!(b.bl_binary_search(&1), Ok(0));
        assert_eq!(b.bl_binary_search(&3), Ok(4));
    }

    #[test]
    fn test_binary_search_lifetime() {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct Assignment {
            topic: String,
            partition: i32,
        }

        let xs = vec![
            Assignment {
                topic: "abc".into(),
                partition: 1,
            },
            Assignment {
                topic: "def".into(),
                partition: 2,
            },
            Assignment {
                topic: "ghi".into(),
                partition: 3,
            },
        ];

        let key: &str = "def";
        let r = xs.bl_binary_search_by_key(&key, |e| &e.topic);
        assert_eq!(Ok(1), r.map(|i| i));
    }
}
