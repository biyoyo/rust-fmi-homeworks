fn main() {
    //println!("{:?}", fizzbuzz(15));
    //println!("{:?}", custom_buzz(8, 2, 3));

    let labels = [String::from("foo"), String::from("bar"), String::from("foobar")];

    let mut f = FizzBuzzer 
    {
        k1: 3,
        k2: 5,
        labels: labels, 
    };

    println!("{:?}", FizzBuzzer::take(&f, 10));

    let rf = &mut  f;

    FizzBuzzer::change_label(rf, 0, &String::from("hello"));
    FizzBuzzer::change_label(rf, 2, &String::from("hellobar"));

    println!("{:?}", FizzBuzzer::take(&f, 15));

}

pub fn fizzbuzz(n: usize) -> Vec<String> 
{
    let mut vec = Vec::<String>::new();

    for i in 1..n+1
    {
        let fizz : bool = i % 3 == 0; 
        let buzz : bool = i % 5 == 0; 

        if fizz && buzz
        {
            vec.push(String::from("Fizzbuzz"));
        }
        else if fizz 
        {
            vec.push(String::from("Fizz"));
        }
        else if buzz
        {
            vec.push(String::from("Buzz"));
        }
        else
        {
            vec.push(i.to_string());
        }
    }
    vec
}

pub fn custom_buzz(n: usize, k1: u8, k2: u8) -> Vec<String> 
{
    if k1 <= 1 || k2 <= 1 
    {
        panic!("Invalid divisors");
    }

    let mut vec = Vec::<String>::new();
    for i in 1..n+1
    {
        let fizz : bool = (i as u8) % k1 == 0; 
        let buzz : bool = (i as u8) % k2 == 0; 

        if fizz && buzz 
        {
            vec.push(String::from("Fizzbuzz"));
        } 
        else if fizz
        {
            vec.push(String::from("Fizz"));
        }
        else if buzz
        {
            vec.push(String::from("Buzz"));
        }
        else
        {
            vec.push(i.to_string());
        }
    }
    vec
}

pub struct FizzBuzzer 
{
    pub k1: u8,
    pub k2: u8,
    pub labels: [String; 3],
}

impl FizzBuzzer 
{
    pub fn take(&self, n: usize) -> Vec<String> 
    {
        let n1 = self.k1;
        let n2 = self.k2;

        let label0 = &self.labels[0];
        let label1 = &self.labels[1];
        let label2 = &self.labels[2]; 

        if n1 <= 1 || n2 <= 1 
        {
            panic!("Invalid divisors");
        }
        let mut vec = Vec::<String>::new();
        for i in 1..n+1
        {
            let fizz : bool = (i as u8) % n1 == 0; 
            let buzz : bool = (i as u8) % n2 == 0; 

            if fizz && buzz 
            {
                vec.push(label2.to_string());
            } 
            else if fizz
            {
                vec.push(label0.to_string());
            }
            else if buzz
            {
                vec.push(label1.to_string());
            }
            else
            {
                vec.push(i.to_string());
            }
        }
        vec
    }

    pub fn change_label(&mut self, index: usize, value: &String) 
    {
        if index > 2
        {
            panic!("Invalid index of label");
        }

        self.labels[index] = value.clone();
    }
}

#[test]
fn test_basic() {
    let expected = vec![1.to_string(), 2.to_string(), String::from("Fizz")];

    assert_eq!(fizzbuzz(3), expected);
    assert_eq!(custom_buzz(3, 3, 5), expected);

    let mut fizzbuzzer = FizzBuzzer {
        k1: 3,
        k2: 5,
        labels: [
            String::from("Fizz"),
            String::from("Buzz"),
            String::from("Fizzbuzz")
        ],
    };
    assert_eq!(fizzbuzzer.take(3), expected);
    fizzbuzzer.change_label(0, &String::from("Fiz"));
}

#[test]
fn m_tests()
{
    let expected = Vec::<String>::new();
    assert_eq!(fizzbuzz(0), expected);
    assert_eq!(custom_buzz(0, 3, 5), expected);
}