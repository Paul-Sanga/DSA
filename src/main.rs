use doubly_linked_list::List;


fn main() {
    let mut list = List::new();
    list.add_last(5);
    list.add_last(6);
    list.add_last(7);
    // list.remove_last();
    list.remove_first();
    println!("List: {:?}", list);
    println!("The size of list is: {}, {:?}, {:?}", list.size(), list.head(), list.tail());
}
