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
    /// Create a new empty queue.
    pub fn new()->Self{
        Queue { head: None, tail:std::ptr::null_mut() }
    }
    
    /// Add an element to the end of the queue.
    pub fn enqueue(&mut self , elem : T){
        let mut new_node=Box::new(Node{elem,next:None});
        //Get a raw pointer to the new node
        let new_tail = new_node.as_mut() as *mut _;

        if self.head.is_none(){
            self.head=Some(new_node);
        }else{
            unsafe{
                (*self.tail).next=Some(new_node)
                ;
            }
        }

        self.tail=new_tail;
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

    #[test]
    fn test_enqueue(){
        let mut q = Queue::new();
        q.enqueue(2);
        q.enqueue(3);
        q.enqueue(4);
        assert_eq!(q.head.unwrap().elem, 2);
        unsafe {
            assert_eq!((*q.tail).elem, 4);
            assert!((*q.tail).next.is_none());
        }
    }
}