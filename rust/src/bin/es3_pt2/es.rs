/**
--- Part Two ---
The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?

Your puzzle answer was 80703636.
 **/

struct Component {
    l: usize,
    r: usize,
    row: usize,
    value: u32,
    taken: bool,
}

struct Symbol{
    l: usize,
    r: usize,
    row: usize,
    value: char,
    components: Vec<Component>,
}
const MAX_LINES : usize = 1050;

fn main() {
    let input = include_str!("input.txt");
    let lines: Vec<_> = input.split('\n').collect();
    let rows: usize = lines.len();
    let mut num_stack: [Vec<Component>;MAX_LINES] = array_init::array_init(|_| Vec::new());
    let mut symbol_stack = Vec::new();

    let mut res: u32 = 0;

    // 1. Build the stacks
    lines
        .iter()
        .enumerate()
        .for_each(|(row, line)| {
            let mut num = String::new();
            let mut start_index:usize = 0;

            // initialize stacks
            num_stack[row] = Vec::new();

            line
                .chars()
                .enumerate()
                .for_each(|(index, c)| {

                    // num parsing
                    if c.is_numeric() {
                        if start_index == 0 {
                            start_index = index;
                        }
                        num.push(c);
                    }

                    // end of num parsing
                    if !c.is_numeric() || index == line.len() - 1 {
                        // end of num parsing
                        if num.len() > 0 {

                            num_stack[row].push(Component {
                                l:if start_index > 0 { start_index - 1 } else { 0 }, // also diagonals
                                r: index + 1, // also diagonals
                                row,
                                value: num.parse::<u32>().unwrap(),
                                taken: false,
                            });
                            start_index = 0;
                            num.clear();
                        }

                        // track symbol position
                        if c!='.' &&  !c.is_numeric() {

                            symbol_stack.push(Symbol {
                                l: index,
                                r: index + 1,
                                row,
                                value: c,
                                components: Vec::new(),
                            });
                        }
                    }
                });
        });


    // 2. Find the strongly-connected components
    symbol_stack
        .iter_mut()
        .for_each(|symbol|{

            // only 3 layers
            let three_layers = vec![
                if symbol.row > 0 { symbol.row - 1 } else { 0 }, // before
                symbol.row, // current
                symbol.row + 1, // after
            ];

            three_layers
                .iter()
                .for_each(|row| {
                num_stack[*row]
                    .iter_mut()
                    .filter(|num| !num.taken) // no duplicates
                    .for_each(|num| {
                        if num.l <= symbol.l && num.r >= symbol.r {
                            num.taken = true;
                            symbol.components.push(Component {
                                l: num.l,
                                r: num.r,
                                row: num.row,
                                value: num.value,
                                taken: true,
                            });
                        }
                    });
            });
        });

    // 3. Calculate the gear ratios
    symbol_stack
        .iter_mut()
        .filter(|symbol| symbol.value == '*')
        .filter(|symbol| symbol.components.len() >= 2)
        .for_each(|symbol|{
            let mut gear_ratio: u32 = 1;
            symbol
                .components
                .iter()
                .for_each(|component|{
                    println!("{} {} {}", component.l, component.r, component.value);
                    gear_ratio *= component.value;
                });
            res += gear_ratio;
        });


    println!("res: {}", res);
}

