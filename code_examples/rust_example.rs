// Importing standard library modules
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::time::{Duration, Instant};

// Defining an enum to represent a direction
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Defining a struct to represent a point in 2D space
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Associated function (constructor)
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // Method to move the point in a given direction
    fn move_point(&mut self, direction: Direction, distance: i32) {
        match direction {
            Direction::Up => self.y += distance,
            Direction::Down => self.y -= distance,
            Direction::Left => self.x -= distance,
            Direction::Right => self.x += distance,
        }
    }

    // Method to display the point's coordinates
    fn display(&self) {
        println!("Point({}, {})", self.x, self.y);
    }
}

// Function to demonstrate error handling with Result
fn read_file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// Function to demonstrate the use of generics
fn get_largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }

    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}

// Function to demonstrate a basic HashMap usage
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let counter = word_count.entry(word.to_string()).or_insert(0);
        *counter += 1;
    }
    word_count
}

// Function to demonstrate a simple pattern matching
fn match_number(num: i32) -> &'static str {
    match num {
        1 => "One",
        2 | 3 | 5 | 7 => "Prime",
        4 | 6 | 8 | 9 | 10 => "Composite",
        _ => "Unknown",
    }
}

// Function to demonstrate a closure and iterator
fn sum_of_squares(numbers: &[i32]) -> i32 {
    numbers.iter().map(|&x| x * x).sum()
}

// Function to demonstrate a loop and ownership
fn reverse_string(s: String) -> String {
    let mut reversed = String::new();
    for c in s.chars().rev() {
        reversed.push(c);
    }
    reversed
}

fn main() {
    // Creating and manipulating a Point
    let mut p = Point::new(0, 0);
    p.move_point(Direction::Right, 10);
    p.move_point(Direction::Up, 5);
    p.display();

    // Handling potential errors
    match read_file_to_string("example.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }

    // Using generics to find the largest element
    let numbers = vec![34, 50, 25, 100, 65];
    if let Some(largest) = get_largest(&numbers) {
        println!("The largest number is {}", largest);
    }

    // Counting word occurrences in a string
    let text = "hello world wonderful world";
    let word_count = count_words(text);
    for (word, count) in word_count {
        println!("{}: {}", word, count);
    }

    // Pattern matching with numbers
    let num = 7;
    println!("The number {} is {}", num, match_number(num));

    // Calculating the sum of squares using an iterator
    let squares_sum = sum_of_squares(&numbers);
    println!("Sum of squares: {}", squares_sum);

    // Reversing a string
    let original = String::from("Hello, Rust!");
    let reversed = reverse_string(original.clone());
    println!("Original: {}, Reversed: {}", original, reversed);

    // Measuring time taken for an operation
    let start = Instant::now();
    let duration = Duration::from_millis(2000);
    std::thread::sleep(duration);
    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
}
