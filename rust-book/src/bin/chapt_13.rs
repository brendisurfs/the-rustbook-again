use std::thread;

#[derive(Debug)]
struct Rect {
    pub width: u32,
    pub height: u32,
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_sizes(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_filter_shoe_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoe_in_sizes(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        )
    }
}

// iteraters and Closures
fn main() {
    // listing 13-5
    let mut list = vec![1, 2, 3];
    println!("{:?}", list);

    thread::spawn(move || {
        list.push(77);
        println!("from thread: {list:?}");
    })
    .join()
    .unwrap();

    let mut r_list = [
        Rect {
            width: 10,
            height: 1,
        },
        Rect {
            width: 3,
            height: 7,
        },
        Rect {
            width: 4,
            height: 12,
        },
    ];

    r_list.sort_by_key(|r| r.width);

    println!("{:#?}", r_list);

    let vec_one = vec![1, 2, 3, 4, 5];

    let vec_two = vec_one.iter().map(|x| x + 1).collect::<Vec<_>>();

    assert_eq!(vec_two, vec![2, 3, 4, 5, 6]);
}

pub enum Vibe<T> {
    Rad(T),
    Bummer,
}

impl<T> Vibe<T> {
    // like unwrap_or_else, but its lit.
    pub fn unwrap_or_nah<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Vibe::Rad(x) => x,
            Vibe::Bummer => f(),
        }
    }
    // like unwrap, but way sicker.
    pub fn vibe_check(self) -> T {
        match self {
            Vibe::Rad(val) => val,
            Vibe::Bummer => panic!("not a vibe"),
        }
    }
}

