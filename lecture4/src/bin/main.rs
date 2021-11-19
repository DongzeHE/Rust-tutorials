// use crate::paths;
use lecture4_lib::paths;

// mod modC;
use lecture4_lib::modC::modC_helper;
use lecture4_lib::modC;

use paths::shapes::new_rect;

use std::{collections::HashMap, io::BufRead};

mod binA;



fn main() {
    let rect = crate::paths::shapes::rectangles::Rect{
        width: 5,
        height: 6,
    };
    let rect1 = new_rect(1, 2);
    rect.get_area();
    rect1.get_area();
    let mut map = HashMap::new();
    map.insert(1, 1);

    let paths_path = std::path::Path::new("src/paths.rs");

    let f =  std::fs::File::open(paths_path).unwrap();
    let bf = std::io::BufReader::new(f);
    for l in bf.lines() {
        println!("{}", l.unwrap())
    }
}
