#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn show_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn test_show_in_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 12,
            style: String::from("sandal"),
        },
    ];
    let in_size = show_in_size(shoes, 10);
    assert_eq!(in_size.len(), 1);
}

#[test]
fn test_counter() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    // let v_iter_mut = v.iter_mut();
    // let v_iter_own = v.into_iter();
    // for val in v_iter_own {
    //     print!("{} ", val);
    // }
    let v_iter = v.iter();
    let v_copy = v_iter.clone();
    let total: u32 = v_iter.sum();
    for value in v_copy {
        print!("{} ", value);
    }

    let v2 = vec![1, 2, 3, 4, 5];
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();

    for value in v3 {
        print!("{} ", value);
    }
}
