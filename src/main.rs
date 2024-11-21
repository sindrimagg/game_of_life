use std::collections::HashMap;
use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut alive = HashSet::new();

    alive.insert((4, 5));
    alive.insert((4, 4));
    alive.insert((4, 3));
    alive.insert((3, 5));
    alive.insert((2, 4));
    //    alive.insert((6, 5));
    //    alive.insert((6, 6));
    //    alive.insert((5, 6));

    let mut pos_change = HashSet::new();
    for (a, b) in &alive {
        check_around((*a, *b), &mut pos_change);
    }

    loop {
        print_world(30, &alive);
        sleep(Duration::from_millis(100));
        let mut cnt = HashMap::new();
        for (a, b) in &pos_change {
            if alive.contains(&(*a, *b)) {
                for i in -1..=1 {
                    for j in -1..=1 {
                        match cnt.get(&(a + i, b + j)) {
                            Some(count) => {
                                cnt.insert((a + i, b + j), count + 1);
                            }
                            None => {
                                cnt.insert((a + i, b + j), 1);
                            }
                        }
                    }
                }
            }
        }

        let mut pos = HashSet::new();
        for (a, b) in &pos_change {
            match cnt.get(&(*a, *b)) {
                Some(count) => {
                    if alive.contains(&(*a, *b)) {
                        if *count < 3 || *count > 4 {
                            alive.remove(&(*a, *b));
                            check_around((*a, *b), &mut pos);
                        }
                    } else {
                        if *count == 3 {
                            alive.insert((*a, *b));
                            check_around((*a, *b), &mut pos);
                        }
                    }
                }
                None => {
                    continue;
                }
            }
        }

        pos_change = pos;
    }
}

fn check_around((a, b): (isize, isize), pos: &mut HashSet<(isize, isize)>) {
    for i in -1..=1 {
        for j in -1..=1 {
            pos.insert((a + i, b + j));
        }
    }
}

fn print_world(n: isize, world: &HashSet<(isize, isize)>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let n_usize: usize = n.try_into().unwrap();
    println!("{}", "-".repeat(n_usize * 2 + 1));
    let alive = "■";
    for i in 0isize..n {
        for j in 0isize..n {
            if world.contains(&(i, j)) {
                print!("|{alive}");
            } else {
                print!("| ");
            }
        }
        println!("|");
    }
    println!("{}", "-".repeat(n_usize * 2 + 1));
}
