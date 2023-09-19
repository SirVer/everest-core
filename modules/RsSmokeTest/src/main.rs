use std::collections::BTreeMap;
use std::sync::RwLock;
use std::{thread, time};

mod eventually_generated;

struct Kvs {
    values: RwLock<BTreeMap<String, serde_json::Value>>,
}

impl eventually_generated::KvsService for Kvs {
    fn store(&self, key: String, value: serde_json::Value) -> ::everestrs::Result<()> {
        let mut v = self.values.write().expect("should never be poisoned.");
        v.insert(key, value);
        Ok(())
    }

    fn load(&self, key: String) -> ::everestrs::Result<serde_json::Value> {
        let v = self.values.read().expect("should never be poisoned.");
        Ok(v.get(&key).cloned().unwrap_or(serde_json::Value::Null))
    }

    fn delete(&self, key: String) -> ::everestrs::Result<()> {
        let mut v = self.values.write().expect("should never be poisoned.");
        v.remove(&key);
        Ok(())
    }

    fn exists(&self, key: String) -> ::everestrs::Result<bool> {
        let v = self.values.read().expect("should never be poisoned.");
        Ok(v.contains_key(&key))
    }
}

pub struct Module {
    kvs: Kvs,
}

impl eventually_generated::Module for Module {
    fn main(&self) -> &dyn eventually_generated::KvsService {
        &self.kvs
    }

    fn on_ready(&self) {
        println!("Welcome to the RsSmokeTest module!");
    }
}

fn main() {
    let module = Module {
        kvs: Kvs {
            values: RwLock::new(BTreeMap::new()),
        },
    };
    let _mod = eventually_generated::init_from_commandline(module);

    // Everest is driving execution in the background for us, nothing to do.
    loop {
        let dt = time::Duration::from_millis(250);
        thread::sleep(dt);
    }
}
