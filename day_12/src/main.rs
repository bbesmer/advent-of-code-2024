use std::{fs::File, io::Read, path::Path, time::Instant};

fn main() {
    let path = Path::new("./input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut garden = String::new();
    if let Err(why) = file.read_to_string(&mut garden) {
        panic!("couldn't read {}: {}", path.display(), why);
    }
//     let garden = "OOOOO
// OXOXO
// OOOOO
// OXOXO
// OOOOO
// ";
    let iterations = 1;
    let start = Instant::now();
    let mut price = 0;
    for _ in 0..iterations {
        let (width, height, garden) = garden_from_string(garden.to_owned());
        price = calculate_total_fence_price(width, height, &garden);
    }
    let duration = start.elapsed();
    println!("Time per iteration: {:?}", duration / iterations);
    println!("Price={}", price);
    assert_eq!(1370258, price);
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

fn calculate_total_fence_price(width: usize, height: usize, garden: &Vec<char>) -> u32 {
    let mut visited: Vec<bool> = vec![false; width * height];
    let mut total_price = 0;
    // print_matrix(width, &garden);
    while let Some(index) = get_non_visited_plot(&visited) {
        let regional_price =
            calculate_regional_fence_price(width, height, garden, &mut visited, index);
        total_price += regional_price;
    }
    return total_price;
}

fn get_non_visited_plot(visited: &Vec<bool>) -> Option<usize> {
    for (i, x) in visited.iter().enumerate() {
        if !*x {
            return Some(i);
        }
    }
    return None;
}

fn calculate_regional_fence_price(
    width: usize,
    height: usize,
    garden: &Vec<char>,
    visited: &mut Vec<bool>,
    index: usize,
) -> u32 {
    let letter = garden[index];
    visited[index] = true;
    let mut queue = vec![index];

    let mut area = 0;
    let mut fences = 0;
    while let Some(index) = queue.pop() {
        let mut neighbors: Vec<usize> = vec![];
        area += 1;
        // left
        if index % width == 0 {
            fences += 1;
        } else {
            neighbors.push(index - 1);
        }

        // right
        if index % width == width - 1 {
            fences += 1;
        } else {
            neighbors.push(index + 1);
        }

        // top
        if index / width == 0 {
            fences += 1;
        } else {
            neighbors.push(index - width);
        }

        //bottom
        if index / width == height - 1 {
            fences += 1;
        } else {
            neighbors.push(index + width);
        }

        for neighbor in neighbors {
            if garden[neighbor] != letter {
                fences += 1;
            } else if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push(neighbor)
            }
        }
    }
    // println!("Region {}: Area={}, Fences={}", letter, area, fences);
    return area * fences;
}

fn _print_matrix<T>(width: usize, vector: &Vec<T>)
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
}
