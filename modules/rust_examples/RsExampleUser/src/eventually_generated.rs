use everestrs::{Error, Result, RuntimePublisher, RuntimeSubscriber, Subscriber};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// The publisher for the example module. The class is clone-able and just holds
/// a shared-ptr to the cpp implementation.
#[derive(Clone)]
pub struct ExamplePublisher {
    runtime: RuntimePublisher,
}

impl ExamplePublisher {
    /// This command checks if something is stored under a given key
    ///
    /// `key`: Key to check the existence for
    ///
    /// Returns: Returns 'True' if something was stored for this key
    pub fn uses_something(&self, key: String) -> Result<bool> {
        let args = serde_json::json!({
            "key": key,
        });
        let blob = self
            .runtime
            .call_command("their_example", "uses_something", &args);
        let return_value: bool =
            ::serde_json::from_value(blob).map_err(|_| Error::InvalidArgument("return_value"))?;
        Ok(return_value)
    }
}

/// The collection of all publishers for this module.
#[derive(Clone)]
pub struct ModulePublisher {
    pub their_publisher: ExamplePublisher,
    pub another_publisher: ExamplePublisher,
}

/// Trait for the user to implement.
pub trait ExampleSubscriber: Sync + Send {
    fn on_max_current(&self, pub_impl: &ModulePublisher, value: f64);
}

/// Trait for the user to implement.
pub trait OnReadySubscriber: Sync + Send {
    fn on_ready(&self, pub_impl: &ModulePublisher);
}

/// The struct holding everything necessary for the module to run.
///
/// If the user may drop the provided subscriber and the code will still work.
/// The user may furthermore drop the [Module] - in this case we will stop
/// receiving callbacks - however the cloned publisher will continue to work
/// until dropped.
pub struct Module {
    /// All subscribers.
    their_example: Arc<dyn ExampleSubscriber>,
    another_example: Arc<dyn ExampleSubscriber>,
    on_ready: Arc<dyn OnReadySubscriber>,

    /// The publisher.
    publisher: ModulePublisher,

    /// The handle to the subscriber runtime.
    runtime: Mutex<Option<RuntimeSubscriber>>,
}

impl Module {
    pub fn new(
        their_example: Arc<dyn ExampleSubscriber>,
        another_example: Arc<dyn ExampleSubscriber>,
        on_ready: Arc<dyn OnReadySubscriber>,
    ) -> Arc<Self> {
        let runtime = RuntimePublisher::new();
        let publisher = ModulePublisher {
            their_publisher: ExamplePublisher {
                runtime: runtime.clone(),
            },
            another_publisher: ExamplePublisher {
                runtime: runtime.clone(),
            },
        };

        let this = Arc::new(Self {
            their_example,
            another_example,
            on_ready,
            publisher: publisher,
            runtime: Mutex::new(None), // module: runtime.clone(),
        });
        let weak_this = Arc::<Module>::downgrade(&this);
        *this.runtime.lock().unwrap() = Some(RuntimeSubscriber::new(&runtime, weak_this));

        this
    }
}

impl Subscriber for Module {
    fn handle_command(
        &self,
        // rt: &Runtime,
        implementation_id: &str,
        name: &str,
        parameters: HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value> {
        todo!()
    }

    fn handle_variable(
        &self,
        implementation_id: &str,
        name: &str,
        value: serde_json::Value,
    ) -> Result<()> {
        match implementation_id {
            "their_example" => example_interface::handle_variable(
                &self.publisher,
                self.their_example.as_ref(),
                name,
                value,
            ),
            "another_example" => example_interface::handle_variable(
                &self.publisher,
                self.another_example.as_ref(),
                name,
                value,
            ),
            _ => Err(Error::InvalidArgument("Unknown variable received.")),
        }
    }

    fn on_ready(&self) {
        self.on_ready.on_ready(&self.publisher)
    }
}

mod example_interface {
    // use Runtime;
    use super::*;

    pub(super) fn handle_variable<T: super::ExampleSubscriber + ?Sized>(
        pub_impl: &super::ModulePublisher,
        their_example_subscriber: &T,
        name: &str,
        value: serde_json::Value,
    ) -> Result<()> {
        match name {
            "max_current" => {
                let v: f64 = ::serde_json::from_value(value)
                    .map_err(|_| Error::InvalidArgument("max_current"))?;
                their_example_subscriber.on_max_current(pub_impl, v);
                Ok(())
            }
            _ => Err(Error::InvalidArgument("Unknown variable received.")),
        }
    }
}
