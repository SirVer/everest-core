use std::collections::HashMap;

// TODO(sirver): missing error handling
pub trait KvsService: Sync {
    /// This command removes the value stored under a given key
    ///
    /// `key`: Key to delete the value for
    fn delete(&mut self, key: String) -> everestrs::Result<()>;

    /// This command checks if something is stored under a given key
    ///
    /// `key`: Key to check the existence for
    ///
    /// Returns: Returns 'True' if something was stored for this key*/
    fn exists(&mut self, key: String) -> ::everestrs::Result<bool>;

    /// This command loads the previously stored value for a given key (it will return null if the
    /// key does not exist)
    ///
    /// `key`: Key to load the value for
    ///
    /// Returns: The previously stored value
    fn load(&mut self, key: String) -> ::everestrs::Result<serde_json::Value>;

    /// This command stores a value under a given key
    ///
    /// `key`: Key to store the value for
    /// `value`: Value to store
    fn store(&mut self, key: String, value: ::serde_json::Value) -> ::everestrs::Result<()>;
}

pub trait Module: Sync + Sized {
    fn on_ready(&mut self) {}
    fn main(&mut self) -> &mut dyn KvsService;
}

/// We want the user to just implement the `Module` trait above to get access to everything that
/// EVerest has to offer, however for the `everestrs` library, we have to implement the
/// `GenericModule`. This thin wrapper does the translation between the generic module and the
/// specific implementation provided by the user.
pub struct GenericToSpecificModuleProxy<T: Module>(T);

impl<T: Module> everestrs::GenericModule for GenericToSpecificModuleProxy<T> {
    fn handle_cmd(
        &mut self,
        implementation_id: &str,
        cmd_name: &str,
        parameters: HashMap<String, serde_json::Value>,
    ) -> ::everestrs::Result<serde_json::Value> {
        match implementation_id {
            "main" => main::handle_cmd(self.0.main(), cmd_name, parameters),
            _ => Err(everestrs::Error::InvalidArgument(
                "Unknown implementation_id called.",
            )),
        }
    }

    fn on_ready(&mut self) {
        self.0.on_ready()
    }
}

pub fn init_from_commandline<T: Module>(
    specific_module: T,
) -> everestrs::Runtime<GenericToSpecificModuleProxy<T>> {
    let cnt = GenericToSpecificModuleProxy(specific_module);
    everestrs::Runtime::from_commandline(cnt)
}

mod main {
    use std::collections::HashMap;

    pub fn handle_cmd(
        main_service: &mut dyn super::KvsService,
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
