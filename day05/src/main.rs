fn main() {
    println!("AOC 2024 day 05!");

    let file_contents = std::fs::read_to_string("input.txt")
        .expect("err");

    let res = check_rules(&file_contents);

    println!("part 1 result: {}", res);
}

#[derive(Clone,Copy)]
struct Rule {
    x: Page,
    y: Page,
}

type Page = u32;
type Book = Vec<Page>;

struct SafetyManual(Vec<Rule>, Vec<Book>);

trait CheckValidity {
    fn check_validity(self, book: Book) -> bool;
}

impl CheckValidity for Rule {
    fn check_validity(self, book: Book) -> bool {
        let page_x = book.iter().position(|p| *p == self.x);
        let page_y = book.iter().position(|p| *p == self.y);
        match (page_x, page_y) {
            (Some(x), Some(y)) => x < y,
            _ => true
        }
    }
}

fn check_rules(input: &str) -> u32 {
    let prog = get_update_record(input);
    let rules = prog.0;
    let mut total: Page = 0;

    for book in prog.1 {
        let valid = rules.iter()
            .all(|rule| rule.check_validity(book.to_vec()));

        if valid {
            let mid_index = book.len() / 2;
            let mid_value = book[mid_index];
            if book.len() % 2 != 0 {
                println!("err: book {:?} has uneven amount of elements", book);
            }
            //println!("valid book: {:?}, index: {}, value: {}", book.to_vec(), mid_index, mid_value);
            total = total + mid_value;
        }
    }

    return total.try_into().unwrap();
}

fn get_update_record(input: &str) -> SafetyManual {
    let lines = input.lines();
    let mut rules: Vec<Rule> = vec![];
    let mut books: Vec<Book> = vec![];

    for line in lines {
        let is_rule = line.contains("|");

        if is_rule {
            let nums: Vec<u32>  = line.split("|")
                .map(|x| {
                    match x.trim().parse::<Page>() {
                        Ok(value) => value,
                        Err(err) => {
                            println!("str: {}, err: {}", x, err);
                            0
                        }
                    }
                })
                .collect();

            if nums.len() == 2 {
                rules.push(Rule{
                    x: nums[0],
                    y: nums[1]
                });
            }
        }
        else {
            let nums: Vec<u32> = line.split(",")
                    .map(|x| {
                        match x.trim().parse::<Page>() {
                            Ok(value) => value,
                            Err(err) => {
                                println!("str: '{}', err: {}", x, err);
                                0
                            }
                        }
                    })
                    .collect();

            if nums.len() > 1 {
                books.push(nums);
            }
        }
    }

    SafetyManual(rules, books)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let input = r#"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
        "#;

        assert_eq!(check_rules(input), 143);
    }

    #[test]
    fn test_init1() {
        let input = r#"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
        "#;

        assert_eq!(check_rules(input), 61);
    }

    #[test]
    fn test_init2() {
        let input = r#"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
        "#;

        assert_eq!(check_rules(input), 143);
    }
}