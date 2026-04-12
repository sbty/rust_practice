fn greet(name: &str) {
    println!("こんにちは, {}さん", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn study_message(minutes: i32) -> &'static str {
    if minutes >= 60 {
        "よく頑張った"
    } else if minutes >= 30 {
        "いい感じ"
    } else {
        "少しずつ進めよう"
    }
}

fn square(n: i32) -> i32 {
    n * n
}

#[test]
fn test_square() {
    assert_eq!(square(4), 16);
}


fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[test]
fn test_is_even() {
    assert_eq!(is_even(6), true);
}

fn grade(score: i32) -> &'static str {
    if score >= 90 {
        "A"
    } else if score >= 60 {
        "B"
    } else {
        "C"
    }
}

#[test]
fn test_grade() {
    assert_eq!(grade(85), "B");
}


fn main() {
    let name = "shiba";
    let result = add(10, 20);
    let message = study_message(45);

    greet(name);
    println!("合計: {}", result);
    println!("メッセージ: {}", message);

    let len: i32 = 10;
    let num: i32 = 101;
    let my_score: i32 = 90;

    println!("square = {}", square(len));
    println!("is_even ? {}", is_even(num));
    println!("grade = {}", grade(my_score));
   
}