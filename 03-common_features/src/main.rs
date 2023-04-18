const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y: i32 = 5;

    let y: i32 = y + 1;

    {
        let y: i32 = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
    println!("The value of a constant is: {THREE_HOURS_IN_SECONDS}");

}