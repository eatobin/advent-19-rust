const INPUT: [i32; 100] = [
    68958, 82218, 54333, 59177, 51874, 100259, 95468, 124006, 75078, 113631, 90315, 147580, 68233,
    81025, 133084, 90959, 81196, 92443, 124832, 65871, 57704, 140203, 113053, 76337, 72195, 115917,
    87843, 131768, 105816, 131153, 59714, 94107, 50933, 139545, 94969, 149463, 60042, 66028,
    111190, 63257, 50020, 88783, 81428, 73977, 149240, 137152, 74738, 55067, 128829, 56465, 81962,
    67242, 94121, 92303, 68477, 88595, 64324, 82527, 134717, 140344, 132026, 137558, 95643, 79010,
    146346, 86246, 52341, 147564, 89159, 66456, 83190, 128675, 130658, 122857, 134538, 122151,
    133900, 117462, 117791, 139254, 86366, 66165, 92897, 121218, 135962, 143061, 129220, 114623,
    98257, 76722, 121014, 77713, 137858, 133282, 103595, 118981, 149137, 101141, 70765, 141113,
];

fn fuel_value(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn rec_fuel_value(mass: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut fuel: i32 = fuel_value(mass);

    while fuel > 0 {
        sum += fuel;
        fuel = fuel_value(fuel);
    }
    sum
}

fn main() {
    let mut tot_fuel: i32 = 0;
    let mut tot_fuel_rec: i32 = 0;

    for mass in INPUT {
        tot_fuel += fuel_value(mass);
    }
    println!("Part a: {}", tot_fuel); // 3337766

    for mass in INPUT {
        tot_fuel_rec += rec_fuel_value(mass);
    }
    println!("Part b: {}", tot_fuel_rec); // 5003788
}

// fn part_one() -> i32 {
//     INPUT.iter().map(|i| i / 3 - 2).sum::<i32>()
// }
//
// fn part_two() -> i32 {
//     INPUT
//         .iter()
//         .map(|&i| {
//             let (mut ans, mut mass) = (0, i);
//             loop {
//                 mass = (mass / 3) - 2;
//                 if mass <= 0 {
//                     break ans;
//                 }
//                 ans += mass;
//             }
//         })
//         .sum::<i32>()
// }

// fn main() {
// println!("Part one: {}", part_one());
// println!("Part two: {}", part_two());
// }
