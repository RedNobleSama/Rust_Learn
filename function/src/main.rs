fn main() {
    println!("Hello, world!");
    another_function(5);
    let x = five();
    println!("this is value of {}", x);
    println!(x);
    print!();
    format!();

}

fn another_function(x: u32) {
    println!("Another function {}", x);
}

fn five() -> u32 {
    return 5;
}