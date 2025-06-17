fn main() {
    let garden = "AAAA
BBCD
BBCC
EEEC
";
    let (width, height, garden) = garden_from_string(garden.to_owned());
    let price = calculate_fence_price(width, height, &garden);
    println!("Price={}", price)
}

fn garden_from_string(garden: String) -> (usize, usize, Vec<char>) {
    let mut length = 0;
    let mut height = 0;
    let mut resutl = vec![];
    for c in garden.chars() {
        if c != '\n' {
            length += 1;
            resutl.push(c);
        } else {
            height += 1
        }
    }
    return (length / height, height, resutl);
}

fn calculate_fence_price(width: usize, height: usize, garden: &Vec<char>) -> u32 {
    let mut visited: Vec<u8> = vec![0; width * height];
    let total_price = 0;
    // print_matrix(width, garden);
    print_matrix(width, &visited);
    while let Some(index) = get_non_visited_plot(&visited) {
        let letter = garden[index];
        let mut fencesForRegion: Vec<u8> = vec![0; width * height];
        visited[index] = 1; // TODO calculate price for region
    }
    print_matrix(width, &visited);
    return total_price;
}

fn get_non_visited_plot(visited: &Vec<u8>) -> Option<usize> {
    for (i, x) in visited.iter().enumerate() {
        if *x == 0 {
            return Some(i);
        }
    }
    return None;
}

fn print_matrix<T>(width: usize, vector: &Vec<T>)
where
    T: std::fmt::Display,
{
    for (i, x) in vector.iter().enumerate() {
        if i % width == 0 {
            print!("\n");
        }
        print!("{}", x);
    }
    print!("\n");
    print!("\n");
}
