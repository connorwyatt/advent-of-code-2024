use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::{many_m_n, separated_list1},
    IResult,
};

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", calculate_program_output(INPUT));
}

fn calculate_program_output(input: &str) -> String {
    let (_, (mut computer, instructions)) = parse_input(input).unwrap();

    let mut instruction_pointer = 0;

    while let Some([opcode, operand]) = instructions.windows(2).nth(instruction_pointer) {
        let initial_instruction_pointer = instruction_pointer;

        match opcode {
            0 => {
                adv(&mut computer, operand);
            }
            1 => {
                bxl(&mut computer, operand);
            }
            2 => {
                bst(&mut computer, operand);
            }
            3 => {
                jnz(&computer, &mut instruction_pointer, operand);
            }
            4 => {
                bxc(&mut computer);
            }
            5 => {
                out(&mut computer, operand);
            }
            6 => {
                bdv(&mut computer, operand);
            }
            7 => {
                cdv(&mut computer, operand);
            }
            _ => unreachable!("values are 3-bit unsigned integers"),
        }

        if instruction_pointer == initial_instruction_pointer {
            instruction_pointer += 2;
        }
    }

    computer
        .output
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn adv(computer: &mut Computer, operand: &u8) {
    computer.register_a /= 2i32.pow(get_combo_operand(computer, operand) as u32);
}

fn bxl(computer: &mut Computer, operand: &u8) {
    computer.register_b ^= *operand as i32;
}

fn bst(computer: &mut Computer, operand: &u8) {
    computer.register_b = get_combo_operand(computer, operand) % 8
}

fn jnz(computer: &Computer, instruction_pointer: &mut usize, operand: &u8) {
    if computer.register_a == 0 {
        return;
    }

    *instruction_pointer = *operand as usize;
}

fn bxc(computer: &mut Computer) {
    computer.register_b ^= computer.register_c;
}

fn out(computer: &mut Computer, operand: &u8) {
    computer
        .output
        .push(get_combo_operand(computer, operand) % 8);
}

fn bdv(computer: &mut Computer, operand: &u8) {
    computer.register_b =
        computer.register_a / 2i32.pow(get_combo_operand(computer, operand) as u32);
}

fn cdv(computer: &mut Computer, operand: &u8) {
    computer.register_c =
        computer.register_a / 2i32.pow(get_combo_operand(computer, operand) as u32);
}

fn get_combo_operand(computer: &Computer, operand: &u8) -> i32 {
    match operand {
        0..=3 => *operand as i32,
        4 => computer.register_a,
        5 => computer.register_b,
        6 => computer.register_c,
        7 => unreachable!("7 is reserved and will not appear in valid programs"),
        _ => unreachable!("values are 3-bit unsigned integers"),
    }
}

#[derive(Debug)]
struct Computer {
    register_a: i32,
    register_b: i32,
    register_c: i32,
    output: Vec<i32>,
}

fn parse_register<'a>(input: &'a str, register: &char) -> IResult<&'a str, i32> {
    let (input, _) = tag(format!("Register {}: ", register).as_str())(input)?;
    let (input, value) = map_res(digit1, |digits: &str| digits.parse::<i32>())(input)?;
    Ok((input, value))
}

fn parse_program(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, _) = tag("Program: ")(input)?;
    let (input, value) = separated_list1(
        tag(","),
        map_res(digit1, |digits: &str| digits.parse::<u8>()),
    )(input)?;
    Ok((input, value))
}

fn parse_input(input: &str) -> IResult<&str, (Computer, Vec<u8>)> {
    let (input, register_a) = parse_register(input, &'A')?;
    let (input, _) = line_ending(input)?;
    let (input, register_b) = parse_register(input, &'B')?;
    let (input, _) = line_ending(input)?;
    let (input, register_c) = parse_register(input, &'C')?;

    let (input, _) = many_m_n(2, 2, line_ending)(input)?;
    let (input, program) = parse_program(input)?;

    Ok((
        input,
        (
            Computer {
                register_a,
                register_b,
                register_c,
                output: Vec::new(),
            },
            program,
        ),
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("aoc-input/example-input.txt");

    #[test]
    fn calculate_program_output_works() {
        assert_eq!(
            calculate_program_output(EXAMPLE_INPUT),
            String::from("4,6,3,5,6,3,5,2,1,0")
        )
    }
}
