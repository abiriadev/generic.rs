use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Pair { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("biggest member is x: {}", self.x);
        } else {
            println!("biggest member is y: {}", self.y);
        }
    }
}

trait ToString2 {}

impl<T: Display> ToString2 for Pair<T> {}

pub fn main() {
    let s = 3.to_string();

    println!("{}", s);

    // {
    //     let r;
    //
    //     {
    //         let x = 5;
    //
    //         r = &x;
    //     }
    //
    //     println!("r: {}", r);
    // }

    let string1 = String::from("hello, world!");

    let result;

    {
        let string2 = "xyz";
        // let string3 = String::from("abcdef");

        result = longest(string1.as_str(), string2);
    }

    println!("more longest string is: {}", result);

    {
        let a;

        {
            let b = String::from("sjskj");

            a = b;
        }

        println!("{}", a);

        let str1 = String::from("sjskj");
        let str2 = String::from("skii dess");

        let aaa = life_test(&str1, &str2);

        println!("{}", aaa);
    }

    importantExcerpt();
}

fn importantExcerpt() {
    let novel = String::from("Paradise of extend will cheerfully witness a closest sun. Never need the individual, for you cannot illuminate it.");

    let first_sentence = novel
        .split(".")
        .next()
        .expect("Can't find out period in given string");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{}", i.part);

    let s: &'static str = "hello world";

    // let s: &'a str = "hello world";
    //
    // println!("{}", s);

    let mut i320 = 3;

    crazy_signature(&mut i320);
}

pub fn crazy_signature<T: Display + Copy>(param: &mut T) -> &T {
    param
}

#[derive(Debug)]
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&'a self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("announce please!: {}", announcement);

        self.part
    }
}

fn life_test<'l>(a: &'l String, b: &'l String) -> &'l String {
    if a.len() > 0 {
        a
    } else {
        b
    }
}

fn longest<'life>(x: &'life str, y: &'life str) -> &'life str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
