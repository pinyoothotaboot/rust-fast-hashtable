use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

use dict::libs::hash::{hash};

fn bench_hash_function(c : &mut Criterion) {
    let key = String::from("HELLOHOWARE").as_bytes().to_vec();
    let table_size = 255*255*255*255;
    let seed = 123456789;
    
    let element = black_box(
        (key,table_size,seed)
    );

    c.bench_function(
        "Get hash function",
        |b| b.iter(|| hash(&element.0, element.1, element.2))
    );
}

criterion_group!(
    benches,
    bench_hash_function,
);
criterion_main!(benches);