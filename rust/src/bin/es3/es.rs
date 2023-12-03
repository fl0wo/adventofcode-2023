/**
--- Day 3: Gear Ratios ---
You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

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
In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
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

    let mut summing_num = Vec::new();

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
                            res += num.value;
                            summing_num.push(num.value);

                            num.taken = true;
                            // if you want to connect the edges to the graph
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

    println!("res: {}", res);
}

