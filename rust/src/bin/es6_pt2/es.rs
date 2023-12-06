/**
--- Part Two ---
As the race is about to start, you realize the piece of paper with race times and record distances you got earlier actually just has very bad kerning. There's really only one race - ignore the spaces between the numbers on each line.

So, the example from before:

Time:      7  15   30
Distance:  9  40  200
...now instead means this:

Time:      71530
Distance:  940200
Now, you have to figure out how many ways there are to win this single race. In this example, the race lasts for 71530 milliseconds and the record distance you need to beat is 940200 millimeters. You could hold the button anywhere from 14 to 71516 milliseconds and beat the record, a total of 71503 ways!

How many ways can you beat the record in this one much longer race?


 **/

use std::ops::Index;
use ::phf::{phf_map, Map};

fn main() {
    let input = include_str!("in").split("\n").collect::<Vec<&str>>();

    let time = input[0].split_once(":").unwrap().1
        .replace(" ","")
        .parse::<u64>()
        .unwrap();

    let distance = input[1].split_once(":").unwrap().1
        .replace(" ","")
        .parse::<u64>()
        .unwrap();

    println!("times: {:?}", time);
    println!("distances: {:?}", distance);

    let mut res: u64 = calc_ways_to_win(time, distance);
    println!("{:?}", res);
}

fn calc_ways_to_win(time: u64, distance: u64) -> u64 {

    // time: in ms
    // distance: in mm

    // I can hold the btn for 0...time ms
    // How many ways I can win?
    // it will be a left bound and a right bound

    let mut ways = 0;

    // do a binary search to find the left bound
    let mut left = 0;
    let mut right = time;
    let mut mid = (left + right) / 2; // how many ms I hold the btn

    // binary search each to find the left most
    while left <= right {
        mid = (left + right) / 2;
        let distance_traveled = calc_distance_traveled(mid, time, distance);
         if distance_traveled > distance {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    let mut left2 = left;
    let mut right2 = time;
    let mut mid2 = (left2 + right2) / 2; // how many ms I hold the btn

    // now find the right-most border of the range
    while left2 <= right2 {
        mid2 = (left2 + right2) / 2;
        let distance_traveled = calc_distance_traveled(mid2, time, distance);
         if distance_traveled > distance {
            left2 = mid2 + 1;
        } else {
            right2 = mid2 - 1;
        }
    }

    return (right2 - left) + 1;
}

fn calc_distance_traveled(ms: u64, time: u64, distance: u64) -> u64 {
    return ms * (time - ms);
}