mod adapter;
mod libs;

use crate::{adapter::sdict::SDict, libs::interface::Dict};

fn main() {
    let mut dict : SDict<String>  = SDict::<String>::new();
    let key = String::from("Hello");
    let value = String::from("World");

    let do_set = dict.set(key.clone(), value.clone());
    println!("SET : {:?}",do_set);

    let key1 = String::from("Hi");
    let value1 = String::from("How are you");

    let do_set = dict.set(key1.clone(), value1);
    println!("SET : {:?}",do_set);

    let do_get = dict.get(key.clone());
    println!("GET : {:?}",do_get);

    let do_get = dict.get(key1.clone());
    println!("GET : {:?}",do_get);

    let do_get = dict.get(key.clone());
    println!("GET : {:?}",do_get);

    let size = dict.length();
    println!("SIZE : {}",size);

    let do_del = dict.delete(key.clone());
    println!("DEL : {:?}",do_del);

    let do_get = dict.get(key.clone());
    println!("GET : {:?}",do_get);

    let do_del = dict.delete(key1.clone());
    println!("DEL : {:?}",do_del);

    let do_get = dict.get(key.clone());
    println!("GET : {:?}",do_get);

    let size = dict.length();
    println!("SIZE : {}",size);

    let do_clear = dict.clear();
    println!("CLEAR : {:?}",do_clear);

    let do_get = dict.get(key.clone());
    println!("GET : {:?}",do_get);

    let size = dict.length();
    println!("SIZE : {}",size);

    println!("Hello, world!");
}
