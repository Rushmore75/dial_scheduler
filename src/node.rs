pub struct Node<T> {
    
    // a raw pointer to the parent, seems like a good idea
    parent: Option<*const Node<T>>,
    // boxed so it goes on the heap instead of the stack, making my life easier :D
    children: [Box<Option<Node<T>>>; 2], // TODO location to change if children pool number gets changed
    // the contents will function as the data at each "index"
    contents: Option<T>,    
}

impl<'a, T: Copy> Node<T> {
    /*
        private functions
    */
    fn internal_new(parent: Option<*const Node<T>>) -> Self {
        // should all the boxed None-s get combined? (less memory usage)
        match parent {
            // TODO location to change if children gets changed
            Some(x) => return Self {
                parent: Some(x), // stores a pointer to the parent
                children: [Box::new(None), Box::new(None)],
                contents: None, // by default contents will just be none, it can be filled with the corresponding function
            },
            None => return Self { 
                parent: None,
                children: [Box::new(None), Box::new(None)],
                contents: None,
            },
        };
    }

    fn create_child(&'a mut self, switch: i32) -> Result<&str, &str> {

        /*
            Returns self
        */

        // basically a function, it's called a closure
        let mut execute = |loc: usize| {
            // get a raw pointer
            let p_self: *const Node<T> = self;
            // create the child node
            let child_node = Node::internal_new(Some(p_self));
            // place the child in the list of the parent at index 
            self.children[loc] = Box::new(Some(child_node)); 
        };

        match switch {
            
            0 => {
                execute(0);
                Ok("ok")
                },
            1 =>{
                execute(1);
                Ok("ok")
                },
            i32::MIN..=i32::MAX => Err("The node children needs a number 0 or 1"),   
        }
    }

    fn get_child(&'a mut self, index: usize) -> &mut Option<Node<T>> {
        // dereference out of the Box
        // then borrow mutably
        &mut *self.children[index]
    }

    fn get_parent(&self) -> Option<&Node<T>> {
        
        match self.parent {
            Some(p) => {
                unsafe {
                    // checks that there is data at the pointer
                    if let Some(val) = p.as_ref() {
                        // https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref
                        Some(val)
                    } else {
                        panic!("the raw pointer to a child's parent node didn't lead to anything")
                    }
                }
            },
            // the Node has no parents
            None => None,
        }
    }

    /*
        public functions
    */

    pub fn new() -> Self {
        /*
            The external new method, this just creates the first Node, hence why
            there is not option for making a parent
        */
        Self { 
            parent: None,
            children: [Box::new(None), Box::new(None)], 
            contents: None,
        }
    }


    pub fn put_at(& mut self, item: T, index: usize) {
        
        let bits = 63; // this is the max (right or left non overflowing) bit-shift possible on usize

        /*
            Follow the Node tree down
        */

        let mut current_node: &mut Node<T> = self;
        // start at the maximum possible bit-shift then work down
        for i in (0..bits).rev() {

            // decides if you take child 1 or 0
            let loc_index = index & (1 << i);
            // get the child
            let child: &mut Option<Node<T>> = current_node.get_child(loc_index);                
            
            match child {
                // Some means that there are more children to search through
                Some(x) => current_node = x,
                // None means that we have reached the end of the tree
                None => break, // I don't think this break is needed, it should be the last iteration of the for loop
            };
        }
        current_node.contents = Some(item);

    }

}