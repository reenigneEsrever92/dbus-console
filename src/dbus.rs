use std::{str::FromStr, sync::Arc};

use zbus::{
    blocking::{Connection, Proxy},
    dbus_proxy,
    xml::Node,
    Message, Result,
};

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

pub struct DBusClient {
    con: Connection,
}

impl Default for DBusClient {
    fn default() -> Self {
        Self {
            con: Connection::session().unwrap(),
        }
    }
}

impl DBusClient {
    pub fn new(con: Connection) -> Self {
        Self { con }
    }

    pub fn list_names(&self) -> Vec<String> {
        let proxy = Proxy::new(
            &self.con,
            "org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            "org.freedesktop.DBus",
        )
        .unwrap();

        proxy.call_method("ListNames", &()).unwrap().body().unwrap()
    }

    pub fn introspect(&self, service: &str, path: &str) -> Node {
        let proxy = Proxy::new(
            &self.con,
            service,
            path,
            "org.freedesktop.DBus.Introspectable",
        )
        .unwrap();

        Node::from_str(
            proxy
                .call_method("Introspect", &())
                .unwrap()
                .body()
                .unwrap(),
        )
        .unwrap()
    }

    pub fn call_function<T>(
        &self,
        service: &str,
        path: &str,
        interface: &str,
        method: &str,
        args: &T,
    ) -> Arc<Message>
    where
        T: serde::ser::Serialize + zvariant::DynamicType,
    {
        let proxy = Proxy::new(&self.con, service, path, interface).unwrap();

        proxy.call_method(method, args).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::dbus::DBusClient;

    #[test]
    fn test_list_names() {
        let dbus_client = DBusClient::default();
        assert!(dbus_client.list_names().len() > 1);
    }

    #[test]
    fn test_instrospect() {
        let dbus_client = DBusClient::default();
        assert!(
            dbus_client
                .introspect("org.freedesktop.DBus", "/")
                .nodes()
                .len()
                > 0
        );
    }

    #[test]
    fn test_call() {
        let dbus_client = DBusClient::default();
        assert!(dbus_client
            .call_function(
                "org.freedesktop.DBus",
                "/org/freedesktop/DBus",
                "org.freedesktop.DBus",
                "ListNames",
                &()
            )
            .body::<Vec<String>>()
            .is_ok());
    }
}
