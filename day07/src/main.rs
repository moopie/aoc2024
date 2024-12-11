enum Op {
    Mul,
    Add
}

type Num = u32;

fn parse_eqs(input: String) -> u32 {
    let eqs: Vec<String> = input
        .lines()
        .filter(|x| !x.to_string().trim().is_empty())
        .map(|x| x.to_string())
        .collect();

    for eq in eqs {
        let f: Vec<String> = eq.split(":")
            .map(|x| x.trim().to_string())
            .collect();
        let result = f[0].parse::<Num>().expect("err");
        let arr = f[1]
            .split(" ")
            .map(|x| x.parse::<Num>().expect("err"))
            .collect();

        let solutions = get_solutions(arr);
    }
    0
}

fn get_solutions(equasion: Vec<Num>) -> Vec<Vec<Op>> {
    let ops = vec!['*', '+'];

    vec![]
}

fn main() {
    println!("AOC day 7!");
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
}
