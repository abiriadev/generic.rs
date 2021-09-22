// use std::ops::GeneratorState;

mod generic_enum;
mod generic_struct;

use crate::generic_struct::DPoint;
use generic_enum::GenericEnum;
use std::fmt::Display;
use std::iter::Sum;
// use std::iter::Sum;

mod pair;

fn main() {
    let largest = get_largest(&vec![
        43, 321, 21, 4, 34_3244, 423, 4322, 345352523, 432, 423, 21,
    ]);

    println!("{}", largest);

    generic_struct();

    generic_enum();

    trait_lab();

    pair::main();
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!(
            "{}, aut: {}",
            String::from("(read more)"),
            self.summarize_author()
        )
    }
    fn get_length(&self) -> usize;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
    }
    // fn summary(&self) -> String {
    //     format!("{}, by {}, ({})", self.headline, self.author, self.location)
    // }

    fn get_length(&self) -> usize {
        self.content.len()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn get_length(&self) -> usize {
        self.content.len()
    }
}

pub fn notify<T: Summary + Display>(item: T, item2: T) {
    println!("NEWS: {}", item.summarize());
}

fn trait_lab() {
    // todo!()

    let news = NewsArticle {
        headline: "title".to_string(),
        location: "location".to_string(),
        author: "Abiria".to_string(),
        content: "Lanistas sunt animaliss de audax mortem.".to_string(),
    };

    let summery = news.summarize();

    println!("{}", summery);
}

fn generic_enum() {
    let gen_enum = GenericEnum::<i32>::asdf;

    println!("{:#?}", gen_enum);

    let gen_enum2: GenericEnum<String> = GenericEnum::<String>::stru(String::from("hello world"));

    println!("{:#?}", gen_enum2);
}

fn generic_struct() {
    let integer: generic_struct::Point<i32> = generic_struct::Point { x: 5, y: 39999 };

    let float: generic_struct::Point<f64> = generic_struct::Point {
        x: 232.32,
        y: 1221.12,
    };

    let tu: generic_struct::DPoint<String, f64> = generic_struct::DPoint {
        x: String::from("sdkljasdk"),
        y: 3443.3443,
    };

    println!("{}", integer.x());

    // integer.dis
    let f64point: generic_struct::Point<f64> = generic_struct::Point {
        x: 32.32f64,
        y: 312331.2f64,
    };

    // println!("{}", f64point.distance_from_origin());
    println!("{}", f64point.distance_from_origin());

    let dp1 = DPoint { x: 5, y: 10.423 };

    // let dp3 = tu.mixup(&dp1);
    //
    // println!("{:#?}", dp3);

    // let dp4 =  dp3.mixup(&dp1);

    // println!("{:?}", dp4);
}

fn get_largest<T: PartialOrd + Copy>(input: &[T]) -> T {
    let mut largest: T = input[0];

    for &item in input {
        if item > largest {
            largest = item
        }
    }

    largest
}
