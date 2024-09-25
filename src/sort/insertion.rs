use super::*;

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