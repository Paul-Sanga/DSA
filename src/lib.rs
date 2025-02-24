use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Default, Debug)]
pub struct Node{
    prev: Option<Weak<RefCell<Node>>>,
    value: u32,
    next: Option<Rc<RefCell<Node>>>
}

impl Node {
    fn new(value: u32)->Self{
        Self { prev: None, value, next: None }
    }
    fn get_value(&self)->u32{
        self.value
    }
}

#[derive(Default, Debug)]
pub struct List{
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Weak<RefCell<Node>>>,
    size: u128
}

impl List{
    pub fn new()->Self{
        Self { head: None, tail: None, size: 0 }
    }

    pub fn add_first(&mut self, value: u32){
        let node = Rc::new(RefCell::new(Node::new(value)));
        if self.size == 0{
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::downgrade(&node));
            self.size += 1;
        }else{
            if let Some(current_head) = &self.head{
                current_head.borrow_mut().prev = Some(Rc::downgrade(&node));
                node.borrow_mut().next = Some(Rc::clone(&current_head));
                self.head = Some(Rc::clone(&node));
                self.size += 1;
            }else{
                println!("Empty head node!!")
            }
        }
    }

    pub fn add_last(&mut self, value: u32){
        let node = Rc::new(RefCell::new(Node::new(value)));
        if self.size == 0{
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::downgrade(&node));
            self.size += 1;
        }else {
            if let Some(current_tail) = &self.tail{
                if let Some(tail_node) = current_tail.upgrade(){
                    node.borrow_mut().prev = Some(Rc::downgrade(&tail_node));
                    tail_node.borrow_mut().next = Some(Rc::clone(&node));
                    self.tail = Some(Rc::downgrade(&node));
                    self.size += 1;
                }else{
                    println!("Empty tail node");
                }
            }
        }
    }

    pub fn head(&self)->Option<u32>{
        if let Some(node) = &self.head{
            return Some(node.borrow().get_value());
        }else{
            None
        }
    }

    pub fn tail(&self)->Option<u32>{
        if let Some(tail_node) = &self.tail{
            if let Some(tail) = tail_node.upgrade(){
                return Some(tail.borrow().get_value());
            }else {
                None
            }
        }else {
            None
        }
    }

    pub fn size(&self) -> u128{
        self.size
    }

}