
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
    fn set(&mut self,key : String , value : T) -> Result<bool,&'static str>;

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
    fn get(&self,key : String) -> Result<Option<T>,&'static str>;

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
    fn delete(&mut self,key : String) -> Result<Option<T>,&'static str>;

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
    fn update(&mut self,key : String,value : T) -> Result<bool,&'static str>;

    /**
     * Function : resize
     * @sync
     * About : Auto resizing of nodes ship 2x
     * Return :
     *      Ok : True if resize successed
     *      Err : The message error if resize has failed
     */
    fn resize(&mut self) -> Result<bool,&'static str>;

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