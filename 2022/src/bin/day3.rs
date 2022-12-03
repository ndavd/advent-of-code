#[derive(Clone)]
struct Rucksack {
    left_compartment: String,
    right_compartment: String,
}

trait RucksackTrait {
    type Item;
    type Priority;

    fn new(s: &String) -> Result<self::Rucksack, &str>;
    fn get_item_priority(item: Self::Item) -> Result<Self::Priority, &'static str>;

    fn get_items(&mut self) -> (Vec<Self::Item>, Vec<Self::Item>);
    fn get_common_item(&mut self) -> Result<Self::Item, &str>;

    fn get_group_common_item(group: &[String]) -> Result<Self::Item, &str>;
}

impl RucksackTrait for Rucksack {
    type Item = char;
    type Priority = i32;

    fn new(s: &String) -> Result<Self, &str> {
        let (left_compartment, right_compartment) = s.split_at(s.len() / 2);
        if left_compartment.len() != right_compartment.len() {
            return Err("Rucksack::new::Left and right compartments must have the same length");
        }

        Ok(Rucksack {
            left_compartment: left_compartment.to_string(),
            right_compartment: right_compartment.to_string(),
        })
    }

    fn get_item_priority(item: Self::Item) -> Result<i32, &'static str> {
        let lowercase_a_value = 'a' as i32;
        let uppercase_a_value = 'A' as i32;
        let item_value = item as i32;

        match item {
            'a'..='z' => Ok(item_value - lowercase_a_value + 1),
            'A'..='Z' => Ok(item_value - uppercase_a_value + 27),
            _ => Err("Rucksack::get_item_priority::Invalid char"),
        }
    }

    fn get_items(&mut self) -> (Vec<char>, Vec<char>) {
        (
            self.left_compartment
                .chars()
                .collect::<Vec<Self::Item>>()
                .clone(),
            self.right_compartment
                .chars()
                .collect::<Vec<Self::Item>>()
                .clone(),
        )
    }

    fn get_common_item(&mut self) -> Result<Self::Item, &str> {
        let (left_compartment_items, right_compartment_items) = self.get_items();
        let mut common_item: Option<Self::Item> = None;

        left_compartment_items.iter().for_each(|l_item| {
            right_compartment_items.iter().for_each(|r_item| {
                if l_item == r_item {
                    common_item = Some(l_item.clone());
                }
            })
        });

        common_item.ok_or("Rucksack::get_common_item::There is no common item")
    }

    fn get_group_common_item(group: &[String]) -> Result<Self::Item, &str> {
        if group.len() != 3 {
            return Err("Rucksack::get_group_common_item::A group must have 3 elements");
        }
        let mut common_item: Option<Self::Item> = None;

        let first_elf_items: Vec<char> = group[0].chars().collect();

        for item in first_elf_items {
            if group[1].contains(item) && group[2].contains(item) {
                common_item = Some(item);
                break;
            }
        }

        common_item.ok_or("Rucksack::get_group_common_item::There is no common item")
    }
}

pub fn get_answer(input: aoc::Input) -> aoc::Answer {
    let mut common_items_priority = 0;

    input.iter().for_each(|rucksack_as_string| {
        let mut rucksack = Rucksack::new(rucksack_as_string).unwrap();
        let common_item = rucksack.get_common_item().unwrap();
        common_items_priority += Rucksack::get_item_priority(common_item).unwrap();
    });

    if input.len() % 3 != 0 {
        panic!("Can't make groups of 3")
    }
    let mut group_common_items_priority = 0;

    for i in (0..input.len()).step_by(3) {
        let group = &input[i..i + 3];

        group_common_items_priority +=
            Rucksack::get_item_priority(Rucksack::get_group_common_item(group).unwrap()).unwrap()
    }

    (common_items_priority, group_common_items_priority)
}

fn main() -> Result<(), ()> {
    aoc::AoC::new(3, 157, 70).compute(&get_answer)
}
