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

    fn check<T>(&mut self, a: &mut [T]) -> SortResult where T: PartialOrd + Clone {
        if a[self.j] < a[self.min] {
            self.min = self.j;
        }
        self.j += 1;
        SortResult::Swap(vec![self.j]) // makes this sort more visually interesting
    }

    fn next_pass(&mut self) {
        self.i += 1;
        self.j = self.i + 1;
        self.min = self.i;
    }

    fn swap_min<T>(&mut self, a: &mut [T]) -> SortResult where T: PartialOrd + Clone {
        let i = self.i;
        let min = self.min;
        swap(a, i, min);
        self.next_pass();
        SortResult::Swap(vec![i, min])
    }
}

impl<T> Sort<T> for SelectionSort where T: PartialOrd + Clone {
    fn step(&mut self, a: &mut [T]) -> SortResult {
        if self.i < a.len() - 1 {
            if self.j < a.len() {
                self.check(a)
            } else if self.min != self.i {
                self.swap_min(a)
            } else {
                self.next_pass();
                SortResult::Ok
            }
        } else {
            SortResult::Done
        }
    }
}