extern crate yew;
extern crate refinerox_front;

use yew::prelude::*;
use refinerox_front::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
