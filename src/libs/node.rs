

#[derive(Clone,PartialEq,Debug)]
pub struct Node<T> {
    value : Option<T>,
    key : Vec<u8>,
    pub next : Option<Box<Node<T>>>
}

impl <T> Node<T>
where T : Clone + std::fmt::Debug
{
    pub fn new(key : Vec<u8> , value : Option<T>,next : Option<Box<Node<T>>>) -> Self {
        Node { value, key, next}
    }

    pub fn set_next(&mut self,node : Option<Box<Node<T>>>) {
        self.next = node;
    }

    #[inline]
    pub fn matched(&self,key : &Vec<u8>) -> bool {
        // Check length of hash key and store hash key.
        // When length of 2 keys has not match.Then return false
        if key.len() != self.key.len() {
            return false;
        }

        // Check first and last charactor key.
        // If not matched return false
        if  key.first() != self.key.first()  || 
            key.last()  != self.key.last()
        {
            return false;
        }

        // Loop from charactor at 1 to before last char of key
        for i in 1..self.key.len() - 1 {
            if key[i] - self.key[i] != 0 {
                return false;
            }
        }

        return true;
    }


    pub fn get_value(&self) -> Option<T> {
        self.value.clone()
    }

    pub fn get_key(&self) -> Vec<u8> {
        self.key.clone()
    }

}