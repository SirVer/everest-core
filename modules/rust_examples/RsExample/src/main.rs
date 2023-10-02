use eventually_generated::KvsService;
use std::{thread, time};

mod eventually_generated;

pub struct Module {
    foobar_publisher: eventually_generated::FoobarPublisher,
    their_store_client: eventually_generated::KvsClient,
}

impl KvsService for Module {
    fn store(&self, key: String, value: serde_json::Value) -> ::everestrs::Result<()> {
        self.their_store_client.store(key, value)
    }

    fn load(&self, key: String) -> ::everestrs::Result<serde_json::Value> {
        self.their_store_client.load(key)
    }

    fn delete(&self, key: String) -> ::everestrs::Result<()> {
        self.their_store_client.delete(key)
    }

    fn exists(&self, key: String) -> ::everestrs::Result<bool> {
        self.their_store_client.exists(key)
    }
}

impl eventually_generated::ExampleService for Module {
    fn uses_something(&self, key: String) -> ::everestrs::Result<bool> {
        if !self.their_store_client.exists(key.clone())? {
            println!("IT SHOULD NOT AND DOES NOT EXIST");
        }

        let test_array = vec![1, 2, 3];
        self.their_store_client
            .store(key.clone(), test_array.clone().into())?;

        let exi = self.their_store_client.exists(key.clone())?;
        if exi {
            println!("IT ACTUALLY EXISTS");
        }

        let ret: Vec<i32> = serde_json::from_value(self.their_store_client.load(key)?)
            .expect("Wanted an array as return value");

        println!("loaded array: {ret:?}, original array: {test_array:?}");
        Ok(exi)
    }
}

impl eventually_generated::Module for Module {
    fn foobar(&self) -> &dyn eventually_generated::ExampleService {
        self
    }

    fn my_store(&self) -> &dyn KvsService {
        self
    }

    fn on_ready(&self) {
        println!("Welcome to the RsExample module!");
        // Ignoring any potential errors here.
        // TODO(sirver): This should use the configuration values
        let _ = self.foobar_publisher.publish_max_current(32.);
    }
}

fn main() {
    let _mod =
        eventually_generated::init_from_commandline(|foobar_publisher, their_store_client| {
            Module {
                foobar_publisher,
                their_store_client,
            }
        });

    // Everest is driving execution in the background for us, nothing to do.
    loop {
        let dt = time::Duration::from_millis(250);
        thread::sleep(dt);
    }
}
