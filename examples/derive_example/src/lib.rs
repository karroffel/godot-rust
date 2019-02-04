extern crate gdnative_derive;
extern crate gdnative;

use gdnative_derive::gdnative_expose;

struct Tower {}

#[gdnative_expose]
impl Tower {

    #[export]
    pub fn shoot(&self, dir: gdnative::Vector3) -> () {

    }
}