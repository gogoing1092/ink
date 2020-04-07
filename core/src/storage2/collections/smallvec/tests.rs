// Copyright 2019-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::SmallVec;
use generic_array::typenum::*;

#[test]
fn new_vec_works() {
    let vec = <SmallVec<i32, U4>>::new();
    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.get(0), None);
    assert!(vec.iter().next().is_none());
    let default = <SmallVec<i32, U4> as Default>::default();
    assert!(default.is_empty());
    assert_eq!(default.len(), 0);
    assert_eq!(vec.get(0), None);
    assert!(default.iter().next().is_none());
}

#[test]
fn from_iterator_works() {
    let some_primes = [b'A', b'B', b'C', b'D'];
    assert_eq!(some_primes.iter().copied().collect::<SmallVec<_, U4>>(), {
        let mut vec = SmallVec::new();
        for prime in &some_primes {
            vec.push(*prime)
        }
        vec
    });
}

#[test]
#[should_panic]
fn from_iterator_too_many() {
    let some_primes = [b'A', b'B', b'C', b'D', b'E'];
    let _ = some_primes.iter().copied().collect::<SmallVec<_, U4>>();
}

#[test]
fn from_empty_iterator_works() {
    assert_eq!(
        [].iter().copied().collect::<SmallVec<u8, U4>>(),
        SmallVec::new(),
    );
}

#[test]
fn first_last_of_empty() {
    let mut vec = <SmallVec<u8, U4>>::new();
    assert_eq!(vec.first(), None);
    assert_eq!(vec.first_mut(), None);
    assert_eq!(vec.last(), None);
    assert_eq!(vec.last_mut(), None);
}

#[test]
fn pop_on_empty_works() {
    let mut vec = <SmallVec<u8, U4>>::new();
    assert_eq!(vec.pop(), None);
}

#[test]
fn push_pop_first_last_works() {
    /// Asserts conditions are met for the given storage vector.
    fn assert_vec<F, L>(vec: &SmallVec<u8, U4>, len: u32, first: F, last: L)
    where
        F: Into<Option<u8>>,
        L: Into<Option<u8>>,
    {
        assert_eq!(vec.is_empty(), len == 0);
        assert_eq!(vec.len(), len);
        assert_eq!(vec.first().copied(), first.into());
        assert_eq!(vec.last().copied(), last.into());
    }

    let mut vec = SmallVec::new();
    assert_vec(&vec, 0, None, None);

    // Sequence of `push`
    vec.push(b'A');
    assert_vec(&vec, 1, b'A', b'A');
    vec.push(b'B');
    assert_vec(&vec, 2, b'A', b'B');
    vec.push(b'C');
    assert_vec(&vec, 3, b'A', b'C');
    vec.push(b'D');
    assert_vec(&vec, 4, b'A', b'D');

    // Sequence of `pop`
    assert_eq!(vec.pop(), Some(b'D'));
    assert_vec(&vec, 3, b'A', b'C');
    assert_eq!(vec.pop(), Some(b'C'));
    assert_vec(&vec, 2, b'A', b'B');
    assert_eq!(vec.pop(), Some(b'B'));
    assert_vec(&vec, 1, b'A', b'A');
    assert_eq!(vec.pop(), Some(b'A'));
    assert_vec(&vec, 0, None, None);

    // Pop from empty vector.
    assert_eq!(vec.pop(), None);
    assert_vec(&vec, 0, None, None);
}

#[test]
#[should_panic]
fn push_beyond_limits_fails() {
    let mut vec = [b'A', b'B', b'C', b'D']
        .iter()
        .copied()
        .collect::<SmallVec<_, U4>>();
    vec.push(b'E');
}

/// Creates a storage vector from the given slice.
fn vec_from_slice(slice: &[u8]) -> SmallVec<u8, U4> {
    slice.iter().copied().collect::<SmallVec<u8, U4>>()
}

/// Asserts that the the given ordered storage vector elements are equal to the
/// ordered elements of the given slice.
fn assert_eq_slice(vec: &SmallVec<u8, U4>, slice: &[u8]) {
    assert_eq!(vec.len() as usize, slice.len());
    let vec_copy = vec.iter().copied().collect::<Vec<u8>>();
    assert_eq!(vec_copy.as_slice(), slice);
}

