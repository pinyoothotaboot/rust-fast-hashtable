
use std::clone;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

#[derive(Clone,PartialEq,Debug)]
pub struct Node<T> {
    val : Option<T>,
    next : Option<Box<Node<T>>>
}

impl <T> Node<T>
where T : Clone + std::fmt::Debug
{
    pub fn new(val : Option<T> , next : Option<Box<Node<T>>>) -> Self {
        Node { val, next }
    }

    pub fn get_val(&self) -> Option<T> {
        self.val.clone()
    }
}

fn array_get_bench_matched(c : &mut Criterion) {
    let entries  = vec![Some(Box::new(Node::<String>::new(Some("Hello World".to_string()), None)));1024];
    let index = entries.len() / 2 +1;

    let element = black_box(
        (index,entries)
    );

    fn get(index : &usize,entries : &Vec<Option<Box<Node<String>>>>) -> Option<String> {
        match entries.get(*index) {
            Some(node) =>  {
                None
            },
            None => {
                None
            }
        }
    }

    c.bench_function(
        "Get tha value from node [match]",
        |b| b.iter(|| get(&element.0,&element.1))
    );
}

fn array_get_pointer_level_1_bench_matched(c : &mut Criterion) {
    let entries  = vec![Some(Box::new(Node::<String>::new(Some("Hello World".to_string()), None)));1024];
    let index = entries.len() / 2 + 1;

    let element = black_box(
        (index,entries)
    );

    fn get(index : &usize,entries : &Vec<Option<Box<Node<String>>>>) -> Option<String> {

        match &entries[*index] {
            Some(node) => {
                None
            },
            None => None
        }
    }

    c.bench_function(
        "Get tha value from enties [no match lv 1]",
        |b| b.iter(|| get(&element.0,&element.1))
    );
}

fn constant_array_get_bench_matched(c : &mut Criterion) {
    let entries : &[Option<Box<Node<String>>>;10] = &[
        Some(Box::new(Node::<String>::new(Some("Hello World".to_string()), None))),
        Some(Box::new(Node::<String>::new(Some("Hello World".to_string()), None))),
        Some(Box::new(Node::<String>::new(Some("Hello World".to_string()), None))),
        Some(Box::new(Node::<String>::new(Some("Hello World".to_string()), None))),
        Some(Box::new(Node::<String>::new(Some("Hello World".to_string()), None))),
        Some(Box::new(Node::<String>::new(Some("Hello World".to_string()), None))),
        Some(Box::new(Node::<String>::new(Some("Hello World".to_string()), None))),
        Some(Box::new(Node::<String>::new(Some("Hello World".to_string()), None))),
        Some(Box::new(Node::<String>::new(Some("Hello World".to_string()), None))),
        Some(Box::new(Node::<String>::new(Some("Hello World".to_string()), None))),
    ];

    let index = entries.len() / 2 + 1;

    let element = black_box(
        (index,entries)
    );

    fn get(index : &usize,entries : &[Option<Box<Node<String>>>]) -> Option<String> {
        
        match &entries.get(*index) {
            Some(node) => {
                None
            },
            None => None
        }
    }

    c.bench_function(
        "Get tha value from enties [constant array]",
        |b| b.iter(|| get(&element.0,element.1))
    );
}


fn vector_get_bench_matched(c : &mut Criterion) { 
    let mut entries : Vec<Option<Box<Node<String>>>> = Vec::new();
    let arr = vec![Some(Box::new(Node::<String>::new(Some("Hello World".to_string()), None)));1024];
    entries.extend(arr);

    let index = entries.len() / 2 + 1;

    let element = black_box(
        (index,entries)
    );

    fn get(index : &usize,entries : &Vec<Option<Box<Node<String>>>>) -> Option<String> {
        
        match entries.get(*index) {
            Some(node) => {
                None
            },
            None => None
        }
    }

    c.bench_function(
        "Get tha value from enties Vec",
        |b| b.iter(|| get(&element.0,&element.1))
    );
}

criterion_group!(
    benches,
    array_get_bench_matched,
    array_get_pointer_level_1_bench_matched,
    constant_array_get_bench_matched,
    vector_get_bench_matched
);
criterion_main!(benches);