use std::collections::HashMap;

use crate::sort::*;

pub enum AppScreen {
    Menu,
    Sort,
}

pub struct App {
    pub current_screen: AppScreen,
    pub algorithms: Vec<(&'static str, fn() -> Box<dyn Sort<usize>>)>,
}

impl App {
    pub fn new() -> App {
        let mut algorithms: Vec<(&str, fn() -> Box<dyn Sort<usize>>)> = vec![
                ("Bogosort", || Box::new(BogoSort::new())),
                ("Bubble sort", || Box::new(BubbleSort::new())),
                ("Cocktail shaker sort", || Box::new(CocktailShakerSort::new())),
                ("Insertion sort", || Box::new(InsertionSort::new())),
                ("Quicksort", || Box::new(QuickSort::new())),
                ("Selection sort", || Box::new(SelectionSort::new())),   
        ];

        App {
            algorithms,
            current_screen: AppScreen::Menu,
        }
    }
}