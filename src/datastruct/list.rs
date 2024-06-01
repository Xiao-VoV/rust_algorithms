use std::{fmt::DebugStruct, rc::Rc};
//先实现一个 i32 类型的试试
pub struct LinkedList {
    value: i32,
    next: Option<Box<LinkedList>>,
}

impl LinkedList {
    pub fn new(value: i32) -> Box<Self> {
        Box::new(Self { value, next: None })
    }

    pub fn push_back(&mut self, node: Box<Self>) {
        if self.next.is_some() {
            self.next.as_mut().unwrap().push_back(node);
        } else {
            self.next = Some(node);
        }
    }

    pub fn push_back_by_value(&mut self, value: i32) {
        self.push_back(LinkedList::new(value));
    }

    pub fn print_list(&self) {
        print!("{}->", self.value);
        if self.next.is_some() {
            self.next.as_ref().unwrap().print_list();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn list_test() {
        let mut head = LinkedList::new(0);
        head.push_back(LinkedList::new(1));
        head.push_back(LinkedList::new(2));
        head.push_back(LinkedList::new(3));
        head.push_back(LinkedList::new(4));
        head.push_back(LinkedList::new(5));
        head.push_back(LinkedList::new(6));

        head.print_list();
    }
}
