use std::{thread, time};

mod eventually_generated;

pub struct Module {
    their_example_client: eventually_generated::ExampleClient,
}

impl eventually_generated::ExampleUserService for Module {}

impl eventually_generated::ExampleSubscriber for Module {
    fn on_max_current(&self, value: f64) {
        println!("Received max_current: {value}");
    }
}

impl eventually_generated::Module for Module {
    fn main(&self) -> &dyn eventually_generated::ExampleUserService {
        self
    }

    fn their_example_subscriber(&self) -> &dyn eventually_generated::ExampleSubscriber {
        self
    }

    fn on_ready(&self) {
        // Ignoring any potential errors here.
        let _ = self
            .their_example_client
            .uses_something("hello_there".to_string());
    }
}

fn main() {
    let _mod = eventually_generated::init_from_commandline(|their_example_client| Module {
        their_example_client,
    });

    // Everest is driving execution in the background for us, nothing to do.
    loop {
        let dt = time::Duration::from_millis(250);
        thread::sleep(dt);
    }
}
