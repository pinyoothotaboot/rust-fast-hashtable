

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

    #[inline]
    pub fn matched(&self,key : &Vec<u8>) -> bool {
        let len_key1 = key.len();
        let len_key2 = self.key.len();
    
        // Confirm length of 2 strings has equal
        if len_key1 - len_key2 != 0 {
            return false;
        }
    
        // Comfirm first and last charactor has matched
        let first_last = ( key[0] - self.key[0] ) + ( key[len_key1 - 1] - self.key[len_key2 - 1] );
        if first_last != 0 {
            return false;
        }
    
        // Mid = ((total length - length of 2 chars) / 2 )  + 1
        // Example :
        // keys = 5000 charactors
        // mid  = 5000 - 2 / 2 + 1
        //      = 2500
        let mid : usize = (len_key1 - 2) / 2 + 1;
        let shif_mid : usize = mid - 1;
    
        // Loop i = 2 to 2500 - 1 , i+=2
        // Forward step :
        //    ODD step    - x1[i] - x2[i] , 2,4,6,8...2498 (<2500-1 = 2549) , i+=2
        //    EVEN step   - x1[i-1] - x2[i-1] , 1,3,5,7,...2547 , i+=1
        // Backward step :
        //    ODD step    - x1[mid + i] - x2[mid + i] , 2500 + 2,2500 + 4 ,2500 + 6,...2500 + 2498=4998 , i+=2
        //    EVEN        - x1[mid + i - 1] - x2[mid + i - 1] , 2500 + 2 - 1 , 2500 + 4 - 1,...2500 + 2498 - 1 = 4997
        for i in (2..mid - 1).step_by(2)  {
            let result = ( key[i] -   self.key[i] ) + 
                             ( key[i-1] - self.key[i-1] ) +
                             ( key[mid + i] - self.key[mid + i] ) + 
                             ( key[shif_mid + i] - self.key[shif_mid + i] );
            if result != 0 {
                return false;
            }
        }
        
        true
    }


    pub fn get_value(&self) -> Option<T> {
        self.value.clone()
    }

    pub fn get_key(&self) -> Vec<u8> {
        self.key.clone()
    }

    pub fn set_value(&mut self,value : T) {
        self.value = Some(value);
    }

    pub fn set_key(&mut self,new_key : Vec<u8>) {
        self.key = new_key;
    }

}