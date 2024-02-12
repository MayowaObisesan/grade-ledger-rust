use std::collections::HashMap;
use std::io;

fn main() {
    let mut student_grades: HashMap<String, f32> = HashMap::new();

    loop {
        println!("This is our primary group ledger");
        println!("Choose from the below menu:");
        println!("1. Add student grade");
        println!("2. Get student grade");
        println!("3. Update student grade");
        println!("4. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number");
                continue;
            }
        };

        match choice {
            1 => add_grade(&mut student_grades),
            2 => get_grade(&student_grades),
            3 => update_grade(&mut student_grades),
            4 => break,
            _ => println!("Invalid choice. Please try again"),
        }
    }
}

fn update_grade(student_grades: &mut HashMap<String, f32>) {
    // Check that at least one student exists
    if student_grades.len() == 0 {
        println!("No student exist. Create student first.");
    }
    // Get the student name
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Unable to read input");

    // Get the student grade
    let mut grade = String::new();
    io::stdin()
        .read_line(&mut grade)
        .expect("Unable to read input");
    let grade = match grade.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid grade.");
            return;
        }
    };
    if let Some(cur_grade) = student_grades.get_mut(&name) {
        *cur_grade = grade;
    }
}

fn get_grade(student_grades: &HashMap<String, f32>) {
    println!("Enter student name");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    let name = name.trim().to_string();

    match student_grades.get(&name) {
        Some(grade) => println!("Student {} has a grade of {}", name, grade),
        None => println!("No grade found for student {}", name),
    }
}

fn add_grade(student_grades: &mut HashMap<String, f32>) {
    println!("You can create a student here.");
    println!("Enter student name: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Unable to read your input");
    let name = name.trim().to_string();

    // Take Student input
    println!("Enter student grade");
    let mut grade = String::new();
    io::stdin()
        .read_line(&mut grade)
        .expect("Unable to read your input");

    // let grade = grade.trim().parse().unwrap_or_else(|_| {
    //     println!("Invalid input. Please enter a valid grade.");
    // });
    let grade = match grade.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid grade.");
            return;
        }
    };

    student_grades.insert(name, grade);
    println!("Grade added successfully.");
}
