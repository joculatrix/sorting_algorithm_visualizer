use super::*;

#[derive(PartialEq)]
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
}

impl<T> Sort<T> for CocktailShakerSort where T: PartialOrd + Clone {
    fn step(&mut self, a: &mut [T]) -> SortResult {
        let i = self.i;
        if i < a.len() - (1 + self.end_pad) && self.direction == ShakerDirection::Forward {
            if a[i] > a[i + 1] {
                swap(a, i, i + 1);
                self.swapped = true;
                self.i += 1;
                SortResult::Swap(vec![i, i + 1])
            } else {
                self.i += 1;
                SortResult::Ok
            }
        } else if i > self.start_pad && self.direction == ShakerDirection::Backward {
            if a[i] < a[i - 1] {
                swap(a, i, i - 1);
                self.swapped = true;
                self.i -= 1;
                SortResult::Swap(vec![i, i - 1])
            } else {
                self.i -= 1;
                SortResult::Ok
            }
        } else if !self.swapped {
            SortResult::Done
        } else {
            self.swapped = false;
            match self.direction {
                ShakerDirection::Forward => {
                    self.direction = ShakerDirection::Backward;
                    self.end_pad += 1;
                    SortResult::Ok
                }
                ShakerDirection::Backward => {
                    self.direction = ShakerDirection::Forward;
                    self.start_pad += 1;
                    SortResult::Ok
                }
            }
        }
    }
}