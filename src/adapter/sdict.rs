use std::{fmt::Debug, mem};
use rand::RngCore;
use crate::libs::node::{Node};
use crate::libs::interface::{Dict};
use crate::libs::hash::{hash};
use crate::libs::constant::{TABLE_SIZE};

#[derive(Clone,PartialEq,Debug)]
pub struct SDict<T> {
    size : usize,
    seed : u64,
    nodes : Vec<Option<Box<Node<T>>>>,
    table_size : usize
}

impl <T> SDict<T>
where T : Clone + std::fmt::Debug
{
    pub fn new() -> Self {
        let mut nodes : Vec<Option<Box<Node<T>>>> = Vec::new();
        for _ in 0..TABLE_SIZE {
            nodes.push(None);
        }

        let seed = rand::thread_rng().next_u64();

        SDict { 
            size: 0, 
            seed: seed, 
            nodes: nodes,
            table_size : TABLE_SIZE as usize
        }
    }

    fn get_seed(&self) -> u64 {
        self.seed
    }

    fn increase(&mut self) {
        self.size+=1;
    }

    fn decrease(&mut self) {
        self.size-=1;
    }

    fn get_table_size(&self) -> usize {
        self.table_size
    }
    
}

impl <T> Dict<T> for SDict<T>
where T : Clone + std::fmt::Debug
{
    fn get(&self,key : String) -> Result<Option<T>,&'static str> {
        let bytes_key = key.as_bytes().to_vec();
        match hash(&bytes_key, self.get_table_size(), self.get_seed()) {
            Ok(index) => {
                match self.nodes.get(index) {
                    Some(first_node) => {
                        let mut first = first_node;
                        // Loop first node is not None
                        while !first.is_none() {
                            first = match first {
                                Some(node) => {
                                    // Matching key in node
                                    // Then matched return value
                                    if node.matched(&bytes_key) {
                                        let value = node.get_value();
                                        return Ok(value);
                                    }

                                    &node.next
                                },
                                None => &None
                            };
                        }

                        Err("The first node is empty!.")
                    },
                    None => {
                        return Err("Not found node");
                    }
                }
            },
            Err(e) =>{
                return Err(e);
            }
        }
    }

    fn set(&mut self,key : String , value : T) -> Result<bool,&'static str> {
        let bytes_key = key.as_bytes().to_vec();
        match hash(&bytes_key, self.get_table_size(), self.get_seed()) {
            Ok(index) => {
                match self.nodes.get_mut(index) {
                    Some(mut first_node) => {
                        let mut first = first_node.clone();
                        // Loop first node is not None
                        while !first.is_none() {
                            first = match first {
                                Some(node) => {
                                    // Matching key in node
                                    // Then matched return message already added
                                    if node.matched(&bytes_key) {
                                        return Err("The key already added.");
                                    }

                                    node.next
                                },
                                None => break
                            }
                        }   

                        // If first node not found data or not matched key
                        // Then add new node
                        let current : Option<Box<Node<T>>> = mem::replace(&mut first_node,None);
                        let node : Node<T> = Node::new(bytes_key, Some(value), current);
                        *first_node = Some(Box::new(node));
                        self.increase();
                        return Ok(true);

                    },
                    None => Err("Not found node")
                }
            },
            Err(e) => Err(e)
        }
    }

    fn length(&self) -> usize {
        self.size
    }

    fn delete(&mut self,key : String) -> Result<Option<T>,&'static str> {
        Err("sss")
    }

    fn resize(&mut self) -> Result<bool,&'static str> {
        Ok(true)
    }
    fn clear(&mut self) -> Result<bool,&'static str> {
        Ok(true)
    }
}
