/**
--- Part Two ---
Everyone will starve if you only plant such a small number of seeds. Re-reading the almanac, it looks like the seeds: line actually describes ranges of seed numbers.

The values on the initial seeds: line come in pairs. Within each pair, the first value is the start of the range and the second value is the length of the range. So, in the first line of the example above:

seeds: 79 14 55 13
This line describes two ranges of seed numbers to be planted in the garden. The first range starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92. The second range starts with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.

Now, rather than considering four seed numbers, you need to consider a total of 27 seed numbers.

In the above example, the lowest location number can be obtained from seed number 82, which corresponds to soil 84, fertilizer 84, water 84, light 77, temperature 45, humidity 46, and location 46. So, the lowest location number is 46.

Consider all of the initial seed numbers listed in the ranges on the first line of the almanac. What is the lowest location number that corresponds to any of the initial seed numbers?
 **/

// 2 elements DP array

use memoize::memoize;

static mut MAPS: Vec<Vec<Vec<u64>>> = Vec::new();

fn get_map_at_index(index: usize, pos: usize) -> &'static Vec<u64> {
    unsafe {
        return &MAPS[index][pos];
    }
}

fn get_map_at_index_1(index: usize) -> &'static Vec<Vec<u64>> {
    unsafe {
        return &MAPS[index];
    }
}

fn get_maps_len() -> usize {
    unsafe {
        return MAPS.len();
    }
}

fn main() {
    let lines = include_str!("in");

    let blocks = lines
        .split("\n\n")
        .map(|line| {
            return line.split_at(line.find(":").unwrap())
                .1
                .split("\n");
        })
        .map(|lines| {
            return lines
                .map(|line| {
                    return line
                        .replace(":", "")
                        .split(" ")
                        .filter(|n| !n.is_empty() && !n.contains(":") && !n.contains("\n"))
                        .map(|n| {
                            // fix `Err` value: ParseIntError { kind: PosOverflow }
                            return n.trim().parse::<u64>().unwrap();
                        })
                        .collect::<Vec<u64>>();
                });
        });

    for block in blocks {
        let mut map: Vec<Vec<u64>> = Vec::new();
        for line in block {
            if line.len() == 0 {
                continue;
            }
            let mut v: Vec<u64> = Vec::new();
            for n in line {
                v.push(n);
            }
            map.push(v);
        }
        unsafe {
            MAPS.push(map);
        }
    }

    // println!("{:?}", maps);

    let seeds = get_map_at_index(0, 0).clone();
    // lets do it recursively

    let res = seeds
        .chunks(2)
        .map(|chunk| {
            let start = chunk[0];
            let len = chunk[1];
            return (start..start+len).collect::<Vec<u64>>();
        })
        .flat_map(|seed| {
            return seed.iter().map(|s| {
                return *s;
            }).collect::<Vec<u64>>();
        })
        .map(|seed| {
            return solve_recursive(seed, 1);
        })
        .min()
        .unwrap();

    println!("{:?}", res);
}

#[memoize]
fn solve_recursive(seed: u64, pos: usize) -> u64 {
    if pos >= get_maps_len() {
        return seed;
    }

    let mut res: Vec<u64> = Vec::new();

    let maps = get_map_at_index_1(pos);

    for map in maps {
        let dest = map[0];
        let src = map[1];
        let len = map[2];
        if seed >= src && seed < src + len {
            res.push(dest + (seed - src));
        }
    }

    // If no rules apply, the number is unchanged.
    if res.len() == 0 {
        res.push(seed);
    }

    let mut res2: Vec<u64> = Vec::new();
    for r in res {
        res2.push(solve_recursive(r, pos+1));
    }

    return *res2.iter().min().unwrap();
}