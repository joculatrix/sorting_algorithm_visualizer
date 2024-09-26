use super::*;

enum ShakerDirection { Forward, Backward }

pub struct CocktailShakerSort {
    direction: ShakerDirection,
    i: usize,
    swapped: bool,
    start_pad: usize,
    end_pad: usize,
}

impl CocktailShakerSort {
    pub fn new() -> CocktailShakerSort {
        CocktailShakerSort {
            direction: ShakerDirection::Forward,
            i: 0,
            swapped: false,
            start_pad: 0,
            end_pad: 0,
        }
    }

    fn forward_pass<T>(&mut self, a: &mut [T]) -> SortResult where T: PartialOrd + Clone {
        let i = self.i;
        if a[i] > a[i + 1] {
            swap(a, i, i + 1);
            self.swapped = true;
            self.i += 1;
            SortResult::Swap(vec![i, i + 1])
        } else {
            self.i += 1;
            SortResult::Ok
        }
    }

    fn backward_pass<T>(&mut self, a: &mut [T]) -> SortResult where T: PartialOrd + Clone {
        let i = self.i;
        if a[i] < a[i - 1] {
            swap(a, i, i - 1);
            self.swapped = true;
            self.i -= 1;
            SortResult::Swap(vec![i, i - 1])
        } else {
            self.i -= 1;
            SortResult::Ok
        }
    }

    fn switch_direction(&mut self) {
        self.swapped = false;
        match self.direction {
            ShakerDirection::Forward => {
                self.direction = ShakerDirection::Backward;
                self.end_pad += 1;
            }
            ShakerDirection::Backward => {
                self.direction = ShakerDirection::Forward;
                self.start_pad += 1;
            }
        }
    }
}

impl<T> Sort<T> for CocktailShakerSort where T: PartialOrd + Clone {
    fn step(&mut self, a: &mut [T]) -> SortResult {
        let i = self.i;
        match self.direction {
            ShakerDirection::Forward if i < a.len() - (1 + self.end_pad) => {
                self.forward_pass(a)
            }
            ShakerDirection::Backward if i > self.start_pad => {
                self.backward_pass(a)
            }
            _ if !self.swapped => SortResult::Done, // pass completed without swapping anything
            _ => { // next pass
                self.switch_direction();
                SortResult::Ok
            }
        }
    }
}