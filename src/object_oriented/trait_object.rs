pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for i in self.components.iter() {
            i.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw a button! with label : {:?}", self.label);
    }
}
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw a SelectBox with text : {:?}", self.options);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem;

    use super::{Button, Screen, SelectBox};

    #[test]
    pub fn OOP_test() {
        let button1 = Box::new(Button {
            width: 12,
            height: 13,
            label: "123".to_string(),
        });
        let button2 = Box::new(Button {
            width: 12,
            height: 13,
            label: "456".to_string(),
        });
        let selct_box1 = Box::new(SelectBox {
            width: 111,
            height: 222,
            options: vec!["333".to_string()],
        });

        println!("size of : {}", mem::size_of::<SelectBox>());
        let vec: Vec<Box<dyn Draw>> = vec![button1, button2, selct_box1];
        let screen = Screen { components: vec };
        screen.run();
    }
}
