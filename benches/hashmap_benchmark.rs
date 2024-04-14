use std::collections::HashMap;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn bench_set_hashmap(c : &mut Criterion) {
    let key = String::from("HELLO");
    let value = String::from("How are you today.");
    let mut dict : HashMap<String, String> = HashMap::new();

    let element = black_box(
        (key,value)
    );

    fn set(dict : &mut HashMap<String,String>,key: &String ,value: &String) {
        let result = dict.insert(key.to_string(), value.to_string());
    }

    c.bench_function(
        "Set value to HashMap",
        |b| b.iter(|| set(&mut dict,&element.0, &element.1))
    );
}

fn bench_get_hashmap(c : &mut Criterion) {
    let key = String::from("HELLO");
    let value = String::from("How are you today.");
    let mut dict : HashMap<String, String> = HashMap::new();
    let result = dict.insert(key.clone(), value.clone());

    let element = black_box(
        key
    );

    fn get(dict : &mut HashMap<String,String>,key: &String) {
        let result = dict.get(key);
    }

    c.bench_function(
        "Get value from HashMap by key",
        |b| b.iter(|| get(&mut dict,&element))
    );
}

fn bench_update_hashmap(c : &mut Criterion) {
    let key = String::from("HELLO");
    let value = String::from("How are you today.");
    let new_value = String::from("I'm fine thank.");

    let mut dict : HashMap<String, String> = HashMap::new();
    let result = dict.insert(key.clone(), value.clone());

    let element = black_box(
        (key,value)
    );

    fn update(dict : &mut HashMap<String,String>,key: &String,new_value : &String) {
        if let Some(x) = dict.get_mut(key) {
            *x = new_value.to_string();
        }
    }

    c.bench_function(
        "Update value to HashMap by key",
        |b| b.iter(|| update(&mut dict,&element.0,&element.1))
    );
}

criterion_group!(
    benches,
    bench_set_hashmap,
    bench_get_hashmap,
    bench_update_hashmap
);
criterion_main!(benches);