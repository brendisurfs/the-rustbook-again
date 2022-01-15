fn main() {
    let string_one = String::from("Hello mate");
    let len = calc_len(&string_one);
    println!("{}", len);

    // now with references
    let mut str_two = String::from("sick");
    // we have to pass in &mut to allow for that value to be modified inside the function.
    // references canot be modified.
    // this here is what we call a `mutable reference`
    // a good way to think:
    // this just makes it very clear that the function will mutate the value it borrows.
    world(&mut str_two);

    // this does not work.
    // you can only have one mutable reference to a piece of data at a time.
    // let r2 = &mut s;
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn world(some_str: &mut String) {
    some_str.push_str(", world");
}
