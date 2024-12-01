use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("{file_path}\n");

    let contents = fs::read_to_string(file_path)
        .expect("something went wrong");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut arr: (Vec<i32>, Vec<i32>) = (vec![], vec![]);

    for (index, line) in lines.iter().enumerate() {
        let nums: Vec<&str> = line.split_whitespace().collect();

        if nums.len() != 2 {
            eprintln!("Cant parse file @ line {index}\n");
        } else {
            let a = nums[0].parse::<i32>().expect("err");
            let b = nums[1].parse::<i32>().expect("err");
            arr.0.push(a);
            arr.1.push(b);
        }

        println!("{line}\n");
    }

    let sx = arr.0.len();
    let sy = arr.1.len();
    println!("{} and {}", arr.0.len(), arr.1.len());
    arr.0.sort();
    arr.1.sort();

    let zip: Vec<(i32, i32)> = arr.0.iter().zip(arr.1.iter())
        .map(|(&a, &b)| (a, b)).collect();

    let mut res: i32 = 0;

    for item in zip.iter() {
        let n = (item.1 - item.0).abs();
        res += n;
    }

    println!("final result: {res}");
}
