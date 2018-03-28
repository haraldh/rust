//! DO NOT EDIT
//! This file is automatically generated by the varlink rust generator

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde_json;
use std::io;
use std::sync::{Arc, RwLock};
use varlink;
use varlink::CallTrait;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct _PingReply {
    #[serde(skip_serializing_if = "Option::is_none")] pub pong: Option<String>,
}

impl varlink::VarlinkReply for _PingReply {}

#[derive(Serialize, Deserialize, Debug)]
pub struct _PingArgs {
    #[serde(skip_serializing_if = "Option::is_none")] pub ping: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct _PingErrorArgs {
    #[serde(skip_serializing_if = "Option::is_none")] pub parameter: Option<i64>,
}

pub trait _CallErr: varlink::CallTrait {
    fn reply_ping_error(&mut self, parameter: Option<i64>) -> io::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "org.example.ping.PingError".into(),
            Some(serde_json::to_value(_PingErrorArgs { parameter }).unwrap()),
        ))
    }
}

impl<'a> _CallErr for varlink::Call<'a> {}

pub enum _Error {
    PingError(_PingErrorArgs),
    VarlinkError_(varlink::Error),
    UnknownError_(varlink::Reply),
    IOError_(io::Error),
    JSONError_(serde_json::Error),
}

impl From<varlink::Reply> for _Error {
    fn from(e: varlink::Reply) -> Self {
        if varlink::Error::is_error(&e) {
            return _Error::VarlinkError_(e.into());
        }

        match e {
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "org.example.ping.PingError" =>
            {
                match e {
                    varlink::Reply {
                        parameters: Some(p),
                        ..
                    } => match serde_json::from_value(p) {
                        Ok(v) => _Error::PingError(v),
                        Err(_) => _Error::PingError(_PingErrorArgs {
                            ..Default::default()
                        }),
                    },
                    _ => _Error::PingError(_PingErrorArgs {
                        ..Default::default()
                    }),
                }
            }
            _ => return _Error::UnknownError_(e),
        }
    }
}

impl From<io::Error> for _Error {
    fn from(e: io::Error) -> Self {
        _Error::IOError_(e)
    }
}

impl From<serde_json::Error> for _Error {
    fn from(e: serde_json::Error) -> Self {
        use serde_json::error::Category;
        match e.classify() {
            Category::Io => _Error::IOError_(e.into()),
            _ => _Error::JSONError_(e),
        }
    }
}

impl From<_Error> for io::Error {
    fn from(e: _Error) -> Self {
        match e {
            _Error::PingError(e) => io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "org.example.ping.PingError: '{}'",
                    serde_json::to_string_pretty(&e).unwrap()
                ),
            ),
            _Error::VarlinkError_(e) => e.into(),
            _Error::IOError_(e) => e,
            _Error::JSONError_(e) => e.into(),
            _Error::UnknownError_(e) => io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "unknown varlink error: {}",
                    serde_json::to_string_pretty(&e).unwrap()
                ),
            ),
        }
    }
}
pub trait _CallPing: _CallErr {
    fn reply(&mut self, pong: Option<String>) -> io::Result<()> {
        self.reply_struct(_PingReply { pong }.into())
    }
}

impl<'a> _CallPing for varlink::Call<'a> {}

pub trait VarlinkInterface {
    fn ping(&self, call: &mut _CallPing, ping: Option<String>) -> io::Result<()>;
    fn call_upgraded(&self, _call: &mut varlink::Call) -> io::Result<()> {
        Ok(())
    }
}

pub trait VarlinkClientInterface {
    fn ping(
        &mut self,
        ping: Option<String>,
    ) -> io::Result<varlink::MethodCall<_PingArgs, _PingReply, _Error>>;
}

pub struct VarlinkClient {
    connection: Arc<RwLock<varlink::Connection>>,
}

impl VarlinkClient {
    pub fn new(connection: Arc<RwLock<varlink::Connection>>) -> Self {
        VarlinkClient { connection }
    }
}

impl VarlinkClientInterface for VarlinkClient {
    fn ping(
        &mut self,
        ping: Option<String>,
    ) -> io::Result<varlink::MethodCall<_PingArgs, _PingReply, _Error>> {
        varlink::MethodCall::<_PingArgs, _PingReply, _Error>::call(
            self.connection.clone(),
            "org.example.ping.Ping".into(),
            _PingArgs { ping },
        )
    }
}

pub struct _InterfaceProxy {
    inner: Box<VarlinkInterface + Send + Sync>,
}

pub fn new(inner: Box<VarlinkInterface + Send + Sync>) -> _InterfaceProxy {
    _InterfaceProxy { inner }
}

impl varlink::Interface for _InterfaceProxy {
    fn get_description(&self) -> &'static str {
        r#"
# Example service
interface org.example.ping

# Returns the same string
method Ping(ping: string) -> (pong: string)

error PingError(parameter: int)
"#
    }

    fn get_name(&self) -> &'static str {
        "org.example.ping"
    }

    fn call_upgraded(&self, call: &mut varlink::Call) -> io::Result<()> {
        self.inner.call_upgraded(call)
    }

    fn call(&self, call: &mut varlink::Call) -> io::Result<()> {
        let req = call.request.unwrap();
        match req.method.as_ref() {
            "org.example.ping.Ping" => {
                if let Some(args) = req.parameters.clone() {
                    let args: _PingArgs = serde_json::from_value(args)?;
                    return self.inner.ping(call as &mut _CallPing, args.ping);
                } else {
                    return call.reply_invalid_parameter(None);
                }
            }

            m => {
                return call.reply_method_not_found(Some(String::from(m)));
            }
        }
    }
}
