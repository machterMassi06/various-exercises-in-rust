///implementation of a queue(FIFO) in rust, I know it's easy to do with just a Vec<T> vector...
/// ... and its push and remove(idx) methods, but I implement it this way ...
/// ... to explore the model rust memory
///
#[derive(Debug,PartialEq)]
pub struct Queue<T>{
    head : Option<Box<Node<T>>>,
    tail :*mut Node<T> ,
}

#[derive(Debug,PartialEq)]
struct Node<T>{
    elem : T , 
    next : Option<Box<Node<T>>>,
}

impl <T> Queue<T>{
    //Create a new empty queue
    pub fn new()->Self{
        Queue { head: None, tail:std::ptr::null_mut() }
    }
}


#[cfg(test)]
pub mod tests{
    use super::*;
    #[test]
    fn test_create_empty_queue(){
        let q:Queue<i32> = Queue::new();
        assert!(q.head.is_none());
        assert!(q.tail.is_null());
    }
}