use std::borrow::Cow;

pub struct FizzBuzzer  {
    labels: [String; 3],
}

impl FizzBuzzer {
    pub fn new(labels: [String; 3]) -> Self {
        FizzBuzzer { labels }
    }

    pub fn iter(&self) -> FizzBuzzerIter {
        let iter = FizzBuzzerIter
        {
            fizzbuzzer: &self,
            index: 0,
        };
        
        iter
    }
}

pub struct FizzBuzzerIter <'a> {
    fizzbuzzer: &'a FizzBuzzer,
    index: u32,
}

impl <'a> Iterator for FizzBuzzerIter <'a> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        let fizz : bool = self.index % 3 == 0;
        let buzz : bool = self.index % 5 == 0;

        if fizz & buzz
        {
            return Some(Cow::from(&self.fizzbuzzer.labels[2]))
        }
        else if fizz
        {
            return Some(Cow::from(&self.fizzbuzzer.labels[0]))
        }
        else if buzz
        {
            return Some(Cow::from(&self.fizzbuzzer.labels[1]))
        }
        else
        {
            return Some(Cow::from(self.index.to_string()))
        }
    }
}

fn main() {

    let labels = [String::from("hello"), String::from("i"), String::from("gay")];
    let fb = FizzBuzzer::new(labels);
    let mut iter = FizzBuzzer::iter(&fb);
    for _i in 1..20
    {
        println!("{:?}", iter.next());
    }
    iter.next();
    iter.next();
    iter.next();
}

macro_rules! assert_match {
    ($expr:expr, $pat:pat) => {
        if let $pat = $expr {
            // all good
        } else {
            assert!(false, "Expression {:?} does not match the pattern {:?}", $expr, stringify!($pat));
        }
    }
}

#[test]
fn test_basic() {
    let fizzbuzzer = FizzBuzzer::new([
        String::from("Fizz"),
        String::from("Buzz"),
        String::from("Fizzbuzz"),
    ]);
    let items: Vec<_> = fizzbuzzer.iter().take(15).collect();

    assert_eq!(items, [
        "1", "2", "Fizz", "4", "Buzz",
        "Fizz", "7", "8", "Fizz", "Buzz",
        "11", "Fizz", "13", "14", "Fizzbuzz"
    ]);
}

#[test]
fn test_cow() {
    let fizzbuzzer = FizzBuzzer::new([
        String::from("Fizz"),
        String::from("Buzz"),
        String::from("Fizzbuzz"),
    ]);
    let mut iter = fizzbuzzer.iter();

    assert_match!(iter.next(), Some(Cow::Owned(_)));    // "1"
    assert_match!(iter.next(), Some(Cow::Owned(_)));    // "2"
    assert_match!(iter.next(), Some(Cow::Borrowed(_))); // "Fizz"
}

#[test]
fn test_labels() {
    let fizzbuzzer = FizzBuzzer::new([
        String::from("Four"),
        String::from("Seasons"),
        String::from("Total Landscaping"),
    ]);
    let items: Vec<_> = fizzbuzzer.iter().take(15).collect();

    assert_eq!(items, [
        "1", "2", "Four", "4", "Seasons",
        "Four", "7", "8", "Four", "Seasons",
        "11", "Four", "13", "14", "Total Landscaping"
    ]);
}
