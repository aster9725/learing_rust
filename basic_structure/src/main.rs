use std::io;

fn main() {
    let mut str = String::new();

    println!("Basic command lists");
    println!("\t1. Create new project");
    println!("\t2. Build");
    println!("\t3. Debug");
    println!("Select > ");
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");

    println!("You typed {}", str);
}
