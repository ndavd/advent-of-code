pub fn get_input(day: usize, is_test_data: bool) -> Result<Vec<String>, ()> {
    Ok(std::fs::read_to_string(format!(
        "./src/data/day{}.{}.txt",
        day,
        if is_test_data { "test" } else { "input" }
    ))
    .unwrap()
    .lines()
    .map(|l| l.to_string())
    .collect())
}

pub fn output_answer<T: std::fmt::Debug>(answer: T, day: usize) -> Result<(), ()> {
    println!("Answer from day {}: {:?}", day, answer);

    Ok(())
}
