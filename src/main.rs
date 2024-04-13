mod adapter;
mod libs;

use crate::{adapter::sdict::SDict, libs::interface::Dict};

fn main() {
    let mut dict : SDict<String>  = SDict::<String>::new();
    let key = String::from("Hello");
    let value = String::from("World");

    let do_set = dict.set(key.clone(), value.clone());
    println!("1.SET : {:?}",do_set);
    dict.display();

    let key1 = String::from("HowABOUTYou");
    let value1 = String::from("How are you");

    let do_set = dict.set(key1.clone(), value1);
    println!("2.SET : {:?}",do_set);
    dict.display();
    let do_get = dict.get(key.clone());
    println!("1.GET : {:?}",do_get);

    let do_get = dict.get(key1.clone());
    println!("2.GET : {:?}",do_get);

    let do_get = dict.get(key.clone());
    println!("3.GET : {:?}",do_get);

    let size = dict.length();
    println!("SIZE : {}",size);

    //let do_del = dict.delete(key.clone());
    //println!("DEL : {:?}",do_del);

    let do_get = dict.get(key.clone());
    println!("4.GET : {:?}",do_get);

    //let do_del = dict.delete(key1.clone());
    //println!("DEL : {:?}",do_del);

    let do_get = dict.get(key.clone());
    println!("5.GET : {:?}",do_get);

    let size = dict.length();
    println!("SIZE : {}",size);

    //let do_clear = dict.clear();
    //println!("CLEAR : {:?}",do_clear);

    let do_update = dict.update(key.clone(), "Hello World 555".to_string());
    println!("UPDATE : {:?}",do_update);

    let do_get = dict.get(key.clone());
    println!("6.GET : {:?}",do_get);

    let size = dict.length();
    println!("SIZE : {}",size);

    println!("Hello, world!");
}
