use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Distance {
    distance: u64,
    direction: u64
}

fn get_node_count(str_map: String) -> u64 {
    let lines = str_map.lines();

    let freqs: HashSet<char> = str_map.chars()
        // remove empty space
        .filter(|x| *x != '.')
        .collect();

    let map: Vec<Vec<char>> = lines
        .map(|x| x.to_string().trim().chars().collect())
        .filter(|x: &Vec<char>| !x.is_empty())
        .collect();

    for freq in freqs {
        let mut freq_locs: Vec<(f64, f64, char)> = vec![];

        for (y, line) in map.iter().enumerate() {
            for (x, ch) in line.iter().enumerate() {
                if freq == *ch {
                    freq_locs.push((x as f64, y as f64, ch.clone()));
                }
            }
        }

        let mut set: HashSet<Distance> = HashSet::new();
        for (x1, y1, _) in freq_locs.iter() {
            for (x2, y2, ch) in freq_locs.iter() {
                if x1 == x2 && y1 == y2 {
                    continue;
                }

                let dist = ((x1 - y1).abs() + (y1 - y2).abs()) as f64;
                let dir = (y2 - y1).atan2(x2 - x1).to_degrees();

                set.insert(Distance {
                    distance: dist.abs() as u64,
                    direction: dir.abs() as u64
                });

                println!("'{}'\tdist: {}\t dir {}", ch, dist.abs() as u64, dir.abs() as u64);
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