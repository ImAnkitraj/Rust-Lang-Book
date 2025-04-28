use std::collections::HashMap;

struct Cacher<T, K>
where
    T: Fn(K) -> K,
    K: std::hash::Hash + Eq + Copy,
{
    calculation: T,
    value: HashMap<K, K>,
}

impl<T, K> Cacher<T, K>
where
    T: Fn(K) -> K,
    K: std::hash::Hash + Eq + Copy,
{
    fn new(calculation: T) -> Cacher<T, K> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> K {
        if self.value.contains_key(&arg) {
            return self.value[&arg];
        } else {
            let v = (self.calculation)(arg);
            self.value.insert(arg, v);
            v
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cache_result = Cacher::new(|num: u32| {
        println!("calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cache_result.value(intensity));
        println!("Next, do {} situps!", cache_result.value(intensity));
    } else if intensity < 50 {
        if random_number == 3 {
            println!("Take a break today!");
        } else {
            println!("Today, run for {} minutes!", cache_result.value(intensity));
        }
    } else {
        println!("Today, do yoga!");
    }
}

fn main() {
    println!("Hello, world!");
    generate_workout(20, 7);
}
