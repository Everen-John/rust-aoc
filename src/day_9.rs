use lazy_static::lazy_static;
use regex::Regex;
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashSet,
    fs::{self},
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

#[derive(PartialEq, Debug, Clone)]
struct LinkedPoint {
    index: i32,
    x_current: i32,
    y_current: i32,
    x_old: Option<i32>,
    y_old: Option<i32>,
    // head: Option<Box<LinkedPoint>>,
    tail: Option<Box<LinkedPoint>>,
}

impl LinkedPoint {
    fn new(i: i32) -> Self {
        LinkedPoint {
            index: i,
            x_current: 0,
            y_current: 0,
            x_old: None,
            y_old: None,
            tail: None,
        }
    }

    fn next(self) -> Option<Box<LinkedPoint>> {
        self.tail
    }

    fn peek(&self) -> Option<Box<LinkedPoint>> {
        self.tail.clone()
    }

    fn old_string(&self) -> String {
        format!("{},{}", &self.x_old.unwrap(), &self.y_old.unwrap()).to_string()
    }

    fn eq(self, linked_point: Self) -> bool {
        linked_point.x_current == self.x_current && linked_point.y_current == self.y_current
    }

    fn index(&self) -> &i32 {
        &self.index
    }
}

impl ToString for LinkedPoint {
    fn to_string(&self) -> String {
        format!("{},{}", &self.x_current, &self.y_current).to_string()
    }
}

pub fn question_two() {
    let binding = read_input();
    let content: Vec<String> = binding.split('\n').map(|x| x.to_string()).collect();

    let mut areas_point_9_visited: HashSet<String> = HashSet::new();
    areas_point_9_visited.insert("0,0".to_string());
    let mut points: Vec<LinkedPoint> = Vec::new();

    let mut linked_points = build_linked_points(9).unwrap();
    println!("{:?}", linked_points);

    content.iter().for_each(|instruction| {
        let caps = LINE_PATTERN.captures(instruction).unwrap();
        let direction = caps.get(1).unwrap().as_str();
        let magnitude: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        println!("Going {} {} times", direction, magnitude);
        // let mut h = *points.get(0).unwrap();
        let mut h = &mut linked_points;
        for _ in 1..=magnitude {
            (h.x_old, h.y_old) = (Some(h.x_current), Some(h.y_current));
            match direction {
                "U" => {
                    h.x_current -= 1;
                }
                "D" => {
                    h.x_current += 1;
                }
                "L" => {
                    h.y_current -= 1;
                }
                "R" => {
                    h.y_current += 1;
                }
                _ => (),
            }
            println!("Point 9 moves from {} to {}", h.old_string(), h.to_string());

            // let mut tail = h.peek().unwrap();
            let mut tail = h;

            while true {
                let mut nt_option = tail.next();
                match nt_option {
                    Some(ref mut nt) => {
                        if !linked_point_t_is_around_h(&tail, &nt) {
                            println!(
                                "point {} ({}) is going to move to where point {} ({}) was",
                                nt.index,
                                nt.to_string(),
                                tail.index,
                                tail.to_string()
                            );
                            nt.x_old = Some(nt.x_current);
                            nt.y_old = Some(nt.y_current);
                            nt.x_current = tail.clone().x_old.unwrap();
                            nt.y_current = tail.clone().y_old.unwrap();
                            println!(
                                "point {} ({}) and point {} ({})",
                                nt.index,
                                nt.to_string(),
                                tail.index,
                                tail.to_string()
                            );

                            if nt.peek().is_none() {
                                println!("point 0 has visited {}!", nt.to_string());
                                areas_point_9_visited.insert(nt.to_string());
                            }
                        }
                        tail = nt;
                    }
                    None => break,
                }
            }
        }
    });
    println!(
        "T has visited {} locations at least once",
        areas_point_9_visited.len()
    );
}

fn build_linked_points<'a>(i: i32) -> Option<Box<LinkedPoint>> {
    if i > 0 {
        let mut point = LinkedPoint::new(i);
        point.tail = build_linked_points(i - 1);
        Some(Box::new(point))
        // point
    } else {
        Some(Box::new(LinkedPoint::new(i)))
    }
}

fn linked_point_t_is_around_h(h: &Box<LinkedPoint>, t: &Box<LinkedPoint>) -> bool {
    let on_top = LinkedPoint {
        index: 0,
        x_current: h.x_current,
        y_current: h.y_current,
        x_old: h.x_old,
        y_old: h.y_old,
        tail: None,
    };
    let top_left = LinkedPoint {
        index: 0,
        x_current: h.x_current - 1,
        y_current: h.y_current - 1,
        x_old: None,
        y_old: None,
        tail: None,
    };
    let top = LinkedPoint {
        index: 0,
        x_current: h.x_current - 1,
        y_current: h.y_current,
        x_old: None,
        y_old: None,
        tail: None,
    };
    let top_right = LinkedPoint {
        index: 0,
        x_current: h.x_current - 1,
        y_current: h.y_current + 1,
        x_old: None,
        y_old: None,
        tail: None,
    };
    let right = LinkedPoint {
        index: 0,
        x_current: h.x_current,
        y_current: h.y_current + 1,
        x_old: None,
        y_old: None,
        tail: None,
    };
    let bottom_right = LinkedPoint {
        index: 0,
        x_current: h.x_current + 1,
        y_current: h.y_current + 1,
        x_old: None,
        y_old: None,
        tail: None,
    };
    let bottom = LinkedPoint {
        index: 0,
        x_current: h.x_current + 1,
        y_current: h.y_current,
        x_old: None,
        y_old: None,
        tail: None,
    };
    let bottom_left = LinkedPoint {
        index: 0,
        x_current: h.x_current + 1,
        y_current: h.y_current - 1,
        x_old: None,
        y_old: None,
        tail: None,
    };
    let left = LinkedPoint {
        index: 0,
        x_current: h.x_current,
        y_current: h.y_current - 1,
        x_old: None,
        y_old: None,
        tail: None,
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
        if point.x_current == t.x_current && point.y_current == t.y_current {
            println!(
                " POINT {} ({}) is around POINT {} ({})",
                t.index,
                t.to_string(),
                h.index,
                h.to_string(),
            );
            true
        } else {
            false
        }
    })
}
