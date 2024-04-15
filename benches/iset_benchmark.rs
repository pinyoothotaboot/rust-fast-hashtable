use dict::adapter::iset;
use iset::ISet;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn bench_add_iset(c : &mut Criterion) {
    let mut set : ISet<String> = ISet::<String>::new();
    let key = set.length() / 2;
    let element = black_box(
        key
    );

    fn add(set : &mut ISet<String>,key: &usize) {
        let result = set.add(key);
    }

    c.bench_function(
        "Add the key to ISet",
        |b| b.iter(|| add(&mut set,&element))
    );
}

fn bench_get_iset(c : &mut Criterion) {
    let mut set : ISet<String> = ISet::<String>::new();
    let key = set.length() / 2;
    let result = set.add(&key);

    let element = black_box(
        key
    );

    fn get(set : &mut ISet<String>,key: &usize) {
        let result = set.get(key);
    }

    c.bench_function(
        "Get key from ISet",
        |b| b.iter(|| get(&mut set,&element))
    );
}

fn bench_has_iset(c : &mut Criterion) {
    let value = String::from("How are you today.");
    let mut set : ISet<String> = ISet::<String>::new();
    let key = set.length() / 2;
    let result = set.add(&key);

    let element = black_box(
        key
    );

    fn has(set : &mut ISet<String>,key: &usize) {
        let result = set.has(key);
    }

    c.bench_function(
        "Check key from ISet",
        |b| b.iter(|| has(&mut set,&element))
    );
}

fn bench_update_iset(c : &mut Criterion) {
    let mut set : ISet<String> = ISet::<String>::new();
    let key = set.length() / 2;
    let new_key = key + 10;
    let result = set.add(&key);

    let element = black_box(
        (key,new_key)
    );

    fn update(set : &mut ISet<String>,key: &usize,new_key : &usize) {
        let result = set.update(key,new_key);
    }

    c.bench_function(
        "Update new key to ISet by key",
        |b| b.iter(|| update(&mut set,&element.0,&element.1))
    );
}

criterion_group!(
    benches,
    bench_add_iset,
    bench_get_iset,
    bench_has_iset,
    bench_update_iset
);
criterion_main!(benches);