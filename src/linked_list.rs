#[derive(Debug,PartialEq)]
// implementation de liste chainee simple en rust 
// ici on vas le faire avec un type generique T 

pub struct LinkedList<T>{
    head : Option<Box<Node<T>>>,
}
#[derive(Debug,PartialEq)]
pub enum LinkedListErr {
    EmptyList ,
    ValueNotInList,
}
#[derive(Debug,PartialEq)]
struct Node<T>{
    value : T ,
    next : Option<Box<Node<T>>>
}
impl <T:PartialEq> LinkedList<T> {
    pub fn new()-> Self{
        LinkedList{head: Option::None}
    }

    pub fn len(&self)-> usize{
        let mut count =0;
        let mut current = &self.head ;
        while let Some(ref node)=current{
            count+=1;
            current=&node.next;
        }
        count
    }
    pub fn push_front(&mut self,value:T){
        // ajout value au debut de la liste 
        let next = self.head.take();
        let new_head = Box::new(Node{
            value ,
            next
        });
        self.head=Some(new_head);
    }
    pub fn push_back(&mut self,value:T){
        // ajoute value a la fin de la liste 
        let new_node = Box::new(Node{
            value ,
            next : Option::None,
        });

        match self.head {
            Some(ref mut node)=> {
                let mut tail =node;
                // traverser la liste jusqu'au dernier elt 
                while let Some(ref mut next_node)=tail.next{
                    tail=next_node;
                }
                tail.next=Some(new_node);
            },
            // liste vide 
            None=> self.head=Some(new_node),
        }
    }
    pub fn contains(&self,value:T)->Result<(),LinkedListErr>{
        let mut current = &self.head ;
        if current.is_none(){
            Err(LinkedListErr::EmptyList)
        }
        else{
            while let Some(ref node )=current {
                if node.value==value{
                    return Ok(());
                }
                current=&node.next;
            }
            Err(LinkedListErr::ValueNotInList)
        }
    }
}




#[cfg(test)]
pub mod tests{

    use super::*;
    #[test]
    fn create_empty_linked_list(){
        let l:LinkedList<u32>=LinkedList::new();
        assert!(l.head.is_none());
    }
    #[test]
    fn test_push(){
        let mut l =LinkedList::new();
        l.push_front(2);
        l.push_front(5);
        l.push_front(6);
        l.push_back(3);
        l.push_back(10);
        assert_eq!(l.len(),5);
        assert!(l.contains(6).is_ok());
        assert!(l.contains(3).is_ok());
        assert!(l.contains(7).is_err());
    }

}