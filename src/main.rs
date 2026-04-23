fn print_rod_print_line(a: i32, b: i32, c: i32) {
    let a_str = if a > 0 { a.to_string() } else { "|".to_string() };
    let b_str = if b > 0 { b.to_string() } else { "|".to_string() };
    let c_str = if c > 0 { c.to_string() } else { "|".to_string() };

    println!("{:^4} {:^4} {:^4}", a_str, b_str, c_str);
}

fn print_rods(rod_a: [i32; 10], rod_b: [i32; 10], rod_c: [i32; 10]) {
    for i in 0..10 {
        print_rod_print_line(rod_a[i], rod_b[i], rod_c[i]);
    }
    println!("{:^4} {:^4} {:^4}", "----", "----", "----");
}

fn is_valid_move(rod_left: [i32; 10], rod_right: [i32; 10]) -> bool {
    let mut top_left_id = 0;
    let mut top_left = rod_left[top_left_id];

    while top_left == 0 && top_left_id < 9 {
        top_left_id += 1;
        top_left = rod_left[top_left_id];
    }

    let mut top_right_id = 0;
    let mut top_right = rod_right[top_right_id];

    while top_right == 0 && top_right_id < 9 {
        top_right_id += 1;
        top_right = rod_right[top_right_id];
    }

    return top_right == 0 || top_left < top_right;
}

fn move_disk_between_rods(rod_left: &mut [i32; 10], rod_right: &mut [i32; 10]) {
    let mut top_left_id = 0;
    let mut top_left = rod_left[top_left_id];

    while top_left == 0 && top_left_id < 9 {
        top_left_id += 1;
        top_left = rod_left[top_left_id];
    }

    let mut top_right_id = 0;
    let mut top_right = rod_right[top_right_id];

    while top_right == 0 && top_right_id < 9 {
        top_right_id += 1;
        top_right = rod_right[top_right_id];
    }

    if top_right == 0 {
        rod_right[top_right_id] = top_left;
    } else if top_right_id > 0 {
        rod_right[top_right_id - 1] = top_left;
    } else {
        panic!("The right rod is full. Cannot place a disk on it.")
    }

    rod_left[top_left_id as usize] = 0;
}

fn move_disk(rod_source: &mut [i32; 10], rod_aux: &mut [i32; 10], rod_target: &mut [i32; 10], n: i32) {
    if n == 1 {
        move_disk_between_rods(rod_source, rod_target);
    } else {
        move_disk(rod_source, rod_target, rod_aux, n - 1);

        move_disk_between_rods(rod_source, rod_target);

        move_disk(rod_aux, rod_source, rod_target, n - 1);
    }
}

fn main() {
    let mut rod_a: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut rod_b: [i32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut rod_c: [i32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    println!("Initial state");
    print_rods(rod_a, rod_b, rod_c);

    move_disk(&mut rod_a, &mut rod_b, &mut rod_c, 10);

    println!("Final state");
    print_rods(rod_a, rod_b, rod_c);
}
