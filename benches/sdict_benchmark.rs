use dict::adapter::sdict;
use sdict::SDict;
use dict::libs::interface::Dict;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn bench_set_sdict(c : &mut Criterion) {
    let key = String::from("HELLO");
    let value = String::from("How are you today.");
    let mut dict : SDict<String> = SDict::<String>::new();

    let element = black_box(
        (key,value)
    );

    fn set(dict : &mut SDict<String>,key: String ,value: String) {
        let result = dict.set(key, value);
    }

    c.bench_function(
        "Set value to sdict",
        |b| b.iter(|| set(&mut dict,element.0.clone(), element.1.clone()))
    );
}

fn bench_get_sdict(c : &mut Criterion) {
    let key = String::from("HELLO");
    let value = String::from("How are you today.");
    let mut dict : SDict<String> = SDict::<String>::new();
    let result = dict.set(key.clone(), value.clone());

    let element = black_box(
        key
    );

    fn get(dict : &mut SDict<String>,key: String) {
        let result = dict.get(key);
    }

    c.bench_function(
        "Get value from sdict by key",
        |b| b.iter(|| get(&mut dict,element.clone()))
    );
}

fn bench_update_sdict(c : &mut Criterion) {
    let key = String::from("HELLO");
    let value = String::from("How are you today.");
    let new_value = String::from("I'm fine thank.");

    let mut dict : SDict<String> = SDict::<String>::new();
    let result = dict.set(key.clone(), value.clone());

    let element = black_box(
        (key,value)
    );

    fn update(dict : &mut SDict<String>,key: String,new_value : String) {
        let result = dict.update(key,new_value);
    }

    c.bench_function(
        "Update value to sdict by key",
        |b| b.iter(|| update(&mut dict,element.0.clone(),element.1.clone()))
    );
}

criterion_group!(
    benches,
    bench_set_sdict,
    bench_get_sdict,
    bench_update_sdict
);
criterion_main!(benches);