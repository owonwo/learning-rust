pub fn practice(nums: Vec<usize>, index: usize) -> usize {
    let index_value = nums.get(index);

    // let p_value = match index_value {
    //     Some(v) => v * 5,
    //     None => index * 5
    // };

    let p_value = index_value.unwrap_or(&index) * 5;

    println!("The practice value is {}", p_value);

    return p_value;
}

pub fn log_lines() {
    let file_path = std::env::args().nth(1).expect("Name argument required");

    let file = std::fs::read_to_string(file_path).expect("We couldn't find the file");

    file.lines().enumerate().for_each(|(line_no, x)| {
        if let Ok(value) = x.parse::<usize>() {
            println!("{}: {}", line_no, value);
        } else {
            println!("{}: {}", line_no, "Not a number");
        }
    })
}