#[test]
fn pop_drop_works() {
    let elems = [b'A', b'B', b'C', b'D'];
    let mut vec = vec_from_slice(&elems);
    assert_eq!(vec.pop_drop(), Some(()));
    assert_eq_slice(&vec, &elems[0..3]);
    assert_eq!(vec.pop_drop(), Some(()));
    assert_eq_slice(&vec, &elems[0..2]);
    assert_eq!(vec.pop_drop(), Some(()));
    assert_eq_slice(&vec, &elems[0..1]);
    assert_eq!(vec.pop_drop(), Some(()));
    assert_eq_slice(&vec, &[]);
    assert_eq!(vec.pop_drop(), None);
    assert_eq_slice(&vec, &[]);
}

#[test]
fn get_works() {
    let elems = [b'A', b'B', b'C', b'D'];
    let mut vec = vec_from_slice(&elems);
    for (n, mut expected) in elems.iter().copied().enumerate() {
        let n = n as u32;
        assert_eq!(vec.get(n), Some(&expected));
        assert_eq!(vec.get_mut(n), Some(&mut expected));
    }
    let len = vec.len();
    assert_eq!(vec.get(len), None);
    assert_eq!(vec.get_mut(len), None);
}

#[test]
fn iter_next_works() {
    let elems = [b'A', b'B', b'C', b'D'];
    let vec = vec_from_slice(&elems);
    // Test iterator over shared references.
    let mut iter = vec.iter();
    assert_eq!(iter.count(), 4);
    assert_eq!(iter.next(), Some(&b'A'));
    assert_eq!(iter.next(), Some(&b'B'));
    assert_eq!(iter.count(), 2);
    assert_eq!(iter.next(), Some(&b'C'));
    assert_eq!(iter.next(), Some(&b'D'));
    assert_eq!(iter.count(), 0);
    assert_eq!(iter.next(), None);
    // Test iterator over exclusive references.
    let mut vec = vec;
    let mut iter = vec.iter_mut();
    assert_eq!(iter.next(), Some(&mut b'A'));
    assert_eq!(iter.next(), Some(&mut b'B'));
    assert_eq!(iter.next(), Some(&mut b'C'));
    assert_eq!(iter.next(), Some(&mut b'D'));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.count(), 0);
}

#[test]
fn iter_nth_works() {
    let elems = [b'A', b'B', b'C', b'D'];
    let vec = vec_from_slice(&elems);
    // Test iterator over shared references.
    let mut iter = vec.iter();
    assert_eq!(iter.count(), 4);
    assert_eq!(iter.nth(1), Some(&b'B'));
    assert_eq!(iter.count(), 2);
    assert_eq!(iter.nth(1), Some(&b'D'));
    assert_eq!(iter.count(), 0);
    assert_eq!(iter.nth(1), None);
    // Test iterator over exclusive references.
    let mut vec = vec;
    let mut iter = vec.iter_mut();
    assert_eq!(iter.nth(1), Some(&mut b'B'));
    assert_eq!(iter.nth(1), Some(&mut b'D'));
    assert_eq!(iter.nth(1), None);
    assert_eq!(iter.count(), 0);
}

#[test]
fn iter_next_back_works() {
    let elems = [b'A', b'B', b'C', b'D'];
    let vec = vec_from_slice(&elems);
    // Test iterator over shared references.
    let mut iter = vec.iter().rev();
    assert_eq!(iter.clone().count(), 4);
    assert_eq!(iter.next(), Some(&b'D'));
    assert_eq!(iter.next(), Some(&b'C'));
    assert_eq!(iter.clone().count(), 2);
    assert_eq!(iter.next(), Some(&b'B'));
    assert_eq!(iter.next(), Some(&b'A'));
    assert_eq!(iter.clone().count(), 0);
    assert_eq!(iter.next(), None);
    // Test iterator over exclusive references.
    let mut vec = vec;
    let mut iter = vec.iter_mut().rev();
    assert_eq!(iter.next(), Some(&mut b'D'));
    assert_eq!(iter.next(), Some(&mut b'C'));
    assert_eq!(iter.next(), Some(&mut b'B'));
    assert_eq!(iter.next(), Some(&mut b'A'));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.count(), 0);
}

