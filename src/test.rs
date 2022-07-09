use std::time::Duration;

use crate::notification::Notification;


pub struct Tree<'a> {
    // holds more trees
    ring: Option<Vec<Tree<'a>>>,
    // holds batches of Notifications to send out
    batch: Option<Vec<Notification<'a>>>
}

impl<'a> Tree<'a> {

    pub fn new() -> Self {

        Self {
            ring: None,
            batch: None
        }
    }

    pub fn add_notification(&self, noti: Notification) {
        let time = noti.get_fire_time();
        

        let mut root = self;

        // steps through each bit starting at bit 128 and works down to bit 1
        for shift in (1..=63).rev() {
            // gets each bit (1 or 0)
            let switch: usize = ((time & (1 << shift)) >> shift).try_into().unwrap();
        

            // set the current tree as a different tree one level down
            if root.ring.is_none() {
                &root.ring.unwrap();
            }

            root = &root.ring.unwrap()[switch];  
            
        }


        match &root.batch {
            Some(_) => {},
            None => {
                let vec = Vec::new();
                root.batch = Some(vec);
            },
        }
        let x = root.batch.as_mut();
        x.unwrap();
    }

    pub fn fire(&'a self, fire: Duration) {
        
        let mut root = self;

        // steps through each bit starting at bit 128 and works down to bit 1
        for shift in (1..=63).rev() {
            // gets each bit (1 or 0)
            let switch: usize = ((fire.as_secs() & (1 << shift)) >> shift).try_into().unwrap();
        
            // set the current tree as a different tree one level down
            if root.ring.is_none() {
                // value is none thusfor we haven't placed a notification down here.
                return;
            }

            root = &mut &root.ring.unwrap()[switch];
        }
        // at this point root is the lowest level of the Tree(s)
        match &root.batch {
            // if the lowest level batch has notifications fire them
            Some(x) => {
                
                for y in x.iter() {

                    println!("{}", y.get_msg());
                }
            },
            // no notifications!
            None => {},
        }
    }


}

