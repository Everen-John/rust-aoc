use lazy_static::{__Deref, lazy_static};
use regex::Regex;
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashSet,
    collections::LinkedList,
    fs::{self},
    ops::DerefMut,
    rc::Rc,
};

#[derive(Clone, PartialEq, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl ToString for Point {
    fn to_string(&self) -> String {
        format!("{},{}", &self.x, &self.y)
    }
}

fn read_input() -> String {
    let file_path = "public/day9test.txt";
    fs::read_to_string(file_path).expect("Should have been able to read the file!")
}

lazy_static! {
    static ref LINE_PATTERN: Regex = Regex::new(r"([A-Z])\s(\d+)").unwrap();
}

pub fn question_one() {
    let binding = read_input();
    let content: Vec<String> = binding.split('\n').map(|x| x.to_string()).collect();

    let mut h = Point { x: 0, y: 0 };
    let mut h_old = Point { x: 0, y: 0 };
    let mut t = Point { x: 0, y: 0 };
    let mut areas_t_visited: HashSet<String> = HashSet::new();
    areas_t_visited.insert(t.to_string());

    content.iter().for_each(|instruction| {
        let caps = LINE_PATTERN.captures(instruction).unwrap();
        let direction = caps.get(1).unwrap().as_str();
        let magnitude: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        println!("Going {} {} times", direction, magnitude);
        for _ in 1..=magnitude {
            h_old = h;
            match direction {
                "U" => {
                    h.x -= 1;
                }
                "D" => {
                    h.x += 1;
                }
                "L" => {
                    h.y -= 1;
                }
                "R" => {
                    h.y += 1;
                }
                _ => (),
            }
            println!(
                "H moves to {} while T is at {}",
                h.to_string(),
                t.to_string()
            );

            if !t_is_around_h_flag(&h, &t) {
                t = h_old;
                println!("T has visited {}", t.to_string());
                areas_t_visited.insert(t.to_string());
            }
        }
    });
    println!(
        "T has visited {} locations at least once",
        areas_t_visited.len()
    );
}

fn t_is_around_h_flag(h: &Point, t: &Point) -> bool {
    let on_top = h.clone();
    let top_left = Point {
        x: h.x - 1,
        y: h.y - 1,
    };
    let top = Point { x: h.x - 1, y: h.y };
    let top_right = Point {
        x: h.x - 1,
        y: h.y + 1,
    };
    let right = Point { x: h.x, y: h.y + 1 };
    let bottom_right = Point {
        x: h.x + 1,
        y: h.y + 1,
    };
    let bottom = Point { x: h.x + 1, y: h.y };
    let bottom_left = Point {
        x: h.x + 1,
        y: h.y - 1,
    };
    let left = Point { x: h.x, y: h.y - 1 };

    let boundaries: [Point; 9] = [
        on_top,
        top_left,
        top,
        top_right,
        right,
        bottom_right,
        bottom,
        bottom_left,
        left,
    ];
    boundaries.iter().any(|Point| {
        if Point.eq(t) {
            println!("T ({}) is around H ({})", t.to_string(), h.to_string());
            true
        } else {
            false
        }
    })
}

#[derive(PartialEq)]
struct LinkedPoint {
    index: i8,
    x_current: i32,
    y_current: i32,
    x_old: Option<i32>,
    y_old: Option<i32>,
}

impl ToString for LinkedPoint {
    fn to_string(&self) -> String {
        format!("({},{})", self.x_current, self.y_current)
    }
}

impl LinkedPoint {
    fn old_to_string(&self) -> String {
        format!("({},{})", self.x_old.unwrap_or(0), self.y_old.unwrap_or(0))
    }

    fn new(i: i8) -> Self {
        LinkedPoint {
            index: i,
            x_current: 0,
            y_current: 0,
            x_old: None,
            y_old: None,
        }
    }

    fn current_to_old(&mut self) {
        self.x_old = Some(self.x_current);
        self.y_old = Some(self.y_current);
    }

    fn eq(&self, other: &LinkedPoint) -> bool {
        self.x_current == other.x_current && other.y_current == self.y_current
    }

    fn assign_new_point(&mut self, other: LinkedPoint) {
        self.x_current = other.x_old.unwrap();
        self.y_current = other.y_old.unwrap();
    }
}

