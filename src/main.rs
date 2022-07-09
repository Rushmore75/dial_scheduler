pub mod notification;
pub mod node;


use std::time::{SystemTime, Duration};

use node::Node;

use crate::notification::Notification;



fn main() {


    // let node = Node::new();

            
    let noti = Notification::new(get_time().as_secs() + 2, "This");

    let mut node = Node::<i32>::new();
    node.put_at(12, 12);


}

fn get_time() -> Duration {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
}