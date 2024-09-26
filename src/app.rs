use crate::sort::*;

#[derive(PartialEq)]
pub enum AppScreen {
    Menu,
    Sort,
}

pub struct Algorithm {
    pub name: &'static str,
    pub new: fn() -> Box<dyn Sort<usize>>,
}

pub struct App {
    pub data: [usize; crate::ARRAY_LEN],
    pub current_screen: AppScreen,
    pub algorithms: Vec<Algorithm>,
    pub selected: usize,
    pub sort: Option<Box<dyn Sort<usize>>>,
    pub swapped: Vec<usize>,
}

impl App {
    pub fn new() -> App {
        let mut algorithms: Vec<Algorithm> = vec![
                Algorithm {
                    name: "Bogosort",
                    new: || Box::new(BogoSort::new())
                },
                Algorithm {
                    name: "Bubble sort",
                    new: || Box::new(BubbleSort::new())
                },
                Algorithm {
                    name: "Cocktail shaker sort",
                    new: || Box::new(CocktailShakerSort::new())
                },
                Algorithm {
                    name: "Insertion sort",
                    new: || Box::new(InsertionSort::new())
                },
                Algorithm {
                    name: "Quicksort",
                    new: || Box::new(QuickSort::new())
                },
                Algorithm {
                    name: "Selection sort",
                    new: || Box::new(SelectionSort::new())
                },
        ];

        App {
            data: core::array::from_fn(|i| i + 1),
            algorithms,
            current_screen: AppScreen::Menu,
            selected: 0,
            sort: None,
            swapped: vec![],
        }
    }
}