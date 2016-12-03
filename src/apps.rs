//! Constants for all known application identifiers.

use ApplicationId;

/// Defines an application identifier.
macro_rules! define_application {
    ($name:ident => $id:expr) => {
        pub fn $name() -> ApplicationId {
            ApplicationId($id.to_owned())
        }
    }
}

define_application!(youtube => "YouTube");
