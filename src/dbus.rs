use std::{sync::Arc, str::FromStr};

use zbus::{blocking::{Connection, Proxy}, xml::Node, dbus_proxy, Message, Result};
use zvariant::ObjectPath;

use crate::AppEvent;

#[dbus_proxy(
    interface = "org.freedesktop.DBus",
    default_service = "org.freedesktop.DBus",
    default_path = "/org/freedesktop/DBus"
)]
trait FreeDesktopDBus {
    fn list_names(&self) -> Result<Vec<String>>;
}

#[dbus_proxy(
    interface = "org.freedesktop.DBus.Introspectable",
    default_service = "org.freedesktop.DBus",
    default_path = "/"
)]
trait Introspect {
    fn introspect(&self) -> Result<String>;
}

struct DBusClient {
    con: Connection, 
}

impl DBusClient {
    fn new(con: Connection) -> Self {
        Self { con }
    }

    fn introspect(&self, path: &str, interface: &str) -> Node {
        let proxy = Proxy::new(&self.con, "org.freedesktop.DBus.Introspectable", path, interface).unwrap();
        Node::from_str(proxy.call_method("Introspect", &()).unwrap().body().unwrap()).unwrap()
    }
}

pub fn load_bus_names() -> Vec<AppEvent> {
    let con = Connection::session().unwrap();

    let proxy = FreeDesktopDBusProxyBlocking::new(&con).unwrap();

    vec![AppEvent::BusNames(proxy.list_names().unwrap())]
}

fn message_to_app_event(message: &Arc<Message>) -> AppEvent {
    match message.message_type() {
        zbus::MessageType::Invalid => AppEvent::InvalidMessage,
        zbus::MessageType::MethodCall => AppEvent::MethodCall,
        zbus::MessageType::MethodReturn => AppEvent::MethodResponse,
        zbus::MessageType::Error => AppEvent::DBusError,
        zbus::MessageType::Signal => todo!(),
    }
}

#[cfg(test)]
mod test {
    use super::{load_bus_names};


    #[test]
    fn test_get_object_path() {
        let app_event = load_bus_names();

        assert_eq!(app_event.len(), 1);
    }
}
