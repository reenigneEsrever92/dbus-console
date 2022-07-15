use std::{str::FromStr, sync::Arc};

use zbus::{
    blocking::{Connection, Proxy},
    xml::Node,
    Message,
};

use crate::error::DBusConsoleResult;

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

    pub fn introspect(&self, service: &str, path: &str) -> DBusConsoleResult<Node> {
        let proxy = Proxy::new(
            &self.con,
            service,
            path,
            "org.freedesktop.DBus.Introspectable",
        )
        .unwrap();

        Ok(Node::from_str(&proxy.introspect()?)?)
    }

    pub fn get_paths(&self, service: &str) -> DBusConsoleResult<Vec<String>> {
        self.do_get_paths(service, "/")
    }

    fn do_get_paths(&self, service: &str, path: &str) -> DBusConsoleResult<Vec<String>> {
        self.introspect(service, path)?.nodes().iter_mut().fold(
            Ok(Vec::new()),
            |mut paths, node| {
                let mut full_name = String::from(path);

                if !full_name.ends_with("/") {
                    full_name.push('/')
                }

                full_name.push_str(node.name().unwrap());

                match paths {
                    Ok(mut vec) => {
                        vec.push(full_name.clone());
                        match self.do_get_paths(service, &full_name) {
                            Ok(mut rec_paths) => {
                                vec.append(&mut rec_paths);
                                Ok(vec)
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Err(e) => Err(e),
                }
            },
        )
    }

    pub fn get_methods(&self, service: &str, path: &str) -> DBusConsoleResult<Vec<String>> {
        Ok(self
            .introspect(service, path)?
            .interfaces()
            .iter()
            .map(|interface| interface.name().to_string())
            .collect())
    }

    pub fn get_signature(
        &self,
        service: &str,
        path: &str,
        interface: &str,
        method: &str,
    ) -> DBusConsoleResult<Option<String>> {
        let node = self.introspect(service, path)?;

        Ok(node
            .interfaces()
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
                    .filter(|arg| arg.direction() == Some("in"))
                    .map(|arg| arg.ty())
                    .fold(String::new(), |a, b| a + b)
            }))
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
        let result = dbus_client
            .introspect(
                "org.freedesktop.Notifications",
                "/org/freedesktop/Notifications",
            )
            .unwrap();

        assert!(result.interfaces().len() > 0);
    }

    #[test]
    fn test_get_paths() {
        let dbus_client = DBusClient::default();
        let result = dbus_client.get_paths("org.freedesktop.Notifications");
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
        let result = dbus_client
            .get_signature(
                "org.freedesktop.Notifications",
                "/org/freedesktop/Notifications",
                "org.freedesktop.Notifications",
                "Notify",
            )
            .unwrap();

        assert_eq!(result, Some(String::from("susssasa{sv}i")));
    }
}
