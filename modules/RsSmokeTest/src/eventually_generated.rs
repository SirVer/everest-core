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

pub trait ExampleSubscriber: Sync {
    fn on_max_current(&self, value: f64);
}

pub trait Module: Sync + Sized {
    fn on_ready(&self) {}
    fn main(&self) -> &dyn KvsService;
    fn foobar_subscriber(&self) -> &dyn ExampleSubscriber;
}

/// We want the user to just implement the `Module` trait above to get access to everything that
/// EVerest has to offer, however for the `everestrs` library, we have to implement the
/// `GenericModule`. This thin wrapper does the translation between the generic module and the
/// specific implementation provided by the user.
pub struct GenericToSpecificModuleProxy<T: Module>(T);

impl<T: Module> everestrs::GenericModule for GenericToSpecificModuleProxy<T> {
    fn handle_command(
        &self,
        implementation_id: &str,
        cmd_name: &str,
        parameters: HashMap<String, serde_json::Value>,
    ) -> ::everestrs::Result<serde_json::Value> {
        match implementation_id {
            "main" => main::handle_command(self.0.main(), cmd_name, parameters),
            _ => Err(everestrs::Error::InvalidArgument(
                "Unknown implementation_id called.",
            )),
        }
    }

    fn handle_variable(
        &self,
        implementation_id: &str,
        name: &str,
        value: serde_json::Value,
    ) -> ::everestrs::Result<()> {
        match implementation_id {
            "foobar" => foobar::handle_variable(self.0.foobar_subscriber(), name, value),
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
    pub(super) fn handle_variable(
        foobar_subscriber: &dyn super::ExampleSubscriber,
        name: &str,
        value: serde_json::Value,
    ) -> ::everestrs::Result<()> {
        match name {
            "max_current" => {
                let v: f64 = ::serde_json::from_value(value)
                    .map_err(|_| everestrs::Error::InvalidArgument("max_current"))?;
                foobar_subscriber.on_max_current(v);
                Ok(())
            }
            _ => Err(everestrs::Error::InvalidArgument(
                "Unknown variable received.",
            )),
        }
    }
}

mod main {
    use std::collections::HashMap;

    pub(super) fn handle_command(
        main_service: &dyn super::KvsService,
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
                let retval = main_service.delete(key)?;
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
                let retval = main_service.exists(key)?;
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
                let retval = main_service.load(key)?;
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
                let retval = main_service.store(key, value)?;
                Ok(retval.into())
            }
            _ => Err(everestrs::Error::InvalidArgument("Unknown command called.")),
        }
    }
}
