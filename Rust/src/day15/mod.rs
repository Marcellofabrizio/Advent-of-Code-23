
pub fn solve1(input: &String) {
    let binding = input.replace("\n", "");
    let contents: Vec<_>= binding.split(",").collect();
    println!("{:?}", contents.into_iter().map(|c| hash(c)).sum::<u32>());
}

fn hash(input: &str) -> u32 {
    
    let mut hashed_value = 0;

    input.chars().for_each(|c| {
        hashed_value += c as u32;
        hashed_value = hashed_value * 17;
        hashed_value = hashed_value % 256;
    });

    hashed_value
}
