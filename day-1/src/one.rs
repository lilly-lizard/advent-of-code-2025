fn decode_password_v1(input: &str) -> u32 {
    let instructions = input.split('\n');
    let mut dial: u32 = 50;
    let mut zero_count: u32 = 0;
    for i in instructions {
        dial = process_instruction_v1(dial, i);
        if dial == 0 {
            zero_count += 1;
        }
        println!("inst: {}, dial: {}, zero: {}", i, dial, zero_count);
    }
    return zero_count;
}
pub fn decode_password_v2(input: &str) -> u32 {
    let instructions = input.split('\n');
    let mut dial: u32 = 50;
    let mut zero_count: u32 = 0;
    for i in instructions {
        dial = process_instruction_v2(dial, i, &mut zero_count);
        println!("inst: {}, dial: {}, zero: {}", i, dial, zero_count);
    }
    return zero_count;
}

fn process_instruction_v1(dial: u32, instruction: &str) -> u32 {
    let incriment = decode_instruction(instruction);
    return rotate_dial_v1(dial, incriment);
}

fn process_instruction_v2(dial: u32, instruction: &str, zero_count: &mut u32) -> u32 {
    let incriment = decode_instruction(instruction);
    return rotate_dial_v2(dial, incriment, zero_count);
}

/// Returns how much to incriment the dial
fn decode_instruction(instruction: &str) -> i32 {
    let (dir, size) = instruction.split_at(1);
    let mut incriment: i32 = size.parse().unwrap();
    if dir == "L" {
        incriment *= -1;
    }
    return incriment;
}

/// Incriments the dial by `incriment` (positive or negative)
fn rotate_dial_v1(initial: u32, incriment: i32) -> u32 {
    let old: i32 = initial as i32;
    let mut new = old + incriment;
    while new < 0 {
        new += 100;
    }
    while 99 < new {
        new -= 100;
    }
    return new as u32;
}

/// Incriments the dial by `incriment` (positive or negative)
fn rotate_dial_v2(initial: u32, incriment: i32, zero_count: &mut u32) -> u32 {
    let mut prev: i32 = initial as i32;
    let mut new = prev + incriment;
    // incriment overflow
    while 99 < new {
        new -= 100;
        *zero_count += 1;
        println!("  +o");
    }
    // decrement overflow
    while new < 0 {
        new += 100;
        if prev != 0 {
            *zero_count += 1;
        }
        prev = new;
        println!("  -o");
    }
    // note that landing on 0 via positive incriment is caught by the incriment overflow code
    if incriment < 0 && new == 0 {
        *zero_count += 1;
    }
    return new as u32;
}

const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

#[cfg(test)]
mod tests {
    use super::{
        TEST_INPUT, decode_instruction, decode_password_v1, decode_password_v2,
        process_instruction_v1, process_instruction_v2,
    };

    #[test]
    fn test_decode_instruction() {
        let i: i32 = decode_instruction("L12");
        assert!(i == -12);
    }

    #[test]
    fn test_instruction_1() {
        let new = process_instruction_v1(11, "R8");
        assert!(new == 19)
    }

    #[test]
    fn test_instruction_2() {
        let new = process_instruction_v1(5, "L10");
        assert!(new == 95)
    }

    #[test]
    fn test_instruction_3() {
        let new = process_instruction_v1(95, "R6");
        assert!(new == 1)
    }

    #[test]
    fn test_zero_count_v1() {
        let zero_count = decode_password_v1(TEST_INPUT);
        assert!(zero_count == 3);
    }

    #[test]
    fn test_zero_count_v2() {
        let zero_count = decode_password_v2(TEST_INPUT);
        println!("bruh {}", zero_count);
        assert!(zero_count == 6);
    }
}
