use std::sync::{Arc, Mutex};
use std::{thread, time};

mod eventually_generated;

struct ExampleClient {
    max_current: Mutex<Option<f64>>,
    thread: Mutex<Option<thread::JoinHandle<()>>>,
}

impl ExampleClient {
    fn new() -> Self {
        Self {
            max_current: Mutex::new(None),
            thread: Mutex::new(None),
        }
    }
}

impl eventually_generated::ExampleSubscriber for ExampleClient {
    fn on_max_current(&self, pub_impl: &eventually_generated::ModulePublisher, value: f64) {
        println!("Received the value {value}");
        let _ = pub_impl
            .their_publisher
            .uses_something("hello_there".to_string());
        *self.max_current.lock().unwrap() = Some(value);

        // Example where we start a thread with the publisher. The cloning is
        // only done if the user wants to offload the publisher into a thread.
        let clone = pub_impl.clone();
        *self.thread.lock().unwrap() = Some(thread::spawn(move || {
            let _ = clone.another_publisher.uses_something("foo".to_string());
        }))
    }
}

struct Module {
    their_example: Arc<ExampleClient>,
    another_example: Arc<ExampleClient>,
    min_current: Mutex<Option<f64>>,
}

impl eventually_generated::OnReadySubscriber for Module {
    fn on_ready(&self, _pub_impl: &eventually_generated::ModulePublisher) {
        let mut their_current = self.their_example.max_current.lock().unwrap();
        let mut another_current = self.another_example.max_current.lock().unwrap();
        *their_current = Some(1.);
        *another_current = Some(2.);
        // uses somehow both...
        *self.min_current.lock().unwrap() = Some(1.);
    }
}

fn main() {
    let their_example = Arc::new(ExampleClient::new());
    let another_example = Arc::new(ExampleClient::new());
    let module = Arc::new(Module {
        their_example: their_example.clone(),
        another_example: another_example.clone(),
        min_current: Mutex::new(None),
    });
    let _ = eventually_generated::Module::new(
        their_example.clone(),
        another_example.clone(),
        module.clone(),
    );
}
