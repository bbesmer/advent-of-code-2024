mod garden;

use std::{fmt::Display, fs::File, io::Read, path::Path, time::Instant};

use crate::garden::{Garden, Plot};

fn main() {
    let path = Path::new("./input.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut garden = String::new();
    if let Err(why) = file.read_to_string(&mut garden) {
        panic!("couldn't read {}: {}", path.display(), why);
    }
    
    // let garden = "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO\n";
//     let garden = "AAAA
// BBCD
// BBCC
// EEEC
// ";
    let discounted = true;
    let iterations = 5;
    let start = Instant::now();
    let mut price = 0;
    for _ in 0..iterations {
        let garden = Garden::new(garden.to_owned());
        price = calculate_total_fence_price(&garden, discounted);
    }
    let duration = start.elapsed();
    println!("Time per iteration: {:?}", duration / iterations);
    println!("Price={}", price);
    if discounted {
        assert_eq!(805814, price);
    } else {
        assert_eq!(1370258, price);
    }
}

fn calculate_total_fence_price(garden: &Garden, discounted: bool) -> u32 {
    let mut visited = vec![false; garden.width * garden.height];
    let mut total_price = 0;
    while let Some(plot) = get_non_visited_plot(&visited, &garden) {
        let regional_price =
            calculate_regional_fence_price(&garden, &mut visited, plot, discounted);
        total_price += regional_price;
    }
    return total_price;
}

fn get_non_visited_plot<'a>(visited: &Vec<bool>, garden: &'a Garden) -> Option<Plot<'a>> {
    for (i, &x) in visited.iter().enumerate() {
        if !x {
            return Some(Plot::new(&garden, i % garden.width, i / garden.width));
        }
    }
    return None;
}

fn calculate_regional_fence_price(
    garden: &Garden,
    visited: &mut Vec<bool>,
    plot: Plot,
    discounted: bool
) -> u32 {
    visited[plot.x + plot.y*garden.width] = true;
    let mut stack = vec![plot];

    let mut area = 0;
    let mut fences = 0;
    let mut inner_edges = 0;
    let mut outer_edges = 0;
    while let Some(plot) = stack.pop() {
        // println!("{},{},{}", plot.letter, plot.x, plot.y);
        let extends_left = plot.extends_left();
        if extends_left {
            if !visited[plot.x - 1 + plot.y*garden.width]{
                visited[plot.x - 1  + plot.y*garden.width] = true;
                stack.push(Plot::new(&garden, plot.x - 1, plot.y));
            }
        }

        let extends_right = plot.extends_right();
        if extends_right {
            if !visited[plot.x + 1 + plot.y*garden.width] {
                visited[plot.x + 1 + plot.y*garden.width] = true;
                stack.push(Plot::new(&garden, plot.x + 1, plot.y));
            }
        }

        let extends_top = plot.extends_top();
        if extends_top {
            if !visited[plot.x + plot.y*garden.width - garden.width] {
                visited[plot.x + plot.y*garden.width - garden.width] = true;
                stack.push(Plot::new(&garden, plot.x, plot.y - 1));
            }
        }
        
        let extends_bottom = plot.extends_bottom();
        if extends_bottom {
            if !visited[plot.x + plot.y*garden.width + garden.width] {
                visited[plot.x + plot.y*garden.width + garden.width] = true;
                stack.push(Plot::new(&garden, plot.x, plot.y + 1));
            }
        }

        area += 1;
        let fence_conditions = [!extends_top, !extends_right, !extends_left, !extends_bottom];
        // println!("{:?}", fence_conditions);
        fences += fence_conditions
            .into_iter()
            .filter(|&x| x)
            .count();

        let outer_edge_conditions = [
            !extends_bottom & !extends_left,
            !extends_left & !extends_top,
            !extends_top & !extends_right,
            !extends_right & !extends_bottom
        ];
        outer_edges += outer_edge_conditions
            .into_iter()
            .fold(0, |acc, b| if b { acc + 1 } else { acc });
        
        let extends_top_left = plot.extends_top_left();
        let extends_bottom_left = plot.extends_bottom_left();
        let extends_top_right = plot.extends_top_right();
        let extends_bottom_right = plot.extends_bottom_right();

        let inner_edge_conditions = [
            extends_bottom & extends_left & !extends_bottom_left,
            extends_bottom & !extends_left & extends_bottom_left,
            !extends_bottom & extends_left & extends_bottom_left,
            extends_left & extends_top & !extends_top_left,
            extends_left & !extends_top & extends_top_left,
            !extends_left & extends_top & extends_top_left,
            extends_top & extends_right & !extends_top_right,
            extends_top & !extends_right & extends_top_right,
            !extends_top & extends_right & extends_top_right,
            extends_right & extends_bottom & !extends_bottom_right,
            extends_right & !extends_bottom & extends_bottom_right,
            !extends_right & extends_bottom & extends_bottom_right,
        ];
        inner_edges += inner_edge_conditions
            .iter()
            .fold(0, |acc, &b| if b { acc + 1 } else { acc });

    }
    // println!("Region {}: Area={}, Fences={}, InnerEdges={}, OuterEdges={}", '?', area, fences, inner_edges, outer_edges);
    let price;
    if discounted {
        price = area * (outer_edges + inner_edges / 3);
    }
    else {
        price = area * fences;
    }
    return price;
}

fn _print_matrix<T>(width: usize, vector: &Vec<T>)
where
    T: Display,
{
    for (i, x) in vector.iter().enumerate() {
        if i % width == 0 {
            print!("\n");
        }
        print!("{}", x);
    }
    print!("\n");
}
