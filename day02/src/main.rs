struct Intcode {
    pointer: usize,
    memory: [i32; 121],
}

fn make_intcode() -> Intcode {
    let intcode: Intcode = Intcode {
        pointer: 0,
        memory: [
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 2, 9, 19, 23, 2, 13, 23,
            27, 1, 6, 27, 31, 2, 6, 31, 35, 2, 13, 35, 39, 1, 39, 10, 43, 2, 43, 13, 47, 1, 9, 47,
            51, 1, 51, 13, 55, 1, 55, 13, 59, 2, 59, 13, 63, 1, 63, 6, 67, 2, 6, 67, 71, 1, 5, 71,
            75, 2, 6, 75, 79, 1, 5, 79, 83, 2, 83, 6, 87, 1, 5, 87, 91, 1, 6, 91, 95, 2, 95, 6, 99,
            1, 5, 99, 103, 1, 6, 103, 107, 1, 107, 2, 111, 1, 111, 5, 0, 99, 2, 14, 0, 0,
        ],
    };
    intcode
}

fn opcode(intcode: &mut Intcode) -> u8 {
    let action: i32 = intcode.memory[intcode.pointer];
    let address1: usize = intcode.memory[intcode.pointer + 1] as usize;
    let address2: usize = intcode.memory[intcode.pointer + 2] as usize;
    let address3: usize = intcode.memory[intcode.pointer + 3] as usize;

    match action {
        1 => {
            intcode.memory[address3] = intcode.memory[address1] + intcode.memory[address2];
            intcode.pointer += 4;
            1
        }
        2 => {
            intcode.memory[address3] = intcode.memory[address1] * intcode.memory[address2];
            intcode.pointer += 4;
            1
        }
        _ => 0,
    }
}

fn updated_memory(intcode: &mut Intcode, noun: i32, verb: i32) -> () {
    intcode.memory[1] = noun;
    intcode.memory[2] = verb;
}

fn noun_verb() -> i32 {
    for noun in 0..101 {
        for verb in 0..101 {
            let mut intcode = make_intcode();
            updated_memory(&mut intcode, noun, verb);

            let mut ic_return: u8 = 1;
            while ic_return == 1 {
                ic_return = opcode(&mut intcode);
            }

            let candidate = intcode.memory[0];
            if candidate == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    -1
}

fn main() {
    let mut intcode = make_intcode();
    updated_memory(&mut intcode, 12, 2);

    let mut ic_return: u8 = 1;
    while ic_return == 1 {
        ic_return = opcode(&mut intcode);
    }

    println!("\nPart A answer = {}. Correct = 2890696", intcode.memory[0]);
    println!("Part B answer = {}. Correct = 8226", noun_verb());
}
