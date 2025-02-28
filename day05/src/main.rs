// fn change_value(arr: &mut [i32; 5]) -> &mut [i32; 5] {
//     arr[1] = 10;
//     arr[4] = 50;
//     arr
// }
//
// fn main() {
//     let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
//     println!("{:p}", &arr);
//     println!("this is {}", arr[4]);
//     arr = *change_value(&mut arr);
//     println!("{:p}", &arr);
//     println!("this is now {}", arr[4]);
// }

// format!("{:04}", 42);             // => "0042" with leading zeros

// int *pad5(const int op, Instruction instruction) {
// char buffer[6];
// snprintf(buffer, 6, "%05d", op);
// for (int i = 0; i < 5; i++) {
// instruction[i] = buffer[i] - '0';
// }
// return instruction;
// }

fn pad5(op: i32, instruction: &mut [u8; 5]) {
    let padded_string = format!("{:05}", op);
    let buffer = padded_string.as_bytes();
    for i in 0..5 {
        instruction[i] = buffer[i] - 48;
    }
}

// fn change_value(arr: &mut [i32; 5], zero: i32, four: i32) {
//     arr[0] = zero;
//     arr[4] = four;
// }

fn main() {
    // let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("{:p}", &arr);
    // println!("this is {}", arr[0]);
    // change_value(&mut arr, 10, 50);
    // println!("{:p}", &arr);
    // println!("this is now {}", arr[0]);
    let mut instruction: [u8; 5] = [1, 2, 3, 4, 5];
    println!("{:p}", &instruction);
    pad5(12345, &mut instruction);
    for i in 0..5 {
        println!("{}", instruction[i]);
    }
    println!("{:p}", &instruction);
}

// struct Intcode {
//     pointer: usize,
//     memory: [i32; 121],
// }
//
// type Instruction = [i32; 5];
//
// fn main() {
//     let mut intcode: Intcode = make_intcode();
//     updated_memory(&mut intcode, 12, 2);
//
//     let mut ic_return: u8 = 1;
//     while ic_return == 1 {
//         ic_return = opcode(&mut intcode);
//     }
//
//     println!("\nPart A answer = {}. Correct = 2890696", intcode.memory[0]);
//     println!("Part B answer = {}. Correct = 8226\n", noun_verb());
// }
//
// fn make_intcode() -> Intcode {
//     let intcode: Intcode = Intcode {
//         pointer: 0,
//         memory: [
//             1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 2, 9, 19, 23, 2, 13, 23,
//             27, 1, 6, 27, 31, 2, 6, 31, 35, 2, 13, 35, 39, 1, 39, 10, 43, 2, 43, 13, 47, 1, 9, 47,
//             51, 1, 51, 13, 55, 1, 55, 13, 59, 2, 59, 13, 63, 1, 63, 6, 67, 2, 6, 67, 71, 1, 5, 71,
//             75, 2, 6, 75, 79, 1, 5, 79, 83, 2, 83, 6, 87, 1, 5, 87, 91, 1, 6, 91, 95, 2, 95, 6, 99,
//             1, 5, 99, 103, 1, 6, 103, 107, 1, 107, 2, 111, 1, 111, 5, 0, 99, 2, 14, 0, 0,
//         ],
//     };
//     intcode
// }
//
// // fn pad5(op: i32, instruction: &mut [i32; 5]) -> [i32; 5] {
// //
// // }
//
// fn opcode(intcode: &mut Intcode) -> u8 {
//     let action: i32 = intcode.memory[intcode.pointer];
//     let address1: usize = intcode.memory[intcode.pointer + 1] as usize;
//     let address2: usize = intcode.memory[intcode.pointer + 2] as usize;
//     let address3: usize = intcode.memory[intcode.pointer + 3] as usize;
//
//     match action {
//         1 => {
//             intcode.memory[address3] = intcode.memory[address1] + intcode.memory[address2];
//             intcode.pointer += 4;
//             1
//         }
//         2 => {
//             intcode.memory[address3] = intcode.memory[address1] * intcode.memory[address2];
//             intcode.pointer += 4;
//             1
//         }
//         _ => 0,
//     }
// }
//
// fn updated_memory(intcode: &mut Intcode, noun: i32, verb: i32) -> () {
//     intcode.memory[1] = noun;
//     intcode.memory[2] = verb;
// }
//
// fn noun_verb() -> i32 {
//     for noun in 0..101 {
//         for verb in 0..101 {
//             let mut intcode: Intcode = make_intcode();
//             updated_memory(&mut intcode, noun, verb);
//
//             let mut ic_return: u8 = 1;
//             while ic_return == 1 {
//                 ic_return = opcode(&mut intcode);
//             }
//
//             let candidate: i32 = intcode.memory[0];
//             if candidate == 19690720 {
//                 return 100 * noun + verb;
//             }
//         }
//     }
//     -1
// }
