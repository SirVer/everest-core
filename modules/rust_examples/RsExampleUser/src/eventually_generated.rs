use std::collections::HashMap;

/// This interface defines an example_user interface that uses the example interface
pub trait ExampleUserService {}

pub trait ExampleSubscriber {
    fn on_max_current(&self, value: f64);
}

/// This interface defines an example interface that uses multiple framework features
pub struct ExampleClient {
    raw_publisher: everestrs::RawPublisher,
}

impl ExampleClient {
    /// This command checks if something is stored under a given key
    ///
    /// `key`: Key to check the existence for
    ///
    /// Returns: Returns 'True' if something was stored for this key*/
    pub fn uses_something(&self, key: String) -> ::everestrs::Result<bool> {
        let args = serde_json::json!({
            "key": key,
        });
        let blob = self
            .raw_publisher
            .call_command("their_example", "uses_something", &args);
        let return_value: bool = ::serde_json::from_value(blob)
            .map_err(|_| everestrs::Error::InvalidArgument("return_value"))?;
        Ok(return_value)
    }
}

pub trait Module: Sized {
    fn on_ready(&self) {}
    fn main(&self) -> &dyn ExampleUserService;
    fn their_example_subscriber(&self) -> &dyn ExampleSubscriber;
}

/// We want the user to just implement the `Module` trait above to get access to everything that
/// EVerest has to offer, however for the `everestrs` library, we have to implement the
/// `GenericModule`. This thin wrapper does the translation between the generic module and the
/// specific implementation provided by the user.
pub struct GenericToSpecificModuleProxy<T: Module>(T);

impl<T: Module> everestrs::GenericModule for GenericToSpecificModuleProxy<T> {
    #[allow(unused_variables)]
    fn handle_command(
        &self,
        implementation_id: &str,
        cmd_name: &str,
        parameters: HashMap<String, serde_json::Value>,
    ) -> ::everestrs::Result<serde_json::Value> {
        match implementation_id {
            _ => Err(everestrs::Error::InvalidArgument(
                "Unknown implementation_id called.",
            )),
        }
    }

    #[allow(unused_variables)]
    fn handle_variable(
        &self,
        implementation_id: &str,
        name: &str,
        value: serde_json::Value,
    ) -> ::everestrs::Result<()> {
        match implementation_id {
            "their_example" => {
                their_example::handle_variable(self.0.their_example_subscriber(), name, value)
            }
            _ => Err(everestrs::Error::InvalidArgument(
                "Unknown variable received.",
            )),
        }
    }

    fn on_ready(&self) {
        self.0.on_ready()
    }
}

pub fn init_from_commandline<T: Module + 'static>(
    init_module: impl FnOnce(ExampleClient) -> T,
) -> everestrs::Runtime {
    everestrs::Runtime::from_commandline(|raw_publisher| {
        let their_example_client = ExampleClient { raw_publisher };
        let specific_module = init_module(their_example_client);
        GenericToSpecificModuleProxy(specific_module)
    })
}

mod their_example {
    pub(super) fn handle_variable(
        their_example_subscriber: &dyn super::ExampleSubscriber,
        name: &str,
        value: serde_json::Value,
    ) -> ::everestrs::Result<()> {
        match name {
            "max_current" => {
                let v: f64 = ::serde_json::from_value(value)
                    .map_err(|_| everestrs::Error::InvalidArgument("max_current"))?;
                their_example_subscriber.on_max_current(v);
                Ok(())
            }
            _ => Err(everestrs::Error::InvalidArgument(
                "Unknown variable received.",
            )),
        }
    }
}
