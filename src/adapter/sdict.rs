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

impl <T> Drop for SDict<T> {
    fn drop(&mut self) {
        // Loop to drop all node in tables
        for index in 0..self.nodes.len() {
            match self.nodes.get_mut(index) {
                Some(node) => {
                    let mut cur_link = mem::replace(node, None);
                    while let Some(mut boxed_node) = cur_link {
                        cur_link = mem::replace(&mut boxed_node.next, None);
                    }
                },
                None => {}
            }
        }

        // Clear all in nodes
        self.nodes.clear();
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

                                    // Next node
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
                        let mut first = first_node.as_mut();
                        // Loop first node is not None
                        while !first.is_none() {
                            first = match first {
                                Some(node) => {
                                    // Matching key in node
                                    // Then matched return message already added
                                    if node.matched(&bytes_key) {
                                        return Err("The key already added.");
                                    }

                                    // Next node
                                    //node.next
                                    node.next.as_mut().map(|node| &mut *node)
                                },
                                None => None
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
        // Convert strings to ascii number in vector
        let bytes_key = key.as_bytes().to_vec();

        // Calculate hash function and return index of tables
        match hash(&bytes_key, self.get_table_size(), self.get_seed()) {
            Ok(index) => {
                // Get node by index
                match self.nodes.get_mut(index) {
                    Some(first_node) => {

                        // Take first node by reference
                        let mut first = first_node.as_mut();
                        while !first.is_none() {
                            first = match first {
                                Some(node) => {

                                    // Matching key in node
                                    if node.matched(&bytes_key) {
                                        let value = node.get_value();

                                        // If take first node and no more in list
                                        if node.next.is_none() {
                                            *first_node = None;
                                            self.decrease();
                                            return Ok(value);
                                        }

                                        //mFirst = mFirst->next
                                        node.next.as_mut().map(|node| &mut *node);
                                        
                                        // x = mFirst->next->next
                                        let mut x = node.next.as_mut().map(|node| &mut *node);
                                        // Drop x
                                        match mem::replace(&mut x, None) {
                                            Some(next) => {
                                                // mFirst->next = x->next
                                                node.next = Option::take(&mut next.next);
                                                self.decrease();
                                                return Ok(value);
                                            },
                                            None => {
                                                return Err("Not found");
                                            }
                                        }
                                    }

                                    // mFirst -> mFirst->next
                                    node.next.as_mut().map(|node| &mut *node)
                                },
                                None => break
                            };
                        };
                        
                        Err("The nodes of index is empty!.")
                    },
                    None => Err("Not found node")
                }
            },
            Err(e) => Err(e)
        }
    }

    fn update(&mut self,key : String,value : T) -> Result<bool,&'static str> {
        let bytes_key = key.as_bytes().to_vec();
        match hash(&bytes_key, self.get_table_size(), self.get_seed()) {
            Ok(index) => {
                match self.nodes.get_mut(index) {
                    Some(mut first_node) => {
                        let mut first = first_node.as_mut();

                        while !first.is_none() {
                            first = match first {
                                Some(node) => {
                                    // Matching key in node
                                    // Then matched return message already updated
                                    if node.matched(&bytes_key) {
                                        node.set_value(value);
                                        return Ok(true);
                                    }

                                    // Next node
                                    //node.next
                                    node.next.as_mut().map(|node| &mut *node)
                                },
                                None => None
                            }
                        }

                        Err("Not found node matched key!.")
                    },
                    None => Err("Not found node") 
                }
            },
            Err(e) => Err(e) 
        }
    }

    fn resize(&mut self) -> Result<bool,&'static str> {
        Ok(true)
    }

    fn clear(&mut self) -> Result<bool,&'static str> {
        for index in 0..self.nodes.len() {
            match self.nodes.get_mut(index) {
                Some(node) => {
                    let mut cur_link = mem::replace(node, None);
                    while let Some(mut boxed_node) = cur_link {
                        cur_link = mem::replace(&mut boxed_node.next, None);
                    }
                },
                None => {}
            }
        }

        self.size = 0;
        Ok(true)
    }
}
