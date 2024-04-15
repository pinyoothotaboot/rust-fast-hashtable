use dict::adapter::sset;
use sset::SSet;
use dict::libs::interface::Set;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn bench_add_sset(c : &mut Criterion) {
    let key = String::from("HELLO").as_bytes().to_vec();
    let mut set : SSet<String> = SSet::<String>::new();

    let element = black_box(
        key
    );

    fn add(set : &mut SSet<String>,key: &Vec<u8>) {
        let result = set.add(key);
    }

    c.bench_function(
        "Add the key to SSet",
        |b| b.iter(|| add(&mut set,&element))
    );
}

fn bench_get_sset(c : &mut Criterion) {
    let key = String::from("HELLO").as_bytes().to_vec();
    let mut set : SSet<String> = SSet::<String>::new();
    let result = set.add(&key);

    let element = black_box(
        key
    );

    fn get(set : &mut SSet<String>,key: &Vec<u8>) {
        let result = set.get(key);
    }

    c.bench_function(
        "Get key from SSet",
        |b| b.iter(|| get(&mut set,&element))
    );
}

fn bench_has_sset(c : &mut Criterion) {
    let key = String::from("HELLO").as_bytes().to_vec();
    let value = String::from("How are you today.");
    let mut set : SSet<String> = SSet::<String>::new();
    let result = set.add(&key);

    let element = black_box(
        key
    );

    fn has(set : &mut SSet<String>,key: &Vec<u8>) {
        let result = set.has(key);
    }

    c.bench_function(
        "Check key from SSet",
        |b| b.iter(|| has(&mut set,&element))
    );
}

fn bench_update_sset(c : &mut Criterion) {
    let key = String::from("HELLO").as_bytes().to_vec();
    let new_key = String::from("How are you").as_bytes().to_vec();

    let mut set : SSet<String> = SSet::<String>::new();
    let result = set.add(&key);

    let element = black_box(
        (key,new_key)
    );

    fn update(set : &mut SSet<String>,key: &Vec<u8>,new_key : &Vec<u8>) {
        let result = set.update(key,new_key);
    }

    c.bench_function(
        "Update new key to SSet by key",
        |b| b.iter(|| update(&mut set,&element.0,&element.1))
    );
}

criterion_group!(
    benches,
    bench_add_sset,
    bench_get_sset,
    bench_has_sset,
    bench_update_sset
);
criterion_main!(benches);