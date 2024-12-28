#[derive(Debug,PartialEq)]
///pile avec un type generique T 
pub struct Stack<T>{
    head :Option<Box<Node<T>>>,
}
#[derive(Debug,PartialEq)]
struct Node<T>{
    elem:T,
    next:Option<Box<Node<T>>>,

}
impl <T:Copy> Stack<T>{
    pub fn empty_stack()-> Self{
        Stack { head: None }
    }

    pub fn top(s: &Self) ->Option<T> {
        match &s.head {
            None => None,
            Some(n)=> Some(n.elem),
        }
    }
    pub fn push(s: &mut Self, elem: T) {
        let next =s.head.take() ; //ou std::mem::take(&mut s.head)
        let newtop = Box::new(Node{elem, next});
        s.head = Some(newtop);
    }
    pub fn pop(s: &mut Self)->Option<T>{
        match &mut s.head {
            Some(n)=> {
                let value =n.elem;
                s.head=n.next.take();
                Some(value)
            },
            None => None ,

        }
    }
}

#[cfg(test)]
pub mod tests{
    use super::*;
    #[test]
    fn sould_create_empty_stack(){
        let s:Stack<i32>=Stack::empty_stack();
        assert_eq!(s,Stack{head:None});
    }
    #[test]
    fn should_return_the_top_of_an_empty_stack(){
        let s:Stack<i32>=Stack::empty_stack();
        let top=Stack::top(&s);
        assert_eq!(None,top);
    }
    #[test]
    fn should_return_the_top_of_the_stack(){
        let s =Stack{head:Some(Box::new(Node{elem:4,next:None}))};
        assert_eq!(4,Stack::top(&s).unwrap());
    }
    #[test]
    fn should_push_an_element_on_the_stack(){
        let mut s = Stack::empty_stack();
        Stack::push(&mut s,5);
        assert_eq!(5,Stack::top(&s).unwrap());
    }
    #[test]
    fn should_pop_the_empty_stack() {
        let mut s:Stack<i32> = Stack::empty_stack();
        let top_of_stack = Stack::pop(&mut s);
        assert_eq!(None, top_of_stack);
        assert_eq!(None, Stack::top(&s));
    }

    #[test]
    fn should_pop_an_element_from_the_stack() {
        let mut s = Stack::empty_stack();
        Stack::push(&mut s,1);
        assert_eq!(1, Stack::top(&s).unwrap());
        let top_of_stack = Stack::pop(&mut s);
        assert_eq!(1, top_of_stack.unwrap());
        assert_eq!(None,Stack::top(&s));
    }
}