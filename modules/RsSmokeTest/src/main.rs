use std::collections::BTreeMap;
use std::{thread, time};

mod eventually_generated;

struct Kvs {
    values: BTreeMap<String, serde_json::Value>,
}

impl eventually_generated::KvsService for Kvs {
    fn store(&mut self, key: String, value: serde_json::Value) -> ::everestrs::Result<()> {
        self.values.insert(key, value);
        Ok(())
    }

    fn load(&mut self, key: String) -> ::everestrs::Result<serde_json::Value> {
        Ok(self
            .values
            .get(&key)
            .cloned()
            .unwrap_or(serde_json::Value::Null))
    }

    fn delete(&mut self, key: String) -> ::everestrs::Result<()> {
        self.values.remove(&key);
        Ok(())
    }

    fn exists(&mut self, key: String) -> ::everestrs::Result<bool> {
        Ok(self.values.contains_key(&key))
    }
}

pub struct Module {
    kvs: Kvs,
}

impl eventually_generated::Module for Module {
    fn main(&mut self) -> &mut dyn eventually_generated::KvsService {
        &mut self.kvs
    }

    fn on_ready(&mut self) {
        println!("Welcome to the RsSmokeTest module!");
    }
}

fn main() {
    let module = Module {
        kvs: Kvs {
            values: BTreeMap::new(),
        },
    };
    let _mod = eventually_generated::init_from_commandline(module);

    // Everest is driving execution in the background for us, nothing to do.
    loop {
        let dt = time::Duration::from_millis(250);
        thread::sleep(dt);
    }
}
