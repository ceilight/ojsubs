use std::io::BufRead;

fn main() {
    let mut nums = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap());
    let mut prev = nums.next().unwrap();
    let mut count = 0;
    for num in nums {
        if prev < num {
            count += 1;
        }
        prev = num;
    }
    println!("{}", count);
}
