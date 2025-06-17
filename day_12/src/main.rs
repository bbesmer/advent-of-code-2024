fn main() {
    println!("Hello, world!");
    let garden = "AAAA
BBCD
BBCC
EEEC
";
    let (width, height, garden) = garden_from_string(garden.to_owned());
    let price = calculate_fence_price(width, height, garden);
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
        }
        else {
            height += 1
        }
    }
    return (length / height, height, resutl);
}

fn calculate_fence_price(width: usize, height: usize, garden: Vec<char>) -> usize {
    let mut visited: Vec<u8> = vec![0; width * height];
    // print_matrix(width, garden);
    // print_matrix(width, visited);
    0
}

fn print_matrix<T>(width: usize, vector: Vec<T>)
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
