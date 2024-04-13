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

    fn set_table_size(&mut self,size : usize) {
        self.table_size = size;
    }

    fn set_nodes(&mut self,new_nodes : Vec<Option<Box<Node<T>>>>) {
        self.nodes = new_nodes;
    }

    fn available_size(&self) -> usize {
        let mut count : usize = 0;
        for node in self.nodes.iter() {
            if node.is_none() {
                continue;
            }
            count+=1
        }
        count
    }

    pub fn display(&self) {
        for node in self.nodes.iter() {
            println!("NODE : {:?}\n",node);
        }
    }

    fn resize(&mut self) -> Result<bool,&'static str> {
        // Calculate current size div table size
        let d = (self.available_size() / self.get_table_size()) as f32;
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
        match hash(&bytes_key.clone(), self.get_table_size(), self.get_seed()) {
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
                                        return Ok(value)
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

    fn set(&mut self,key : String , value : T) -> Result<bool,&'static str> {
        let bytes_key = key.as_bytes().to_vec();
        let result : Result<bool,&'static str> = match hash(&bytes_key, self.get_table_size(), self.get_seed()) {
            Ok(index) => {
                let result = match self.nodes.get_mut(index) {
                    Some(mut first_node) => {
                        //println!("FIRST NODE : {:?}",first_node);
                        // Insert first if none
                        if first_node.is_none() {
                            // Node* mFirst = First->next
                            let current : Option<Box<Node<T>>> = mem::replace(&mut first_node,None);
                            // Node* tmp = new Node(element,mFirst)
                            let node : Node<T> = Node::<T>::new(bytes_key,Some(value),current);
                            // Node* First = tmp
                            *first_node = Some(Box::new(node));

                            self.increase();
                            //return Ok(true);
                        } else {
                            let mut first = first_node.as_mut();
                            // Loop first node is not None
                            let mut flag : bool = true;
                            while !first.is_none() {
                                first = match first {
                                    Some(node) => {
                                        // Matching key in node
                                        // Then matched return message already added
                                        if node.matched(&bytes_key) {
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
                                let node : Node<T> = Node::new(bytes_key, Some(value), current);
                                *first_node = Some(Box::new(node));
                                self.increase();
                            } else {
                                return Err("The key already seted");
                            }
                        }

                        Ok(true)
                    },
                    None => Err("Not found"),
                };
                result
            },
            Err(e) => Err(e)
        };

        /**
         * TODO :: Failed update
         *
        match result {
            Ok(inserted) => {
                match self.resize() {
                    Ok(resized) => Ok(resized),
                    Err(_e) => Ok(inserted)
                }
            },
            Err(e) => Err(e)
        }
        */
        result
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
