use std::collections::HashMap;

use crate::sort::*;

pub enum AppScreen {
    Menu,
    Sort,
}

pub struct Algorithm {
    pub name: &'static str,
    pub new: fn() -> Box<dyn Sort<usize>>,
}

pub struct App {
    pub current_screen: AppScreen,
    pub algorithms: Vec<Algorithm>,
    pub selected: usize,
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
            algorithms,
            current_screen: AppScreen::Menu,
            selected: 0,
        }
    }
}