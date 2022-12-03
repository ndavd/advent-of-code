pub fn get_answer(input: aoc::Input) -> aoc::Answer {
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
    aoc::AoC::new(1, 24000, 45000).compute(&get_answer)
}
