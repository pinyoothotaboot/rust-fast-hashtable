use dict::adapter::idict;
use idict::IDict;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn bench_set_idict(c : &mut Criterion) {
    let value = String::from("How are you today.");
    let mut dict : IDict<String> = IDict::<String>::new();
    let key = dict.length() / 2;

    let element = black_box(
        (key,value)
    );

    fn set(dict : &mut IDict<String>,key: &usize ,value: &String) {
        let result = dict.set(key, value.to_string());
    }

    c.bench_function(
        "Set value to IDict",
        |b| b.iter(|| set(&mut dict,&element.0, &element.1))
    );
}

fn bench_get_idict(c : &mut Criterion) {
    let value = String::from("How are you today.");
    let mut dict : IDict<String> = IDict::<String>::new();
    let key = dict.length() / 2;
    let result = dict.set(&key, value.clone());
    
    let element = black_box(
        key
    );

    fn get(dict : &mut IDict<String>,key: &usize) {
        let result = dict.get(key);
    }

    c.bench_function(
        "Get value from IDict by key",
        |b| b.iter(|| get(&mut dict,&element))
    );
}

fn bench_update_idict(c : &mut Criterion) {
    let value = String::from("How are you today.");
    let new_value = String::from("I'm fine thank.");

    let mut dict : IDict<String> = IDict::<String>::new();
    let key = dict.length() / 2;
    let result = dict.set(&key, value.clone());

    let element = black_box(
        (key,value)
    );

    fn update(dict : &mut IDict<String>,key: &usize,new_value : &String) {
        let result = dict.update(key,new_value.to_string());
    }

    c.bench_function(
        "Update value to IDict by key",
        |b| b.iter(|| update(&mut dict,&element.0,&element.1))
    );
}

criterion_group!(
    benches,
    bench_set_idict,
    bench_get_idict,
    bench_update_idict
);
criterion_main!(benches);