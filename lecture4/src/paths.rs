pub mod shapes {
    pub fn new_rect(width: u32, height: u32) -> rectangles::Rect {
            rectangles::Rect {
                    width,
                    height
            }
    }

    pub mod rectangles {
            pub struct Rect {
                    pub width: u32,
                    pub height: u32
            }

            impl Rect {
                    pub fn get_area(&self) -> u32 {self.width * self.height }
                    pub fn get_perimeter(&self) -> u32 {self.width * 2 + self.height * 2 }
            }

    }
}

pub fn create_rectangle_v1() {
        shapes::rectangles::Rect {
                width: 5,
                height: 5
        };
    
        shapes::new_rect(5, 5);
    }
    
pub fn create_rectangle_v2() {
    self::shapes::rectangles::Rect {
            width: 5,
            height: 5
    };

    self::shapes::new_rect(5, 5);
}


pub fn create_rectangle_v3() {
        crate::paths::shapes::rectangles::Rect {
                width: 5,
                height: 5
        };
    
        crate::paths::shapes::new_rect(5, 5);
    }

mod create_rectangle_v4 {

    pub fn create_rectangle_v4() {
        super::shapes::rectangles::Rect{
                width: 5,
                height: 5
        };
        super::shapes::new_rect(5, 5);

    }
}