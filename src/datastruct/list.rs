use std::{borrow::Borrow, cell::RefCell, rc::Rc};
//先实现一个 i32 类型的试试
pub struct LinkedList {
    value: i32,
    //Rc 版本的链表，使用 RefCell表示这个 Option 是内部可变的，即使 LinkedList 是一个不可变引用。
    next: RefCell<Option<Rc<LinkedList>>>,
}

impl Iterator for LinkedList {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.value)
    }
}

impl LinkedList {
    pub fn new(value: i32) -> Rc<Self> {
        Rc::new(Self {
            value,
            next: RefCell::new(None),
        })
    }

    pub fn push_back(&self, node: Rc<Self>) {
        if self.next.borrow().is_some() {
            self.next.borrow().as_ref().unwrap().push_back(node);
        } else {
            *self.next.borrow_mut() = Some(node);
        }
    }

    pub fn push_back_by_value(&mut self, value: i32) {
        self.push_back(LinkedList::new(value));
    }

    pub fn print_list(&self) {
        print!("{}->", self.value);
        if self.next.borrow().is_some() {
            self.next.borrow().as_ref().unwrap().print_list();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn list_test() {
        let head = LinkedList::new(0);
        head.push_back(LinkedList::new(1));
        head.push_back(LinkedList::new(2));
        head.push_back(LinkedList::new(3));
        head.push_back(LinkedList::new(4));
        head.push_back(LinkedList::new(5));
        head.push_back(LinkedList::new(6));

        head.print_list();
    }
}
