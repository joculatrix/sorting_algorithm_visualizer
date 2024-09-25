use super::*;

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