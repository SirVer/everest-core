use everestrs::{Publisher, Subscriber};
use std::collections::HashMap;
use std::sync::Arc;

/// The publisher for the example module.
pub struct ExamplePublisher<'a> {
    runtime: &'a everestrs::Runtime,
    name: &'a str,
}

impl<'a> ExamplePublisher<'a> {
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
            .runtime
            .call_command("their_example", "uses_something", &args);
        let return_value: bool = ::serde_json::from_value(blob)
            .map_err(|_| everestrs::Error::InvalidArgument("return_value"))?;
        Ok(return_value)
    }
}

pub trait ExampleSubscriber: Sync + Send {
    fn on_max_current(&self, pub_impl: &ExamplePublisher, value: f64);
}

pub trait OnReadySubscriber: Sync + Send {
    fn on_ready(&self);
}

/// The struct holding the subscriber implementation.
struct ModuleSubscriber {
    /// The server subscribers.
    their_example: Arc<dyn ExampleSubscriber>,
    another_example: Arc<dyn ExampleSubscriber>,

    /// The on-ready subscriber.
    on_ready: Arc<dyn OnReadySubscriber>,
}

impl Subscriber for ModuleSubscriber {
    fn handle_command(
        &self,
        rt: &everestrs::Runtime,
        implementation_id: &str,
        name: &str,
        parameters: HashMap<String, serde_json::Value>,
    ) -> everestrs::Result<serde_json::Value> {
        todo!()
    }

    fn handle_variable(
        &self,
        rt: &everestrs::Runtime,
        implementation_id: &str,
        name: &str,
        value: serde_json::Value,
    ) -> everestrs::Result<()> {
        match implementation_id {
            "their_example" => {
                let pub_impl = ExamplePublisher {
                    runtime: rt,
                    name: "their_example",
                };
                example_interface::handle_variable(
                    &pub_impl,
                    self.their_example.as_ref(),
                    name,
                    value,
                )
            }
            "another_example" => {
                let pub_impl = ExamplePublisher {
                    runtime: rt,
                    name: "another_example",
                };
                example_interface::handle_variable(
                    &pub_impl,
                    self.another_example.as_ref(),
                    name,
                    value,
                )
            }
            _ => Err(everestrs::Error::InvalidArgument(
                "Unknown variable received.",
            )),
        }
    }

    fn on_ready(&self) {
        todo!()
    }
}

pub struct Module {
    mod_impl: everestrs::Runtime,
}

impl Module {
    pub fn new(
        their_example: Arc<dyn ExampleSubscriber>,
        another_example: Arc<dyn ExampleSubscriber>,
        on_ready: Arc<dyn OnReadySubscriber>,
    ) -> Self {
        let sub_impl = Box::new(ModuleSubscriber {
            their_example,
            another_example,
            on_ready,
        });

        Self {
            mod_impl: everestrs::Runtime::from_commandline(sub_impl),
        }
    }
}

// pub trait Module: Sized {
//     fn on_ready(&self) {}
//     fn main(&self) -> &dyn ExampleUserService;
//     fn their_example_subscriber(&self) -> &dyn ExampleSubscriber;
// }

// We want the user to just implement the `Module` trait above to get access to everything that
// EVerest has to offer, however for the `everestrs` library, we have to implement the
// `GenericModule`. This thin wrapper does the translation between the generic module and the
// specific implementation provided by the user.
// pub struct GenericToSpecificModuleProxy<T: Module>(T);

// impl<T: Module> everestrs::GenericModule for GenericToSpecificModuleProxy<T> {
//     #[allow(unused_variables)]
//     fn handle_command(
//         &self,
//         implementation_id: &str,
//         cmd_name: &str,
//         parameters: HashMap<String, serde_json::Value>,
//     ) -> ::everestrs::Result<serde_json::Value> {
//         match implementation_id {
//             _ => Err(everestrs::Error::InvalidArgument(
//                 "Unknown implementation_id called.",
//             )),
//         }
//     }

//     #[allow(unused_variables)]
//     fn handle_variable(
//         &self,
//         implementation_id: &str,
//         name: &str,
//         value: serde_json::Value,
//     ) -> ::everestrs::Result<()> {
//         match implementation_id {
//             "their_example" => {
//                 their_example::handle_variable(self.0.their_example_subscriber(), name, value)
//             }
//             _ => Err(everestrs::Error::InvalidArgument(
//                 "Unknown variable received.",
//             )),
//         }
//     }

//     fn on_ready(&self) {
//         self.0.on_ready()
//     }
// }

// pub fn init_from_commandline<T: Module + 'static>(
//     init_module: impl FnOnce(ExampleClient) -> T,
// ) -> everestrs::Runtime {
//     everestrs::Runtime::from_commandline(|raw_publisher| {
//         let their_example_client = ExampleClient { raw_publisher };
//         let specific_module = init_module(their_example_client);
//         GenericToSpecificModuleProxy(specific_module)
//     })
// }

mod example_interface {
    // use everestrs::Runtime;

    pub(super) fn handle_variable<T: super::ExampleSubscriber + ?Sized>(
        pub_impl: &super::ExamplePublisher,
        their_example_subscriber: &T,
        name: &str,
        value: serde_json::Value,
    ) -> ::everestrs::Result<()> {
        match name {
            "max_current" => {
                let v: f64 = ::serde_json::from_value(value)
                    .map_err(|_| everestrs::Error::InvalidArgument("max_current"))?;
                their_example_subscriber.on_max_current(pub_impl, v);
                Ok(())
            }
            _ => Err(everestrs::Error::InvalidArgument(
                "Unknown variable received.",
            )),
        }
    }
}
