use std::{cmp::Ordering, fs};

#[derive(Clone, Debug)]
struct Page {
    before: Vec<u8>,
    visited: bool,
}

impl Page {
    fn new(visited: bool, before: Vec<u8>) -> Self {
        Self { before, visited }
    }

    fn add_before(&mut self, page: u8) {
        self.before.push(page);
    }

    fn visit(&mut self) {
        self.visited = true;
    }
}

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");

    let order_pairs: Vec<(u8, u8)> = data
        .lines()
        .into_iter()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let pages: Vec<&str> = line.split('|').collect();

            let first = pages[0].parse::<u8>().unwrap();
            let second = pages[1].parse::<u8>().unwrap();
            (first, second)
        })
        .collect();

    let page_list: Vec<Vec<u8>> = data
        .lines()
        .into_iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let list = line
                .split(',')
                .map(|no| no.parse::<u8>().unwrap())
                .collect();
            list
        })
        .collect();

    // Push the page orders - index by what needs to be after
    let mut orders = vec![Page::new(false, Vec::new()); 100]; // every page is 2 digits
    for (first, second) in order_pairs {
        orders[second as usize].add_before(first);
    }

    // Check the page lists
    let valid_pages: Vec<Vec<u8>> = page_list
        .into_iter()
        .filter(|pages| {
            // Discard all pages from the list that are invalid
            let mut curr_orders = orders.clone();
            pages.iter().all(|&page| {
                curr_orders[page as usize].visit();

                // Pages that are not present in the current list at all should be ignored
                // Create a filtered befores list
                let filtered_befores: Vec<u8> = curr_orders[page as usize]
                    .before
                    .clone()
                    .into_iter()
                    .filter(|page| pages.contains(page))
                    .collect();

                // any() returns TRUE if any predicates are true (duh)
                // we need to check if a page order is not visited while it's present in the filtered list
                // this way the result is FALSE if such a page number is found
                // and 'false' will cause the outside filter() to drop this list as invalid
                // note: can't use all() because of empty before lists
                !curr_orders[page as usize].before.iter().any(|bef| {
                    !curr_orders[*bef as usize].visited && filtered_befores.contains(bef)
                })
            })
        })
        .collect();

    // Calculate the sum of the middle elements
    valid_pages
        .iter()
        .map(|page_list| page_list[page_list.len() / 2] as u32)
        .sum()
}

pub fn solve_2(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");

    let order_pairs: Vec<(u8, u8)> = data
        .lines()
        .into_iter()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let pages: Vec<&str> = line.split('|').collect();

            let first = pages[0].parse::<u8>().unwrap();
            let second = pages[1].parse::<u8>().unwrap();
            (first, second)
        })
        .collect();

    let page_list: Vec<Vec<u8>> = data
        .lines()
        .into_iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let list = line
                .split(',')
                .map(|no| no.parse::<u8>().unwrap())
                .collect();
            list
        })
        .collect();

    // Push the page orders - index by what needs to be after
    let mut orders = vec![Page::new(false, Vec::new()); 100]; // every page is 2 digits
    for (first, second) in &order_pairs {
        orders[*second as usize].add_before(*first);
    }

    // Check the page lists
    let mut invalid_pages: Vec<Vec<u8>> = page_list
        .into_iter()
        .filter(|pages| {
            // Discard all pages from the list that are invalid
            let mut curr_orders = orders.clone();
            // Change to any() from pt1
            pages.iter().any(|&page| {
                curr_orders[page as usize].visit();

                // Pages that are not present in the current list at all should be ignored
                // Create a filtered befores list
                let filtered_befores: Vec<u8> = curr_orders[page as usize]
                    .before
                    .clone()
                    .into_iter()
                    .filter(|page| pages.contains(page))
                    .collect();

                // change from pt1
                curr_orders[page as usize].before.iter().any(|bef| {
                    !curr_orders[*bef as usize].visited && filtered_befores.contains(bef)
                })
            })
        })
        .collect();

    let valid_pages: Vec<Vec<u8>> = invalid_pages
        .iter_mut()
        .map(|list| {
            // Make it valid by sorting by the presence of the page pair order
            list.sort_by(|a, b| {
                if order_pairs.contains(&(*a, *b)) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            list.clone()
        })
        .collect();

    // Calculate the sum of the middle elements
    valid_pages
        .iter()
        .map(|page_list| page_list[page_list.len() / 2] as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day5/test1.txt");
        assert_eq!(result, 143);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day5/test2.txt");
        assert_eq!(result, 123);
    }
}
