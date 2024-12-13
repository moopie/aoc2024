use std::{borrow::Borrow, collections::HashSet};

fn get_node_count(str_map: String) -> u64 {
    let lines = str_map.lines();

    let freqs: HashSet<char> = str_map.chars()
        // remove empty space
        .filter(|x| *x != ',')
        .collect();

    let map: Vec<Vec<char>> = lines
        .map(|x| x.to_string().trim().chars().collect())
        .filter(|x: &Vec<char>| !x.is_empty())
        .collect();

    for freq in freqs {
        let mut freq_locs: Vec<(u32, u32)> = vec![];

        for (y, line) in map.iter().enumerate() {
            for (x, ch) in line.iter().enumerate() {
                if freq == *ch {
                    freq_locs.push((x as u32, y as u32));
                }
            }
        }
    }
    0
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let input = r#"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
        "#;

        assert_eq!(get_node_count(input.to_string()), 14);
    }
}