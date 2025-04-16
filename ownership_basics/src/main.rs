#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn new(length: u32, width: u32) -> Self {
        Self { length, width }
    }

    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn change(&mut self, length: u32, width: u32) {
        self.length = length;
        self.width = width;
    }

    fn fit(&self, rect: &Rectangle) -> bool {
        self.length > rect.length && self.width > rect.width
    }
}

fn main() {
    // new object
    let mut rect1 = Rectangle::new(10, 10);

    // change the attributes
    rect1.change(4, 5);

    println!("area of rect1: {:#?}", rect1.area());

    // check if the given rectangle fits or not
    println!(
        "Can hold given rectangle?: {}",
        rect1.fit(&Rectangle::new(10, 10))
    );

    let s1 = String::from("hel lo");

    let word = word(&s1);

    println!("the first word is: {word}");
}

fn word(str: &String) -> &str {
    let s = str.as_bytes();
    let iter = s.iter().index_with();
    for (i, &item) in iter {
        println!("{i}  {item}");
        if item == b' ' {
            return &str[0..i];
        }
    }
    &str[..]
}

struct Enumerate<I> {
    iter: I,
    count: usize,
}

impl<I> Iterator for Enumerate<I>
where
    I: Iterator,
{
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        let count = self.count;
        self.count += 1;
        Some((count, item))
    }
}

trait MyIterator: Iterator {
    fn index_with(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate {
            iter: self,
            count: 0,
        }
    }
}

impl<I: Iterator> MyIterator for I {}
