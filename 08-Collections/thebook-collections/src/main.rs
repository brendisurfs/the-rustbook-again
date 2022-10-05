fn main() {
    let mut v: Vec<i32> = Vec::new(); // vectors can grow in size.

    v.push(2);

    let mut v_two = vec![1, 2, 3, 5]; // different way to initialize.

    for i in &mut v_two {
        // dereference
        *i += 50;
    }
    for j in v_two {
        println!("{j}")
    }

    // using match to find. get returns an option, can check.

    // match v_two.get(2) {
    //     Some(third) => println!("third element is {third}"),
    //     None => println!("no third element"),
    // }

    // Strings in rust

    /*
      strings are just utf8 encoded bytes.
    */

    let mut s1 = String::new();
    let s2 = "this is a string";
    s1.push_str("holy heck this is fun");

    let db_username = "brendo";
    let db_psswd = "password";
    let db_url = "dburl.com";

    // format doesnt take ownership of the strings.
    let db_config = format!("{db_username}:{db_psswd}@{db_url}");
    println!("{db_config:#?}");

    // string indexing.
    let bytes_value = s1.bytes();
}
