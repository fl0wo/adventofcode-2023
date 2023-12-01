fn main() {
    let input = include_str!("in.txt");

    let res:u32 = input
        .split("\n")
        .map(|line| {
            return es1(line);
        })
        .sum();

    println!("{}", res);
}
fn es1(line: &str) -> u32 {
    let first_num = line.chars()
        .find(|c| c.is_numeric())
        .unwrap_or('0');

    let last_num = line.chars()
        .rfind(|c| c.is_numeric())
        .unwrap_or('0');

    let first_num = first_num
        .to_digit(10)
        .unwrap_or(0);

    let last_num = last_num
        .to_digit(10)
        .unwrap_or(0);

    return (10 * first_num) + last_num;
}



