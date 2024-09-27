use itertools::join;
use std::io;

fn insertion_sort(numbers: &mut Vec<i32>) {
    if numbers.len() < 2 {
        return;
    }

    let mut i = 1;
    loop {
        let mut j = i - 1;
        while numbers[j] > numbers[j + 1] {
            numbers.swap(j, j + 1);

            if j == 0 {
                break;
            }
            j = j - 1;
        }
        i += 1;
        if i >= numbers.len() {
            break;
        }
    }
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let mut numbers: Vec<i32> = input
        .trim()
        .split(' ')
        .map(|x| x.trim().parse().expect("Not an integer"))
        .collect();

    insertion_sort(&mut numbers);
    print!("{}", join(&numbers, " "));
}
