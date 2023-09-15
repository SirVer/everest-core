use std::collections::BTreeMap;
use std::env;
use std::path::PathBuf;
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

fn main() {
    let kvs = Kvs {
        values: BTreeMap::new(),
    };
    let module = eventually_generated::Module::new(kvs);

    // Everest is driving execution in the background for us, nothing to do.
    loop {
    let dt = time::Duration::from_secs(120);
    thread::sleep(dt);
    }
}
