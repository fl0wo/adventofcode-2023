use itertools::Itertools;

struct Rule {
    destination: i64,
    source: i64,
    range: i64,
}

#[derive(Debug, Clone)]
struct Range {
    from: i64,
    to: i64,
}

pub fn part_1(input: &str) -> i64 {
    let (seeds_str, maps_str) = input.split_once("\n\n").unwrap();
    let seeds = seeds_str.strip_prefix("seeds: ").unwrap();
    let seeds = seeds.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    let maps: Vec<Vec<Rule>> = maps_str
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| {
                    let mut nums = line.splitn(3, " ");
                    Rule {
                        destination: nums.next().unwrap().parse().unwrap(),
                        source: nums.next().unwrap().parse().unwrap(),
                        range: nums.next().unwrap().parse().unwrap(),
                    }
                })
                .collect()
        })
        .collect();

    seeds
        .map(|seed| {
            maps.iter().fold(seed, |curr, rules| {
                if let Some(rule) = rules
                    .iter()
                    .find(|rule| curr >= rule.source && curr <= rule.source + rule.range)
                {
                    let offset = curr - rule.source;
                    rule.destination + offset
                } else {
                    curr
                }
            })
        })
        .min()
        .unwrap()
}

pub fn part_2(input: &str) -> i64 {
    let (seeds_str, maps_str) = input.split_once("\n\n").unwrap();
    let seeds = seeds_str.strip_prefix("seeds: ").unwrap();
    let seeds = seeds
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .chunks(2);
    let seeds = seeds.into_iter().map(|mut chunk| {
        let from = chunk.next().unwrap();
        let range = chunk.next().unwrap();
        Range {
            from,
            to: from + range,
        }
    });

    let maps: Vec<Vec<Rule>> = maps_str
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| {
                    let mut nums = line.splitn(3, " ");
                    Rule {
                        destination: nums.next().unwrap().parse().unwrap(),
                        source: nums.next().unwrap().parse().unwrap(),
                        range: nums.next().unwrap().parse().unwrap(),
                    }
                })
                .sorted_by(|a, b| a.source.cmp(&b.source))
                .collect()
        })
        .collect();

    // for every range in the seed ranges, transform it to a range of the next kind, repeat until all all maps are applied, at that point all ranges are location ranges
    // loop1 transforms those ranges of seed to ranges of soils
    // loop2 transforms those ranges of soil to ranges of fertilizer
    // loop3 transforms those ranges of fertilizer to ranges of water
    // loop4 transforms those ranges of water to ranges of light
    // loop5 transforms those ranges of light to ranges of temperature
    // loop6 transforms those ranges of temperature to ranges of humidity
    // loop7 transforms those ranges of humidity to ranges of location
    let mut curr_ranges: Vec<Range> = seeds.collect();

    for map in &maps {
        let mut new_ranges: Vec<Range> = Vec::new();

        for range in &curr_ranges {
            let mut curr = range.clone();

            for rule in map {
                let offset = rule.destination - rule.source;
                let rule_applies = curr.from <= curr.to
                    && curr.from <= rule.source + rule.range
                    && curr.to >= rule.source;

                if rule_applies {
                    if curr.from < rule.source {
                        new_ranges.push(Range {
                            from: curr.from,
                            to: rule.source - 1,
                        });
                        curr.from = rule.source;
                        if curr.to < rule.source + rule.range {
                            new_ranges.push(Range {
                                from: curr.from + offset,
                                to: curr.to + offset,
                            });
                            curr.from = curr.to + 1;
                        } else {
                            new_ranges.push(Range {
                                from: curr.from + offset,
                                to: rule.source + rule.range - 1 + offset,
                            });
                            curr.from = rule.source + rule.range;
                        }
                    } else if curr.to < rule.source + rule.range {
                        new_ranges.push(Range {
                            from: curr.from + offset,
                            to: curr.to + offset,
                        });
                        curr.from = curr.to + 1;
                    } else {
                        new_ranges.push(Range {
                            from: curr.from + offset,
                            to: rule.source + rule.range - 1 + offset,
                        });
                        curr.from = rule.source + rule.range;
                    }
                }
            }
            if curr.from <= curr.to {
                new_ranges.push(curr);
            }
        }
        curr_ranges = new_ranges;
    }

    curr_ranges.iter().map(|range| range.from).min().unwrap()
}

fn main() {
    let input = include_str!("in");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}