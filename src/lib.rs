use std::collections::{HashMap, HashSet};
use std::fs;
use std::vec::Vec;

pub struct Game {
    alive: HashSet<(isize, isize)>,
    size: (isize, isize),
    fixed: bool,
    pos_change: HashSet<(isize, isize)>,
}

impl Game {
    pub fn new(args: &Vec<String>, size: (isize, isize), fixed: bool) -> Game {
        let contents = fs::read_to_string(&args[1]).expect("Could not read the file");
        let alive = Self::read(&contents);
        let mut pos_change = alive.clone();
        for (a, b) in &alive {
            Self::check_around((*a, *b), &mut pos_change);
        }

        Game {
            alive,
            pos_change,
            size,
            fixed,
        }
    }

    fn read(contents: &String) -> HashSet<(isize, isize)> {
        let mut alive = HashSet::new();
        for (n, line) in contents.lines().enumerate() {
            println!("{}", line);
            for (m, chr) in line.chars().enumerate() {
                println!("{:?}", (n, m));
                if chr != ' ' && chr != '.' {
                    alive.insert((n.try_into().unwrap(), m.try_into().unwrap()));
                }
            }
        }

        println!("{:?}", alive);

        alive
    }

    pub fn update(&mut self) {
        let mut cnt = HashMap::new();
        for (a, b) in self.pos_change.iter() {
            Self::cnt_around((*a, *b), &self.alive, &mut cnt);
        }

        let mut pos = HashSet::new();
        for (a, b) in self.pos_change.iter() {
            Self::update_cell((*a, *b), &mut self.alive, &cnt, &mut pos);
        }

        self.pos_change = pos;
    }

    fn update_cell(
        (a, b): (isize, isize),
        alive: &mut HashSet<(isize, isize)>,
        cnt: &HashMap<(isize, isize), usize>,
        pos: &mut HashSet<(isize, isize)>,
    ) {
        match cnt.get(&(a, b)) {
            Some(count) => {
                if alive.contains(&(a, b)) {
                    if *count < 3 || *count > 4 {
                        alive.remove(&(a, b));
                        Self::check_around((a, b), pos);
                    }
                } else {
                    if *count == 3 {
                        alive.insert((a, b));
                        Self::check_around((a, b), pos);
                    }
                }
            }
            None => {
                if alive.contains(&(a, b)) {
                    alive.remove(&(a, b));
                    Self::check_around((a, b), pos);
                }
            }
        }
    }

    fn cnt_around(
        (a, b): (isize, isize),
        alive: &HashSet<(isize, isize)>,
        cnt: &mut HashMap<(isize, isize), usize>,
    ) {
        for i in -1..=1 {
            for j in -1..=1 {
                if alive.contains(&(a + i, b + j)) {
                    match cnt.get(&(a, b)) {
                        Some(count) => {
                            cnt.insert((a, b), count + 1);
                        }
                        None => {
                            cnt.insert((a, b), 1);
                        }
                    }
                }
            }
        }
    }

    fn check_around((a, b): (isize, isize), pos: &mut HashSet<(isize, isize)>) {
        for i in -1..=1 {
            for j in -1..=1 {
                pos.insert((a + i, b + j));
            }
        }
    }

    pub fn print(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let (n, m) = self.size;
        let m_usize: usize = m.try_into().unwrap();

        println!("{}", "-".repeat((m_usize + 1) * 2));

        let alive = "■";
        for i in 0isize..n {
            print!("|");
            for j in 0isize..m {
                if self.alive.contains(&(i, j)) {
                    print!(" {alive}");
                } else {
                    print!("  ");
                }
            }
            println!("|");
        }

        println!("{}", "-".repeat((m_usize + 1) * 2));

//        println!("{:?}", self.pos_change);
    }
}
