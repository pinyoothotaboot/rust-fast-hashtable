
pub trait Dict<T> {
    /**
     * Function : set
     * @sync
     * About : The function for set key and value to dictionary
     * Param :
     *      - key : The string of key
     *      - value : The value to store in dictionary
     * Return :
     *      - Ok : True when add success
     *      - Err : The message error when has problem to set in dictionary
     */
    fn set(&mut self,key : &Vec<u8> , value : T) -> Result<bool,&'static str>;

    /**
     * Function : get
     * @sync
     * About : The function for get value from dictionary by key
     * Param :
     *      - key : The string of key
     * Return :
     *      - Ok : The value from dictionary
     *      - Err : The message error when has problem to get from dictionary
     */
    fn get(&self,key : &Vec<u8>) -> Result<Option<T>,&'static str>;

    /**
     * Function : length
     * @sync
     * About : Get size of dictionary
     * Return :
     *      - usize : The length of nodes in dictionary seted
     */
    fn length(&self) -> usize;

    /**
     * Function : delete
     * @sync
     * About : Delete object in dictionary by key
     * Param :
     *      - key : The string of key
     * Return :
     *      - Ok : Value from object after deleted
     *      - Err : The message error if has problem delete
     */
    fn delete(&mut self,key : &Vec<u8>) -> Result<Option<T>,&'static str>;

    /**
     * Function : update
     * @sync
     * About : Update the value to dictionary by key
     * Param :
     *      - key : The string of key
     *      - value : The new value to update
     * Return :
     *      - Ok : True when update successed
     *      - Err : The message error if update has failed
     */
    fn update(&mut self,key : &Vec<u8>,value : T) -> Result<bool,&'static str>;

    /**
     * Function : clear
     * @sync
     * About : Clear all object in nodes
     * Return :
     *      - Ok : True if clear all successed
     *      - Err : The message error when clear all failed
     */
    fn clear(&mut self) -> Result<bool,&'static str>;

}


pub trait Set<T> {
    /**
     * Function : get
     * @sync
     * About : The function for get value from sets by key
     * Param :
     *      - key : The string of key
     * Return :
     *      - Ok : The value from sets
     *      - Err : The message error when has problem to get from sets
     */
    fn get(&self,key : &Vec<u8>) -> Result<Option<Vec<u8>>,&'static str>;

    /**
     * Function : add
     * @sync
     * About : The function for add key and value to sets
     * Param :
     *      - key : The string of key
     *      - value : The value to store in sets
     * Return :
     *      - Ok : True when add success
     *      - Err : The message error when has problem to set in sets
     */
    fn add(&mut self,key : &Vec<u8>) -> Result<bool,&'static str>;

    /**
     * Function : has
     * @sync
     * About : Check key already in setes
     * Param :
     *      - key : The key in sets
     * Return :
     *      - bool : True if key already insets , False if not in sets
     */
    fn has(&self,key : &Vec<u8>) -> bool;

    /**
     * Function : length
     * @sync
     * About : Get size of sets
     * Return :
     *      - usize : The length of nodes in sets
     */
    fn length(&self) -> usize;

    /**
     * Function : update
     * @sync
     * About : Update the value to sets by key
     * Param :
     *      - key : The string of key
     *      - value : The new value to update
     * Return :
     *      - Ok : True when update successed
     *      - Err : The message error if update has failed
     */
    fn update(&mut self,key : &Vec<u8>,new_key : &Vec<u8>) -> Result<bool,&'static str>;

    /**
     * Function : delete
     * @sync
     * About : Delete object in sets by key
     * Param :
     *      - key : The string of key
     * Return :
     *      - Ok : Value from object after deleted
     *      - Err : The message error if has problem delete
     */
    fn delete(&mut self,key : &Vec<u8>) -> Result<Option<T>,&'static str>;

    /**
     * Function : clear
     * @sync
     * About : Clear all object in nodes
     * Return :
     *      - Ok : True if clear all successed
     *      - Err : The message error when clear all failed
     */
    fn clear(&mut self) -> Result<bool,&'static str>;
}