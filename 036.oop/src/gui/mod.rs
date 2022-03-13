pub trait Draw {
    fn draw(&self);
}

// dyn 是动态分配
pub struct Screen {
    pub components : Vec<Box<dyn Draw>>
}

// 也可以用泛型 
// pub struct  Screen1<T:Draw> {
//     pub components : Vec<T>
// }


impl Screen {
    pub fn run(&self) {
        for component in self.components.iter(){
            component.draw();
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
        println!("width {} heigth {} label {}",self.width,self.height,self.label)
    }
}


pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("width {} heigth {} option len {}",self.width,self.height,self.options.len())
    }
}



