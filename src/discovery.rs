//! Utilities for performing mDNS discovery of Cast devices.

use Error;
use mdns;

use std::net::Ipv4Addr;
use std::time::Duration;

use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceInfo
{
    pub ip_addr: Ipv4Addr,
    pub uuid: Uuid,
}

/// Performs Cast discovery.
pub fn run<F>(duration: Duration, mut f: F) -> Result<(), Error>
    where F: FnMut(DeviceInfo)  {
    mdns::discover("_googlecast._tcp.local", Some(duration), |response| {
        if response.records().next().is_none() { return };

        let mut address = None;
        let mut uuid_str = None;

        for record in response.records() {
            match record.kind {
                mdns::RecordKind::A(ref addr) => {
                    address = Some(addr.clone());
                    uuid_str = Some(record.name.replace(".local", ""));
                },
                _ => (),
            }
        }

        let uuid = uuid_str.unwrap().parse().expect("invalid device UUID");

        f(DeviceInfo {
            ip_addr: address.unwrap(),
            uuid: uuid,
        })

    })?;
    Ok(())
}
