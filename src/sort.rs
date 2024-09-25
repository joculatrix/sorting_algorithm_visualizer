// Sorting algorithms are implemented in an unconventional non-looping, incomplete way wherein each
// function call, rather than resulting in a fully sorted list, only progresses the sort by one
// "step"... this makes it easier to integrate with the UI render loop.

use std::{cell::RefCell, rc::Rc};

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

pub struct BogoSort {}

impl<T> Sort<T> for BogoSort where T: PartialOrd + Clone {
    fn step(&mut self, a: &mut [T]) -> SortResult {
        shuffle(a);

        if is_sorted(a) {
            SortResult::Done
        } else {
            SortResult::Ok
        }
    }
}

pub struct BubbleSort {
    swapped: bool,
    i: usize,
}

impl BubbleSort {
    pub fn new() -> BubbleSort {
        BubbleSort {
            swapped: false,
            i: 1,
        }
    }
}

impl<T> Sort<T> for BubbleSort where T: PartialOrd + Clone {
    fn step(&mut self, a: &mut [T]) -> SortResult {
        let i = self.i;
        let j = self.i - 1;

        if i < a.len() {
            if a[j] > a[i] {
                swap(a, i, j);
                self.swapped = true;
                self.i += 1;
                SortResult::Swap(vec![i, j])
            } else {
                self.i += 1;
                SortResult::Ok
            }
        } else if !self.swapped {
            SortResult::Done
        } else {
            self.i = 1;
            self.swapped = false;
            SortResult::Ok
        }
    }
}

pub struct InsertionSort {
    i: usize,
    j: usize,
}

impl InsertionSort {
    pub fn new() -> InsertionSort {
        InsertionSort {
            i: 1,
            j: 1,
        }
    }
}

impl<T> Sort<T> for InsertionSort where T: PartialOrd + Clone {
    fn step(&mut self, a: &mut [T]) -> SortResult {
        if self.i < a.len() {
            if self.j > 0 && a[self.j - 1] > a[self.j] {
                let j = self.j;
                swap(a, j, j - 1);
                self.j -= 1;
                SortResult::Swap(vec![j, j - 1])
            } else {
                self.i += 1;
                self.j = self.i;
                SortResult::Ok
            }
        } else {
            SortResult::Done
        }
    }
}

pub struct SelectionSort {
    i: usize,
    j: usize,
    min: usize,
}

impl SelectionSort {
    pub fn new() -> SelectionSort {
        SelectionSort {
            i: 0,
            j: 1,
            min: 0,
        }
    }
}

impl<T> Sort<T> for SelectionSort where T: PartialOrd + Clone {
    fn step(&mut self, a: &mut [T]) -> SortResult {
        if self.i < a.len() - 1 {
            if self.j < a.len() {
                if a[self.j] < a[self.min] {
                    self.min = self.j;
                }
                self.j += 1;
                SortResult::Ok
            } else if self.min != self.i {
                let i = self.i;
                let min = self.min;
                swap(a, i, min);
                self.i += 1;
                self.j = self.i + 1;
                self.min = self.i;
                SortResult::Swap(vec![i, min])
            } else {
                self.i += 1;
                self.j = self.i + 1;
                self.min = self.i;
                SortResult::Ok
            }
        } else {
            SortResult::Done
        }
    }
}

struct Partition<T> where T: PartialOrd + Clone {
    start: usize,
    end: usize,
    sort: QuickSort<T>,
}

impl<T> Partition<T> where T: PartialOrd + Clone {
    fn new(start: usize, end: usize) -> Partition<T> {
        Partition {
            start,
            end,
            sort: QuickSort::new(),
        }
    }

    fn step(&mut self, a: &mut [T]) -> SortResult {
        self.sort.step(a)
    }
}

pub struct QuickSort<T> where T: PartialOrd + Clone {
    partitions: Option<(Rc<RefCell<Partition<T>>>, Rc<RefCell<Partition<T>>>)>,
    pivot: Option<T>,
    i: usize,
    j: usize,
}

impl<T> QuickSort<T> where T: PartialOrd + Clone {
    pub fn new() -> QuickSort<T> {
        QuickSort {
            partitions: None,
            pivot: None,
            i: 0,
            j: 0,
        }
    }
}

impl<T> Sort<T> for QuickSort<T> where T: PartialOrd + Clone {
    fn step(&mut self, a: &mut [T]) -> SortResult {
        if a.len() <= 1 {
            SortResult::Done
        } else {
            if let Some((p1, p2)) = &mut self.partitions {
                let (p1_start, p1_end) = (p1.borrow().start, p1.borrow().end);
                let (p2_start, p2_end) = (p2.borrow().start, p2.borrow().end);
                match (
                    p1.borrow_mut().step(&mut a[p1_start..p1_end]),
                    p2.borrow_mut().step(&mut a[p2_start..p2_end])
                ) {
                    (SortResult::Done, SortResult::Done) => SortResult::Done,
                    (SortResult::Swap(mut a), SortResult::Swap(mut b)) => {
                        a.append(&mut b);
                        SortResult::Swap(a)
                    }
                    (SortResult::Swap(a), _) | (_, SortResult::Swap(a)) => SortResult::Swap(a),
                    _ => SortResult::Ok,
                }
            } else if let Some(pivot) = &self.pivot {
                if self.j < a.len() - 1 {
                    if a[self.j] <= *pivot {
                        let i = self.i;
                        let j = self.j;
                        swap(a, i, j);
                        self.i += 1;
                        self.j += 1;
                        SortResult::Swap(vec![i, j])
                    } else {
                        self.j += 1;
                        SortResult::Ok
                    }
                } else {
                    let i = self.i;
                    let j = a.len() - 1;
                    swap(a, i, j);

                    self.partitions = Some((
                        Rc::new(RefCell::new(Partition::new(0, i))),
                        Rc::new(RefCell::new(Partition::new(i + 1, a.len()))),
                    ));

                    SortResult::Swap(vec![self.i, a.len() - 1])
                }
            } else {
                self.pivot = Some(a[a.len() - 1].clone());
                SortResult::Ok
            }
        }
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