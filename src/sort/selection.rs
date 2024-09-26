use super::*;

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
                SortResult::Swap(vec![self.j]) // makes this sort more visually interesting
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