use std::collections::HashMap;

pub trait KvsService: Sync {
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
pub trait ExampleService: Sync {
    /// This command checks if something is stored under a given key
    ///
    /// `key`: Key to check the existence for
    ///
    /// Returns: Returns 'True' if something was stored for this key*/
    fn uses_something(&self, key: String) -> ::everestrs::Result<bool>;
}

// NOCOM(#sirver): figure out how to publish something here.
// vars:
  // max_current:
    // description: Provides maximum current of this supply in ampere
    // type: number

// NOCOM(#sirver): other module
// pub trait ExampleSubscriber: Sync {
    // fn on_max_current(&self, value: f64);
// }

pub trait Module: Sync + Sized {
    fn on_ready(&self) {}
    fn foobar(&self) -> &dyn ExampleService;
    fn my_store(&self) -> &dyn KvsService;
    // NOCOM(#sirver): other service
    // fn foobar_subscriber(&self) -> &dyn ExampleSubscriber;
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
            // NOCOM(#sirver): no variables here to handle
            // "foobar" => foobar::handle_variable(self.0.foobar_subscriber(), name, value),
            _ => Err(everestrs::Error::InvalidArgument(
                "Unknown variable received.",
            )),
        }
    }

    fn on_ready(&self) {
        self.0.on_ready()
    }
}

pub fn init_from_commandline<T: Module + 'static>(specific_module: T) -> everestrs::Runtime {
    let cnt = GenericToSpecificModuleProxy(specific_module);
    everestrs::Runtime::from_commandline(cnt)
}

mod foobar {
    use std::collections::HashMap;
    // NOCOM(#sirver): in other module
    // pub(super) fn handle_variable(
        // foobar_subscriber: &dyn super::ExampleSubscriber,
        // name: &str,
        // value: serde_json::Value,
    // ) -> ::everestrs::Result<()> {
        // match name {
            // "max_current" => {
                // let v: f64 = ::serde_json::from_value(value)
                    // .map_err(|_| everestrs::Error::InvalidArgument("max_current"))?;
                // foobar_subscriber.on_max_current(v);
                // Ok(())
            // }
            // _ => Err(everestrs::Error::InvalidArgument(
                // "Unknown variable received.",
            // )),
        // }
    // }

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