pub fn question_two() {
    let binding = read_input();
    let content: Vec<String> = binding.split('\n').map(|x| x.to_string()).collect();

    let mut linked_points: LinkedList<LinkedPoint> = LinkedList::new();

    for i in 0..9 {
        let linked_point = LinkedPoint::new(i);
        linked_points.push_back(linked_point);
    }

    let mut areas_t_visited: HashSet<String> = HashSet::new();
    areas_t_visited.insert(linked_points.back().unwrap().to_string());

    content.iter().for_each(|instruction| {
        let caps = LINE_PATTERN.captures(instruction).unwrap();
        let direction = caps.get(1).unwrap().as_str();
        let magnitude: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        println!("Going {} {} times", direction, magnitude);
        for i in 1..=magnitude {
            println!("Going R {} more times", magnitude - i);
            {
                let mut head = linked_points.front_mut().unwrap();
                // h.current_to_old();
                head.x_old = Some(head.x_current);
                head.y_old = Some(head.y_current);
                // h.current_to_old();
                match direction {
                    "U" => {
                        head.x_current -= 1;
                    }
                    "D" => {
                        head.x_current += 1;
                    }
                    "L" => {
                        head.y_current -= 1;
                    }
                    "R" => {
                        head.y_current += 1;
                    }
                    _ => (),
                }
                println!("Head is now at {} ", head.to_string());
            }

            let mut lp_iter = linked_points.iter();
            let mut h_option = lp_iter.next();
            let mut t_option = lp_iter.next();

            loop {
                if t_option.is_some() {
                    let mut h_ref = h_option.unwrap();
                    let mut h = h_ref.borrow_mut();
                    let mut t_ref = t_option.unwrap();
                    let mut t = t_ref.borrow_mut();

                    if !linked_t_is_around_h_flag(&h, &t) {
                        t.current_to_old();

                        t.x_current = h.x_old.unwrap();
                        t.y_current = h.y_old.unwrap();

                        if t.index == 8 {
                            println!("Point {} has visited {}", t.index, t.to_string());
                            areas_t_visited.insert(t.to_string());
                        }
                    }
                    h_option = t_option;
                    t_option = lp_iter.next();
                } else {
                    break;
                }
            }
        }
    });
    println!(
        "T has visited {} locations at least once",
        areas_t_visited.len()
    );
}

fn linked_t_is_around_h_flag(h: &LinkedPoint, t: &LinkedPoint) -> bool {
    let on_top = LinkedPoint {
        index: 0,
        x_current: h.x_current,
        y_current: h.y_current,
        x_old: None,
        y_old: None,
    };
    let top_left = LinkedPoint {
        index: 0,
        x_current: h.x_current - 1,
        y_current: h.y_current - 1,
        x_old: None,
        y_old: None,
    };
    let top = LinkedPoint {
        index: 0,
        x_current: h.x_current - 1,
        y_current: h.y_current,
        x_old: None,
        y_old: None,
    };
    let top_right = LinkedPoint {
        index: 0,
        x_current: h.x_current - 1,
        y_current: h.y_current + 1,
        x_old: None,
        y_old: None,
    };
    let right = LinkedPoint {
        index: 0,
        x_current: h.x_current,
        y_current: h.y_current + 1,
        x_old: None,
        y_old: None,
    };
    let bottom_right = LinkedPoint {
        index: 0,
        x_current: h.x_current + 1,
        y_current: h.y_current + 1,
        x_old: None,
        y_old: None,
    };
    let bottom = LinkedPoint {
        index: 0,
        x_current: h.x_current + 1,
        y_current: h.y_current,
        x_old: None,
        y_old: None,
    };
    let bottom_left = LinkedPoint {
        index: 0,
        x_current: h.x_current + 1,
        y_current: h.y_current - 1,
        x_old: None,
        y_old: None,
    };
    let left = LinkedPoint {
        index: 0,
        x_current: h.x_current,
        y_current: h.y_current - 1,
        x_old: None,
        y_old: None,
    };

    let boundaries: [LinkedPoint; 9] = [
        on_top,
        top_left,
        top,
        top_right,
        right,
        bottom_right,
        bottom,
        bottom_left,
        left,
    ];
    boundaries.iter().any(|point| {
        if point.eq(t) {
            println!(
                "T {} ({}) is around H {} ({})",
                t.index,
                t.to_string(),
                h.index,
                h.to_string()
            );
            true
        } else {
            false
        }
    })
}
