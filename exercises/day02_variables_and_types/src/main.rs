fn main() {
    let age: i32 = 20;
    let height: f64 = 180.5;
    let is_learning: bool = true;
    let initial: char = 'S';

    let profile = ("shiba", age);
    let scores = [80, 90, 100];

    println!("age: {}", age);
    println!("height: {}", height);
    println!("learning: {}", is_learning);
    println!("initial: {}", initial);
    
    println!("name: {}", profile.0);
    println!("profile age: {}", profile.1);
    
    println!("first score: {}", scores[0]);

    for score in scores {
        println!("score: {}", score);
    }
    
}