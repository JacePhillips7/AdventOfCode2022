fn main() {
    let mut elves: Vec<u32> = vec![]; // vector of elves by total calories
                                      // read input file
    let input = std::fs::read_to_string("input.txt").unwrap();
    // parse input file split on new line
    let line = input.split("\n");
    // loop through each line
    let mut value = 0;
    for i in line {
        if i.trim().is_empty() || i == String::from('\n') {
            elves.push(value);
            value = 0;
        } else {
            let calories: u32 = i.trim().parse().expect("Failed to tally calories: {i}");
            value += calories;
        }
    }
    //sort vector
    elves.sort();
    //print the last one
    println!("{}", elves[elves.len()-1]); //71506
    let len = elves.len();
    let top3 = elves[len-1] + elves[len-2] + elves[len-3];
    println!("{} is your top 3", top3);
}
