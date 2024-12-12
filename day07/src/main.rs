use std::fs::read_to_string;

fn parse_eqs(input: String) -> u32 {
    let mut ret = 0;

    let eqs: Vec<String> = input
        .lines()
        .filter(|x| !x.to_string().trim().is_empty())
        .map(|x| x.to_string())
        .collect();

    for eq in eqs {
        let f: Vec<String> = eq.split(":")
            .map(|x| x.trim().to_string())
            .collect();
        let target = f[0].parse::<u32>().expect("err");
        let arr = f[1]
            .split(" ")
            .map(|x| x.parse::<u32>().expect("err"))
            .collect();

        let solutions = get_solutions(arr, target);
        if solutions == 1 {
            ret += target;
        }
    }

    ret
}

fn get_solutions(equation: Vec<u32>, target: u32) -> u32 {
    let n = equation.len();
    let mut solutions = 0;

    let total_size = 1 << (n-1);

    for i in 0..total_size {
        let mut result = 0;

        for ii in 0..equation.len() {
            let num = equation[ii];
            if i & (1 << ii) != 0 {
                result += num;
            }
            else {
                result *= num;
            }
        }
        println!("{} target: {} | iter {}", result, target, i);
        if result == target {
            solutions += 1;
        }
    }
    println!("{:?} solutions: {}", equation, solutions);

    solutions
}

fn main() {
    println!("AOC day 7!");

    let content = read_to_string("input.txt").expect("err");
    let result_part_1 = parse_eqs(content);

    println!("result for part 1: {}", result_part_1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let input = r#"
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
        "#;

        assert_eq!(parse_eqs(input.to_string()), 3749);
    }

    #[test]
    fn init_v2() {
        let input = r#"
            190: 10 19
        "#;

        assert_eq!(parse_eqs(input.to_string()), 190);
    }

    #[test]
    fn init_v3() {
        let input = r#"
            3267: 81 40 27
        "#;

        assert_eq!(parse_eqs(input.to_string()), 0);
    }

    #[test]
    fn init_v4() {
        let input = r#"
            292: 11 6 16 20
        "#;

        assert_eq!(parse_eqs(input.to_string()), 292);
    }
}
