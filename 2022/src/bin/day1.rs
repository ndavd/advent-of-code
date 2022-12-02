use aoc::*;

pub fn get_answer(input: Vec<String>) -> (i32, i32) {
    let mut elves: Vec<Vec<i32>> = Vec::new();
    elves.push(Vec::new());
    input.iter().for_each(|value| {
        let food_cal = value.parse::<i32>().unwrap_or(-1);
        if food_cal == -1 {
            elves.push(Vec::new());
        } else {
            elves.last_mut().unwrap().push(food_cal);
        }
    });

    let mut elves_calories: Vec<i32> = Vec::new();

    elves
        .iter()
        .for_each(|elve| elves_calories.push(elve.iter().sum()));

    let biggest_elve_calories = *elves_calories.iter().max().unwrap();
    elves_calories.sort();

    (
        biggest_elve_calories,
        elves_calories[elves_calories.len() - 3..].iter().sum(),
    )
}

fn main() -> Result<(), ()> {
    let day = 1;
    let test_input = get_input(day, true)?;
    assert_eq!(get_answer(test_input), (24000, 45000));

    let input = get_input(day, false)?;
    output_answer(get_answer(input), day)
}
