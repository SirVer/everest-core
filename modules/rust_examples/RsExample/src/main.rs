mod eventually_generated;
use eventually_generated::{
    ExampleServiceSubscriber, KvsClientSubscriber, KvsServiceSubscriber, Module, ModulePublisher,
    OnReadySubscriber,
};
use std::sync::Arc;
use std::{thread, time};


pub struct OneClass {}

impl KvsServiceSubscriber for OneClass {
    fn store(
        &self,
        pub_impl: &ModulePublisher,
        key: String,
        value: serde_json::Value,
    ) -> ::everestrs::Result<()> {
        pub_impl.their_store_publisher.store(key, value)
    }

    fn load(
        &self,
        pub_impl: &ModulePublisher,
        key: String,
    ) -> ::everestrs::Result<serde_json::Value> {
        pub_impl.their_store_publisher.load(key)
    }

    fn delete(&self, pub_impl: &ModulePublisher, key: String) -> ::everestrs::Result<()> {
        pub_impl.their_store_publisher.delete(key)
    }

    fn exists(&self, pub_impl: &ModulePublisher, key: String) -> ::everestrs::Result<bool> {
        pub_impl.their_store_publisher.exists(key)
    }
}

impl ExampleServiceSubscriber for OneClass {
    fn uses_something(&self, pub_impl: &ModulePublisher, key: String) -> ::everestrs::Result<bool> {
        if !pub_impl.their_store_publisher.exists(key.clone())? {
            println!("IT SHOULD NOT AND DOES NOT EXIST");
        }

        let test_array = vec![1, 2, 3];
        pub_impl
            .their_store_publisher
            .store(key.clone(), test_array.clone().into())?;

        let exi = pub_impl.their_store_publisher.exists(key.clone())?;
        if exi {
            println!("IT ACTUALLY EXISTS");
        }

        let ret: Vec<i32> = serde_json::from_value(pub_impl.their_store_publisher.load(key)?)
            .expect("Wanted an array as return value");

        println!("loaded array: {ret:?}, original array: {test_array:?}");
        Ok(exi)
    }
}

impl KvsClientSubscriber for OneClass {}

impl OnReadySubscriber for OneClass {
    fn on_ready(&self, _: &ModulePublisher) {}
}

fn main() {
    let one_class = Arc::new(OneClass {});
    let _ = Module::new(
        one_class.clone(),
        one_class.clone(),
        one_class.clone(),
        one_class.clone(),
    );
    // Everest is driving execution in the background for us, nothing to do.
    loop {
        let dt = time::Duration::from_millis(250);
        thread::sleep(dt);
    }
}