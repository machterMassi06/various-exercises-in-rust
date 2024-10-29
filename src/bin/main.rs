use test_rust::linked_list::LinkedList;
fn main () {
    let mut l = LinkedList::new();
    l.push_front(3);
    l.push_front(2);
    l.push_back(5);
    println!("{:?}",l);
}
