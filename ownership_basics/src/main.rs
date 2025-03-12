#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn new(length: u32, width: u32) -> Self {
        Rectangle { length, width }
    }

    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn change(&mut self, length: u32, width: u32) {
        self.length = length;
        self.width = width;
    }

    fn fit(&self, rect: &Rectangle) -> bool {
        self.length> rect.length && self.width > rect.width
    }
}

fn main() {
    // new object
    let mut rect1 = Rectangle::new(10, 10);
    
    // change the attributes
    rect1.change(4, 5);

    println!("area of rect1: {:#?}", rect1.area());
    
    // check if the given rectangle fits or not
    println!("The given rectangle fits inside: {}",rect1.fit(&Rectangle::new(10, 10)));

    let s1 = String::from("hel lo");

    let word = word(&s1);

    println!("the first word is: {word}");
}

fn word(str: &String) -> &str {
    let s = str.as_bytes();
    let mut iter = s.iter().huhu();
    while let Some((i, &item)) = iter.next() {
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

impl<I> Enumerate<I> {
    fn new(iter: I) -> Enumerate<I> {
        Enumerate { iter, count: 0 }
    }
}

impl<T: ?Sized> MyIterator for T where T: Iterator {}

trait MyIterator: Iterator {
    fn huhu(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate {
            iter: self,
            count: 0,
        }
    }
}

impl<I> Iterator for Enumerate<I>
where
    I: Iterator,
{
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        let i = self.count;
        self.count += 1;
        Some((i, a))
    }
}
