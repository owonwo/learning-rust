use crate::shapes::{circle::Circle, rect::Rect};
use crate::shapes::area::Area;
use crate::shapes::collision::{Collides, Contains};
use crate::shapes::shape::Shape;

mod practice;
mod shapes;

fn main() {
    let file = std::fs::read_to_string("./file.txt");

    match file {
        Ok(file) => {
            file.lines()
                .enumerate()
                .filter(|(i, _)| i % 2 == 0)
                .skip(2)
                .for_each(|(_, x)| {
                    println!("{}", x);
                });
        }
        Err(_) => {}
    };

    // colors::print_color(Colors::Blue);

    practice::practice(vec![4, 1, 0, 4], 6);
    practice::practice(vec![4, 1, 0, 4], 2);
    practice::log_lines();

    compute();
    let _ = print_shapes();
}

pub fn compute() {
    let mut rec = Rect::default();
    let circle = Circle {
        x: 0.0,
        y: 0.0,
        radius: 4.0,
    };

    rec.contains_point((rec.x, rec.y));
    rec.contains_point((circle.x, circle.y));

    rec.y = 3.0;
    rec.x = 24.0;
    rec.width = 24.0;
    rec.height = 12.0;

    for (x, y) in &rec {
        println!("Rectangle x->{} y->{}", x, y);
    }

    println!("Circle area {}", circle.area());
    println!("Rectangle {}", rec.to_string());
}

pub fn print_shapes() -> Result<(), std::io::Error> {
    let file = std::fs::read_to_string("shape.txt")?;

    let shapes = file.lines()
        .filter_map(|str| str.parse::<Shape>().ok())
        .collect::<Vec<_>>();

    let collisions: Vec<(&Shape, &Shape)> = shapes
        .iter()
        .skip(1)
        .zip(shapes.iter().take(shapes.len() - 1))
        .filter(|(a, b)| a.is_colliding(b))
        .collect();

    for (a, b) in collisions {
        println!("{} intersects {}", a, b);
    }


    return Ok(());
}

// fn multiply(num: Option<usize>) -> usize {
//     num.unwrap_or(0) * 5
// }
//
// struct Member {
//     name: String,
//     email: String,
//     phone: String,
// }
//
// struct RoundCommodity {
//     name: String,
//     sharing_date: String,
//     created_at: String,
//     participants: Vec<Member>,
// }
//
// struct ActiveCommodity {
//     name: String,
//     sharing_date: String,
//     created_at: String,
// }
//
// enum Commodity {
//     Pending(RoundCommodity),
//     Active(ActiveCommodity),
// }
//
// fn edit_commodity(commodity: Commodity) -> Vec<Member> {
//     let action = match commodity {
//         Commodity::Pending(RoundCommodity { participants, .. }) if participants.len() > 0 => {
//             Some(participants)
//         }
//         _ => None,
//     };
//
//     return action.unwrap().iter().collect();
// }
//
// fn main() {
//     // str variable is immutable
//     // let str = "Hello, world!";
//     let data = vec![1,2,3,4,5];
//     let mut a = data
//         .iter()
//         .map(|x| x + 1);
//
//     let mut collection = vec![];
//     while let Some(v) = a.next() {
//         collection.push(v);
//     }
//
//     println!("{:?}", collection);
//
//     // let b = a[2];
//     // let value = a.get(2);
//     //
//     // let tuple = (5, String::from("hello"));
//     // let (num, string) = tuple;
//
//     // println!("{}", str);
// }

// struct Person {
//     pub first_name: str,
//     pub last_name: str,
//     age: isize,
// }
