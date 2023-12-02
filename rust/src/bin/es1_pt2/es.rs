/**
--- Part Two ---
Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?

Your puzzle answer was 53868.
**/

fn main(){
    let input = include_str!("in.txt");

    // regexps find first number in string
    let res:u32 = input
        .split('\n')
        .map(|line| {
            let mod_line = line
                .replace("one","one1one")
                .replace("three","three3three")
                .replace("five","five5five")
                .replace("six","six6six")
                .replace("seven","seven7seven")
                .replace("two","two2two")
                .replace("four","four4four")
                .replace("eight","eight8eight")
                .replace("nine","nine9nine");

            return es1(mod_line);
        })
        .sum();

    println!("{}", res);
}

pub fn es1(line: String) -> u32 {
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