#[test]
fn iter_nth_back_works() {
    let elems = [b'A', b'B', b'C', b'D'];
    let vec = vec_from_slice(&elems);
    // Test iterator over shared references.
    let mut iter = vec.iter().rev();
    assert_eq!(iter.clone().count(), 4);
    assert_eq!(iter.nth(1), Some(&b'C'));
    assert_eq!(iter.clone().count(), 2);
    assert_eq!(iter.nth(1), Some(&b'A'));
    assert_eq!(iter.clone().count(), 0);
    assert_eq!(iter.nth(1), None);
    // Test iterator over exclusive references.
    let mut vec = vec;
    let mut iter = vec.iter_mut().rev();
    assert_eq!(iter.nth(1), Some(&mut b'C'));
    assert_eq!(iter.nth(1), Some(&mut b'A'));
    assert_eq!(iter.nth(1), None);
    assert_eq!(iter.count(), 0);
}

#[test]
fn swap_works() {
    let elems = [b'A', b'B', b'C', b'D'];
    let mut vec = vec_from_slice(&elems);

    // Swap at same position is a no-op.
    for index in 0..elems.len() as u32 {
        vec.swap(index, index);
        assert_eq_slice(&vec, &elems);
    }

    // Swap first and second
    vec.swap(0, 1);
    assert_eq_slice(&vec, &[b'B', b'A', b'C', b'D']);
    // Swap third and last
    vec.swap(2, 3);
    assert_eq_slice(&vec, &[b'B', b'A', b'D', b'C']);
    // Swap first and last
    vec.swap(0, 3);
    assert_eq_slice(&vec, &[b'C', b'A', b'D', b'B']);
}

#[test]
#[should_panic]
fn swap_one_invalid_index() {
    let mut vec = vec_from_slice(&[b'A', b'B', b'C', b'D']);
    vec.swap(0, vec.len());
}

#[test]
#[should_panic]
fn swap_both_invalid_indices() {
    let mut vec = vec_from_slice(&[b'A', b'B', b'C', b'D']);
    vec.swap(vec.len(), vec.len());
}

#[test]
fn swap_remove_works() {
    let mut vec = vec_from_slice(&[b'A', b'B', b'C', b'D']);

    // Swap remove first element.
    assert_eq!(vec.swap_remove(0), Some(b'A'));
    assert_eq_slice(&vec, &[b'D', b'B', b'C']);
    // Swap remove middle element.
    assert_eq!(vec.swap_remove(1), Some(b'B'));
    assert_eq_slice(&vec, &[b'D', b'C']);
    // Swap remove last element.
    assert_eq!(vec.swap_remove(1), Some(b'C'));
    assert_eq_slice(&vec, &[b'D']);
    // Swap remove only element.
    assert_eq!(vec.swap_remove(0), Some(b'D'));
    assert_eq_slice(&vec, &[]);
    // Swap remove from empty vector.
    assert_eq!(vec.swap_remove(0), None);
    assert_eq_slice(&vec, &[]);
}

#[test]
fn swap_remove_drop_works() {
    let mut vec = vec_from_slice(&[b'A', b'B', b'C', b'D']);

    // Swap remove first element.
    assert_eq!(vec.swap_remove_drop(0), Some(()));
    assert_eq_slice(&vec, &[b'D', b'B', b'C']);
    // Swap remove middle element.
    assert_eq!(vec.swap_remove_drop(1), Some(()));
    assert_eq_slice(&vec, &[b'D', b'C']);
    // Swap remove last element.
    assert_eq!(vec.swap_remove_drop(1), Some(()));
    assert_eq_slice(&vec, &[b'D']);
    // Swap remove only element.
    assert_eq!(vec.swap_remove_drop(0), Some(()));
    assert_eq_slice(&vec, &[]);
    // Swap remove from empty vector.
    assert_eq!(vec.swap_remove_drop(0), None);
    assert_eq_slice(&vec, &[]);
}
