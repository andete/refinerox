extern crate yew;
extern crate refinerox;

use yew::prelude::*;
use refinerox::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
