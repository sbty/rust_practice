fn message_for_study_time(study_time: i32) -> &'static str {
    if study_time >= 60 {
        "よく頑張った"
    } else if study_time >= 30 {
        "いい感じ"
    } else {
         "少しずつ進めよう"
    }
}

fn main() {
    let name = "shiba";
    let mut study_time = 30;

    println!("名前: {}", name);
    println!("学習時間: {} 分", study_time);

    study_time = 59;
    println!("{} ", message_for_study_time(study_time));

    let mut message = String::from("hello");
    message.push_str(", shiba");

    let final_message = format!("{}! Rust を学ぼう", message);

    println!("{}", final_message);
   
}