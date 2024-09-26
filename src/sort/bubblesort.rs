use super::*;

pub struct BubbleSort {
    swapped: bool,
    i: usize,
    n: usize,
}

impl BubbleSort {
    pub fn new() -> BubbleSort {
        BubbleSort {
            swapped: false,
            i: 1,
            n: 0,
        }
    }

    fn next_pass(&mut self) {
        self.i = 1;
        self.swapped = false;
        self.n += 1;
    }
}

impl<T> Sort<T> for BubbleSort where T: PartialOrd + Clone {
    fn step(&mut self, a: &mut [T]) -> SortResult {
        let i = self.i;
        let j = self.i - 1;

        if i < a.len() - self.n {
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
            self.next_pass();
            SortResult::Ok
        }
    }
}