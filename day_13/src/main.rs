fn main() {
    let orthogonal_machine = ClawMachine{ x_a: 1, y_a: 0, x_b: 0, y_b: 1, x_p: 3, y_p: 5 };
    let overshoot_machine = ClawMachine{ x_a: 2, y_a: 0, x_b: 0, y_b: 2, x_p: 3, y_p: 5 };
    let negative_machine = ClawMachine{ x_a: 1, y_a: 1, x_b: 0, y_b: 1, x_p: 3, y_p: 0 };
    let parallel_winning_machine = ClawMachine{ x_a: 1, y_a: 2, x_b: 1, y_b: 2, x_p: 2, y_p: 4 };
    let parallel_overshoot_machine = ClawMachine{ x_a: 2, y_a: 2, x_b: 2, y_b: 2, x_p: 3, y_p: 3 };
    let parallel_missing_machine = ClawMachine{ x_a: 1, y_a: 2, x_b: 1, y_b: 2, x_p: 1, y_p: 4 };

    let machines = [
        orthogonal_machine,
        overshoot_machine,
        negative_machine,
        parallel_winning_machine,
        parallel_overshoot_machine,
        parallel_missing_machine
    ];

    for machine in machines {
        solve(machine);
    }
}

#[derive(Debug)]
struct ClawMachine {
    x_a: i32,
    y_a: i32,
    x_b: i32,
    y_b: i32,
    x_p: i32,
    y_p: i32,
}

fn price(machine: ClawMachine) -> i32{
    return 0;
}

fn solve(machine: ClawMachine) {
    let determinant = machine.x_a * machine.y_b - machine.x_b * machine.y_a;
    if determinant == 0 {
        println!("{:?} d={}", machine, determinant);
    }
    else {
        let n = ((machine.y_b * machine.x_p - machine.x_b * machine.y_p) as f32) / (determinant as f32);
        let m = ((machine.x_a * machine.y_p - machine.y_a * machine.x_p) as f32) / (determinant as f32);
        println!("{:?} d={}, n={}, m={}", machine, determinant, n, m);
    }
}
