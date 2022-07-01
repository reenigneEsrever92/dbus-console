use std::{str::FromStr, sync::Arc};

use zbus::{
    blocking::{Connection, Proxy},
    dbus_proxy,
    xml::Node,
    Message, Result,
};
use zvariant::Signature;

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

        Node::from_str(&proxy.introspect().unwrap()).unwrap()
    }

    pub fn get_signature(
        &self,
        service: &str,
        path: &str,
        interface: &str,
        method: &str,
    ) -> Option<String> {
        let proxy = Proxy::new(&self.con, service, path, interface).unwrap();

        let node = self.introspect(service, path);

        node.interfaces()
            .iter()
            .find(|inf| inf.name() == interface)
            .and_then(|inf| {
                inf.methods()
                    .iter()
                    .find(|mth| mth.name() == method)
                    .cloned()
            })
            .map(|mth| {
                mth.args()
                    .iter()
                    .map(|arg| arg.ty())
                    .fold(String::new(), |a, b| a + b)
            })
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

    #[test]
    fn test_get_signature() {
        let dbus_client = DBusClient::default();
        let result = dbus_client.get_signature(
            "org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            "org.freedesktop.DBus",
            "ListNames",
        );

        println!("{:?}", result);
    }
}
