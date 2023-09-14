mod ownership;

fn main() {
    let fahrenheit: f64 = 72.0;
    let celsius: f64 = to_celsius(fahrenheit);
    println!("{}°F = {}°C", fahrenheit, celsius);
    if celsius > 20.0 {
        println!("That's a nice temperature!");
    }

    let celsius: f64 = 0.0;
    let fahrenheit: f64 = to_fahrenheit(celsius);
    println!("{}°C = {}°F", celsius, fahrenheit);
    if (fahrenheit - 32.0).abs() < 0.001 {
        println!("That's getting pretty cold!");
    }

    println!("f(3) = f(2) + f(1) = f(2) + 1 = f(1) + f(0) + 1 = 1 + 0 + 1 = 2");
    let n: u32 = 3;
    let f: u32 = fibonacci(n);
    println!("fibonacci({}) = {}", n, f);

    println!("Let's sing a song!");
    twelve_days_of_christmas();

    println!("Let's count down!");
    try_loop();

    ownership::play_with_box();
    ownership::ownership_examples();
}

fn to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

// generate the nth Fibonacci
fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn twelve_days_of_christmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas my true love sent to me", days[day]);
        for gift in (0..day + 1).rev() {
            println!("{}", gifts[gift]);
        }
        println!();
    }
}

fn try_loop() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
