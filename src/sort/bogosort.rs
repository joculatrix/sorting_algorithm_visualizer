use super::*;

pub struct BogoSort {}

impl BogoSort {
    pub fn new() -> BogoSort {
        BogoSort {}
    }
}

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