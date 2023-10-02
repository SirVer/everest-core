use std::collections::HashMap;

pub trait KvsService {
    /// This command removes the value stored under a given key
    ///
    /// `key`: Key to delete the value for
    fn delete(&self, key: String) -> everestrs::Result<()>;

    /// This command checks if something is stored under a given key
    ///
    /// `key`: Key to check the existence for
    ///
    /// Returns: Returns 'True' if something was stored for this key*/
    fn exists(&self, key: String) -> ::everestrs::Result<bool>;

    /// This command loads the previously stored value for a given key (it will return null if the
    /// key does not exist)
    ///
    /// `key`: Key to load the value for
    ///
    /// Returns: The previously stored value
    fn load(&self, key: String) -> ::everestrs::Result<serde_json::Value>;

    /// This command stores a value under a given key
    ///
    /// `key`: Key to store the value for
    /// `value`: Value to store
    fn store(&self, key: String, value: ::serde_json::Value) -> ::everestrs::Result<()>;
}

/// This interface defines an example interface that uses multiple framework features
pub trait ExampleService {
    /// This command checks if something is stored under a given key
    ///
    /// `key`: Key to check the existence for
    ///
    /// Returns: Returns 'True' if something was stored for this key*/
    fn uses_something(&self, key: String) -> ::everestrs::Result<bool>;
}

pub struct FoobarPublisher {
    raw_publisher: everestrs::RawPublisher,
}

impl FoobarPublisher {
    pub fn publish_max_current(&self, max_current: f64) -> ::everestrs::Result<()> {
        self.raw_publisher
            .publish_variable("foobar", "max_current", &max_current);
        Ok(())
    }
}

/// This interface defines a simple key-value-store interface
pub struct KvsClient {
    raw_publisher: everestrs::RawPublisher,
}

impl KvsClient {
    /// This command removes the value stored under a given key
    ///
    /// `key`: Key to delete the value for
    pub fn delete(&self, key: String) -> everestrs::Result<()> {
        let args = serde_json::json!({
            "key": key,
        });
        let blob = self
            .raw_publisher
            .call_command("their_store", "delete", &args);
        let return_value: () = ::serde_json::from_value(blob)
            .map_err(|_| everestrs::Error::InvalidArgument("return_value"))?;
        Ok(return_value)
    }

    /// This command checks if something is stored under a given key
    ///
    /// `key`: Key to check the existence for
    ///
    /// Returns: Returns 'True' if something was stored for this key*/
    pub fn exists(&self, key: String) -> ::everestrs::Result<bool> {
        // NOCOM(#sirver): Implement
        todo!();
    }

    /// This command loads the previously stored value for a given key (it will return null if the
    /// key does not exist)
    ///
    /// `key`: Key to load the value for
    ///
    /// Returns: The previously stored value
    pub fn load(&self, key: String) -> ::everestrs::Result<serde_json::Value> {
        // NOCOM(#sirver): Implement
        todo!();
    }

    /// This command stores a value under a given key
    ///
    /// `key`: Key to store the value for
    /// `value`: Value to store
    pub fn store(&self, key: String, value: ::serde_json::Value) -> ::everestrs::Result<()> {
        // NOCOM(#sirver): Implement
        todo!();
    }
}

pub trait Module: Sized {
    fn on_ready(&self) {}
    fn foobar(&self) -> &dyn ExampleService;
    fn my_store(&self) -> &dyn KvsService;
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
            "foobar" => foobar::handle_command(self.0.foobar(), cmd_name, parameters),
            "my_store" => my_store::handle_command(self.0.my_store(), cmd_name, parameters),
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
    init_module: impl FnOnce(FoobarPublisher, KvsClient) -> T,
) -> everestrs::Runtime {
    everestrs::Runtime::from_commandline(|raw_publisher| {
        let foobar_publisher = FoobarPublisher {
            raw_publisher: raw_publisher.clone(),
        };
        let their_store_client = KvsClient { raw_publisher };
        let specific_module = init_module(foobar_publisher, their_store_client);
        GenericToSpecificModuleProxy(specific_module)
    })
}

mod foobar {
    use std::collections::HashMap;

    #[allow(unused_variables)]
    pub(super) fn handle_command(
        foobar_service: &dyn super::ExampleService,
        cmd_name: &str,
        mut parameters: HashMap<String, serde_json::Value>,
    ) -> ::everestrs::Result<serde_json::Value> {
        match cmd_name {
            "uses_something" => {
                let key: String = ::serde_json::from_value(
                    parameters
                        .remove("key")
                        .ok_or(everestrs::Error::MissingArgument("key"))?,
                )
                .map_err(|_| everestrs::Error::InvalidArgument("key"))?;
                #[allow(clippy::let_unit_value)]
                let retval = foobar_service.uses_something(key)?;
                Ok(retval.into())
            }
            _ => Err(everestrs::Error::InvalidArgument("Unknown command called.")),
        }
    }
}

mod my_store {
    use std::collections::HashMap;

    pub(super) fn handle_command(
        my_store_service: &dyn super::KvsService,
        cmd_name: &str,
        mut parameters: HashMap<String, serde_json::Value>,
    ) -> ::everestrs::Result<serde_json::Value> {
        match cmd_name {
            "delete" => {
                let key: String = ::serde_json::from_value(
                    parameters
                        .remove("key")
                        .ok_or(everestrs::Error::MissingArgument("key"))?,
                )
                .map_err(|_| everestrs::Error::InvalidArgument("key"))?;
                #[allow(clippy::let_unit_value)]
                let retval = my_store_service.delete(key)?;
                Ok(retval.into())
            }
            "exists" => {
                let key: String = ::serde_json::from_value(
                    parameters
                        .remove("key")
                        .ok_or(everestrs::Error::MissingArgument("key"))?,
                )
                .map_err(|_| everestrs::Error::InvalidArgument("key"))?;
                #[allow(clippy::let_unit_value)]
                let retval = my_store_service.exists(key)?;
                Ok(retval.into())
            }
            "load" => {
                let key: String = ::serde_json::from_value(
                    parameters
                        .remove("key")
                        .ok_or(everestrs::Error::MissingArgument("key"))?,
                )
                .map_err(|_| everestrs::Error::InvalidArgument("key"))?;
                #[allow(clippy::let_unit_value)]
                let retval = my_store_service.load(key)?;
                Ok(retval.into())
            }
            "store" => {
                let key: String = ::serde_json::from_value(
                    parameters
                        .remove("key")
                        .ok_or(everestrs::Error::MissingArgument("key"))?,
                )
                .map_err(|_| everestrs::Error::InvalidArgument("key"))?;
                let value: ::serde_json::Value = ::serde_json::from_value(
                    parameters
                        .remove("value")
                        .ok_or(everestrs::Error::MissingArgument("value"))?,
                )
                .map_err(|_| everestrs::Error::InvalidArgument("value"))?;
                #[allow(clippy::let_unit_value)]
                let retval = my_store_service.store(key, value)?;
                Ok(retval.into())
            }
            _ => Err(everestrs::Error::InvalidArgument("Unknown command called.")),
        }
    }
}
