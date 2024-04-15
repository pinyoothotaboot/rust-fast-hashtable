use std::{fmt::Debug, mem};
use rand::RngCore;
use crate::libs::node::{Node};
use crate::libs::interface::{Set};
use crate::libs::hash::{hash};
use crate::libs::constant::{TABLE_SIZE};

#[derive(Clone,PartialEq,Debug)]
pub struct SSet<T> {
    size : usize,
    seed : u64,
    nodes : Vec<Option<Box<Node<T>>>>,
    table_size : usize
}

impl <T> SSet<T>
where T : Clone + std::fmt::Debug
{
    pub fn new() -> Self {
        let mut nodes : Vec<Option<Box<Node<T>>>> = Vec::new();
        for _ in 0..TABLE_SIZE {
            nodes.push(None);
        }

        let seed = rand::thread_rng().next_u64();

        SSet { 
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

    fn set_table_size(&mut self,size : usize) {
        self.table_size = size;
    }

    fn set_nodes(&mut self,new_nodes : Vec<Option<Box<Node<T>>>>) {
        self.nodes = new_nodes;
    }

    fn resize(&mut self) -> Result<bool,&'static str> {
        // Calculate current size div table size
        let d = (self.size / self.get_table_size()) as f32;
        let mut new_size : usize = self.get_table_size();

        // If key store >85% of table.Then new size = current size * 2
        // If not key store <25% of table.Then new size = current size / 2
        if d > 0.85 {
            new_size*=2;
        } else if d < 0.25 {
            new_size /= 2;
        } else {
            return Err("Inrange key using");
        }

        // The table size available default at 1024
        if TABLE_SIZE > new_size as u32 {
            return Err("The table size has lessthan deafult");
        }

        // Update new table size
        self.set_table_size(new_size);

        // Initial new tables
        let mut new_nodes : Vec<Option<Box<Node<T>>>> = Vec::new();
        for _ in 0..self.get_table_size() {
            new_nodes.push(None);
        }

        // Loop all current nodes
        for node in self.nodes.iter() {
            // Skip if node is none
            if node.is_none() {
                continue;
            }

            match node {
                Some(obj) => {
                    let key = obj.get_key();
                    // Re-hash function
                    match hash(&key, self.get_table_size(), self.get_seed()) {
                        Ok(index) => {
                            // Insert the node to new table
                            new_nodes.insert(index, node.clone());
                        },
                        Err(e) => {
                            println!("{:?}",e);
                            continue;
                        }
                    }
                },
                None => continue
            }
        }

        // Update new nodes
        self.set_nodes(new_nodes);

        Ok(true)
    }
}

impl <T> Drop for SSet<T> {
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

impl <T> Set<T> for SSet<T>
where T : Clone + std::fmt::Debug
{
    fn add(&mut self,key : &Vec<u8>) -> Result<bool,&'static str> {
        let _resized = self.resize();
        match hash(key, self.get_table_size(), self.get_seed()) {
            Ok(index) => {
                match self.nodes.get_mut(index) {
                    Some(mut first_node) => {

                        let mut first = first_node.as_mut();
                        // Loop first node is not None
                        let mut flag : bool = true;
                        while !first.is_none() {
                            first = match first {
                                Some(node) => {
                                    // Matching key in node
                                    // Then matched return message already added
                                    if node.matched(key) {
                                        flag = false;
                                        break;
                                    }
                                    // Next node
                                    node.next.as_mut().map(|node| &mut *node)
                                },
                                None => None
                            }
                        }; 

                        // If first node not found data or not matched key
                        // Then add new node
                        if flag {
                            let current : Option<Box<Node<T>>> = mem::replace(&mut first_node,None);
                            let node : Node<T> = Node::new(key.clone(), None, current);
                            *first_node = Some(Box::new(node));
                            self.increase();
                            return Ok(true);
                        }

                        return Err("The key already seted");
                        
                    },
                    None => Err("Not found"),
                }
            },
            Err(e) => Err(e)
        }
    }

    fn get(&self,key : &Vec<u8>) -> Result<Option<Vec<u8>>,&'static str> {
        match hash(key, self.get_table_size(), self.get_seed()) {
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
                                    if node.matched(key) {
                                        return Ok(Some(node.get_key()));
                                    } else {
                                        // Next node
                                        &node.next
                                    }
                                },
                                None => &None
                            }
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

    fn has(&self,key : &Vec<u8>) -> bool {
        match hash(key, self.get_table_size(), self.get_seed()) {
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
                                    if node.matched(key) {
                                        return true;
                                    } else {
                                        // Next node
                                        &node.next
                                    }
                                },
                                None => &None
                            }
                        }
                        false
                    },
                    None => {
                        return false;
                    }
                }
            },
            Err(e) =>{
                return false;
            }
        }
    }

    fn update(&mut self,key : &Vec<u8>,new_key : &Vec<u8>) -> Result<bool,&'static str> {
        match hash(key, self.get_table_size(), self.get_seed()) {
            Ok(index) => {
                match self.nodes.get_mut(index) {
                    Some(first_node) => {

                        let mut first = first_node.as_mut();

                        while !first.is_none() {
                            first = match first {
                                Some(node) => {
                                    // Matching key in node
                                    // Then matched return message already updated
                                    if node.matched(key) {
                                        node.set_key(new_key.clone());
                                        return Ok(true);
                                    }

                                    // Next node
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

    fn delete(&mut self,key : &Vec<u8>) -> Result<Option<T>,&'static str> {
        // Calculate hash function and return index of tables
        match hash(key, self.get_table_size(), self.get_seed()) {
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
                                    if node.matched(key) {
                                        let value = node.get_value();

                                        // If take first node and no more in list
                                        if node.next.is_none() {
                                            *first_node = None;
                                            self.decrease();

                                            // Auto resize table
                                            match self.resize() {
                                                Ok(_) => return Ok(value),
                                                Err(_) => return Ok(value)
                                            };
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

                                                // Auto resize table
                                                match self.resize() {
                                                    Ok(_) => return Ok(value),
                                                    Err(_) => return Ok(value)
                                                };
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

    fn length(&self) -> usize {
        self.size
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