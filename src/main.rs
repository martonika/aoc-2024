pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

fn main() {
    let day1_1 = day1::solve_1("src/day1/input.txt");
    let day1_2 = day1::solve_2("src/day1/input.txt");
    println!("Day 1/1: {day1_1}");
    println!("Day 1/2: {day1_2}");
    println!();

    let day2_1 = day2::solve_1("src/day2/input.txt");
    let day2_2 = day2::solve_2("src/day2/input.txt");
    println!("Day 2/1: {day2_1}");
    println!("Day 2/2: {day2_2}");
    println!();

    let day3_1 = day3::solve_1("src/day3/input.txt");
    let day3_2 = day3::solve_2("src/day3/input.txt");
    println!("Day 3/1: {day3_1}");
    println!("Day 3/2: {day3_2}");
    println!();

    let day4_1 = day4::solve_1("src/day4/input.txt");
    let day4_2 = day4::solve_2("src/day4/input.txt");
    println!("Day 4/1: {day4_1}");
    println!("Day 4/2: {day4_2}");
    println!();

    let day5_1 = day5::solve_1("src/day5/input.txt");
    let day5_2 = day5::solve_2("src/day5/input.txt");
    println!("Day 5/1: {day5_1}");
    println!("Day 5/2: {day5_2}");
    println!();

    let day6_1 = day6::solve_1("src/day6/input.txt");
    // this is way too slow - TODO redo with a proper algorithm
    //let day6_2 = day6::solve_2("src/day6/input.txt");
    println!("Day 6/1: {day6_1}");
    //println!("Day 6/2: {day6_2}");
    println!("Day 6/2: 1951");
    println!();

    let day7_1 = day7::solve_1("src/day7/input.txt");
    let day7_2 = day7::solve_2("src/day7/input.txt");
    println!("Day 7/1: {day7_1}");
    println!("Day 7/2: {day7_2}");
    println!();

    let day8_1 = day8::solve_1("src/day8/input.txt");
    let day8_2 = day8::solve_2("src/day8/input.txt");
    println!("Day 8/1: {day8_1}");
    println!("Day 8/2: {day8_2}");
    println!();

    let day9_1 = day9::solve_1("src/day9/input.txt");
    let day9_2 = day9::solve_2("src/day9/input.txt");
    println!("Day 9/1: {day9_1}");
    println!("Day 9/2: {day9_2}");
    println!();

    let day10_1 = day10::solve_1("src/day10/input.txt");
    let day10_2 = day10::solve_2("src/day10/input.txt");
    println!("Day 10/1: {day10_1}");
    println!("Day 10/2: {day10_2}");
    println!();

    let day11_1 = day11::solve_1("src/day11/input.txt", 25);
    let day11_2 = day11::solve_2("src/day11/input.txt", 75);
    println!("Day 11/1: {day11_1}");
    println!("Day 11/2: {day11_2}");
    println!();

    let day12_1 = day12::solve_1("src/day12/input.txt");
    let day12_2 = day12::solve_2("src/day12/input.txt");
    println!("Day 12/1: {day12_1}");
    println!("Day 12/2: {day12_2}");
    println!();

    let day13_1 = day13::solve_1("src/day13/input.txt");
    let day13_2 = day13::solve_2("src/day13/input.txt");
    println!("Day 13/1: {day13_1}");
    println!("Day 13/2: {day13_2}");
    println!();

    // slow
    //let day14_1 = day14::solve_1("src/day14/input.txt");
    //let day14_2 = day14::solve_2("src/day14/input.txt");
    //println!("Day 14/1: {day14_1}");
    //println!("Day 14/2: {day14_2}");
    println!("Day 14/1: 217328832");
    println!("Day 14/2: 7412");
    println!();

    let day15_1 = day15::solve_1("src/day15/input.txt");
    let day15_2 = day15::solve_2("src/day15/input.txt");
    println!("Day 15/1: {day15_1}");
    println!("Day 15/2: {day15_2}");
    println!();

    let day16_1 = day16::solve_1("src/day16/input.txt");
    let day16_2 = day16::solve_2("src/day16/input.txt");
    println!("Day 16/1: {day16_1}");
    println!("Day 16/2: {day16_2}");
    println!();

    let day17_1 = day17::solve_1("src/day17/input.txt");
    let day17_2 = day17::solve_2("src/day17/input.txt");
    println!("Day 17/1: {day17_1}");
    println!("Day 17/2: {day17_2}");
    println!();

    let day18_1 = day18::solve_1("src/day18/input.txt");
    let day18_2 = day18::solve_2("src/day18/input.txt");
    println!("Day 18/1: {day18_1}");
    println!("Day 18/2: {:?}", day18_2);
    println!();

    let day19_1 = day19::solve_1("src/day19/input.txt");
    let day19_2 = day19::solve_2("src/day19/input.txt");
    println!("Day 19/1: {day19_1}");
    println!("Day 19/2: {day19_2}");
    println!();

    let day20_1 = day20::solve_1("src/day20/input.txt");
    let day20_2 = day20::solve_2("src/day20/input.txt");
    println!("Day 20/1: {day20_1}");
    println!("Day 20/2: {day20_2}");
    println!();

    let day21_1 = day21::solve_1("src/day21/input.txt");
    let day21_2 = day21::solve_2("src/day21/input.txt");
    println!("Day 21/1: {day21_1}");
    println!("Day 21/2: {day21_2}");
    println!();

    let day22_1 = day22::solve_1("src/day22/input.txt");
    let day22_2 = day22::solve_2("src/day22/input.txt");
    println!("Day 22/1: {day22_1}");
    println!("Day 22/2: {day22_2}");
    println!();

    let day23_1 = day23::solve_1("src/day23/input.txt");
    let day23_2 = day23::solve_2("src/day23/input.txt");
    println!("Day 23/1: {day23_1}");
    println!("Day 23/2: {day23_2}");
    println!();

    let day24_1 = day24::solve_1("src/day24/input.txt");
    let day24_2 = day24::solve_2("src/day24/input.txt");
    println!("Day 24/1: {day24_1}");
    println!("Day 24/2: {day24_2}");
    println!();
}
