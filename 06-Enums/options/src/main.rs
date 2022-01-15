/*
  OPTIONS AND ENUMS
*/
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);
    println!("{sum}")
}
