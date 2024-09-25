// Sorting algorithms are implemented in an unconventional non-looping, incomplete way wherein each
// function call, rather than resulting in a fully sorted list, only progresses the sort by one
// "step"... this makes it easier to integrate with the UI render loop.

use std::{cell::RefCell, rc::Rc};

mod bogosort;
pub use bogosort::BogoSort;

mod bubblesort;
pub use bubblesort::BubbleSort;

mod cocktail;
pub use cocktail::CocktailShakerSort;

mod insertion;
pub use insertion::InsertionSort;

mod quicksort;
pub use quicksort::QuickSort;

mod selection;
pub use selection::SelectionSort;

pub enum SortResult {
    Done,
    Swap(Vec<usize>),
    Ok,
}

pub trait Sort<T: PartialOrd + Clone> {
    // represents a single step of an algorithm:
    fn step(&mut self, a: &mut [T]) -> SortResult;
}

fn is_sorted<T>(data: &[T]) -> bool where T: PartialOrd {
    for i in 0..data.len() - 2 {
        if data[i] > data[i + 2] {
            return false;
        }
    }
    true
}

fn swap<T>(a: &mut [T], i: usize, j: usize) where T: Clone {
    let temp = a[j].clone();
    a[j] = a[i].clone();
    a[i] = temp;
}

pub fn shuffle<T>(a: &mut [T]) where T: Clone {
    for i in 0..a.len() {
        let random = rand::random::<usize>() % a.len();
        let temp = a[random].clone();
        a[random] = a[i].clone();
        a[i] = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ARRAY_LEN;

    #[test]
    #[ignore = "takes forever"]
    fn bogosort() {
        let mut array: [usize ; ARRAY_LEN] = core::array::from_fn(|i| i + 1);
        shuffle(&mut array);
        let mut sort = BogoSort {};

        loop {
            match sort.step(&mut array) {
                SortResult::Done => break,
                _ => ()
            }
        }

        assert!(is_sorted(&array))
    }

    #[test]
    fn bubble_sort() {
        let mut array: [usize; ARRAY_LEN] = core::array::from_fn(|i| i + 1);
        shuffle(&mut array);
        let mut sort = BubbleSort::new();

        loop {
            match sort.step(&mut array) {
                SortResult::Done => break,
                _ => ()
            }
        }

        assert!(is_sorted(&array))
    }

    #[test]
    fn cocktail_shaker_sort() {
        let mut array: [usize; ARRAY_LEN] = core::array::from_fn(|i| i + 1);
        shuffle(&mut array);
        let mut sort = CocktailShakerSort::new();

        loop {
            match sort.step(&mut array) {
                SortResult::Done => break,
                _ => ()
            }
        }

        assert!(is_sorted(&array))
    }

    #[test]
    fn insertion_sort() {
        let mut array: [usize; ARRAY_LEN] = core::array::from_fn(|i| i + 1);
        shuffle(&mut array);
        let mut sort = InsertionSort::new();

        loop {
            match sort.step(&mut array) {
                SortResult::Done => break,
                _ => ()
            }
        }

        assert!(is_sorted(&array))
    }

    #[test]
    fn selection_sort() {
        let mut array: [usize; ARRAY_LEN] = core::array::from_fn(|i| i + 1);
        shuffle(&mut array);
        let mut sort = SelectionSort::new();

        loop {
            match sort.step(&mut array) {
                SortResult::Done => break,
                _ => ()
            }
        }

        assert!(is_sorted(&array))
    }

    #[test]
    fn quicksort() {
        let mut array: [usize; ARRAY_LEN] = core::array::from_fn(|i| i + 1);
        shuffle(&mut array);
        let mut sort = QuickSort::new();

        loop {
            match sort.step(&mut array) {
                SortResult::Done => break,
                _ => ()
            }
        }

        assert!(is_sorted(&array))
    }
}