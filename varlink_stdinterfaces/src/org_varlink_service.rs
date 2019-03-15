#![doc = "This file was automatically generated by the varlink rust generator"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::io::BufRead;
use std::sync::{Arc, RwLock};

use serde_derive::{Deserialize, Serialize};
use serde_json;

use varlink::{self, CallTrait};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct InterfaceNotFound_Args {
    pub r#interface: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct InvalidParameter_Args {
    pub r#parameter: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MethodNotFound_Args {
    pub r#method: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MethodNotImplemented_Args {
    pub r#method: String,
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum ErrorKind {
    Io_Error(::std::io::ErrorKind),
    SerdeJson_Error(serde_json::error::Category),
    Varlink_Error,
    VarlinkReply_Error(varlink::Reply),
    Generic,
    InterfaceNotFound(Option<InterfaceNotFound_Args>),
    InvalidParameter(Option<InvalidParameter_Args>),
    MethodNotFound(Option<MethodNotFound_Args>),
    MethodNotImplemented(Option<MethodNotImplemented_Args>),
}

impl ::std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            ErrorKind::Io_Error(_) => write!(f, "IO error"),
            ErrorKind::SerdeJson_Error(_) => write!(f, "(De)Serialization Error"),
            ErrorKind::Varlink_Error => write!(f, "Varlink Error"),
            ErrorKind::VarlinkReply_Error(v) => write!(f, "Unknown error reply: '{:#?}'", v),
            ErrorKind::Generic => Ok(()),
            ErrorKind::InterfaceNotFound(v) => {
                write!(f, "org.varlink.service.InterfaceNotFound: {:#?}", v)
            }
            ErrorKind::InvalidParameter(v) => {
                write!(f, "org.varlink.service.InvalidParameter: {:#?}", v)
            }
            ErrorKind::MethodNotFound(v) => {
                write!(f, "org.varlink.service.MethodNotFound: {:#?}", v)
            }
            ErrorKind::MethodNotImplemented(v) => {
                write!(f, "org.varlink.service.MethodNotImplemented: {:#?}", v)
            }
        }
    }
}

impl From<&std::io::Error> for ErrorKind {
    fn from(e: &std::io::Error) -> Self {
        ErrorKind::Io_Error(e.kind())
    }
}

impl From<&serde_json::error::Error> for ErrorKind {
    fn from(e: &serde_json::error::Error) -> Self {
        ErrorKind::SerdeJson_Error(e.classify())
    }
}

impl From<varlink::ErrorKind> for ErrorKind {
    fn from(_e: varlink::ErrorKind) -> Self {
        ErrorKind::Varlink_Error
    }
}

pub struct Error(
    pub ErrorKind,
    pub Option<Box<dyn std::error::Error + 'static>>,
    pub Option<&'static str>,
);

impl Error {
    #[allow(dead_code)]
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

impl From<ErrorKind> for Error {
    fn from(e: ErrorKind) -> Self {
        Error(e, None, None)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.1.as_ref().map(|e| e.as_ref())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error as StdError;
        if let Some(ref o) = self.2 {
            std::fmt::Display::fmt(o, f)?;
        }
        std::fmt::Debug::fmt(&self.0, f)?;
        if let Some(e) = self.source() {
            std::fmt::Display::fmt("\nCaused by:\n", f)?;
            std::fmt::Debug::fmt(&e, f)?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;

impl From<varlink::Error> for Error {
    fn from(_e: varlink::Error) -> Self {
        ErrorKind::Varlink_Error.into()
    }
}

impl From<varlink::Reply> for ErrorKind {
    #[allow(unused_variables)]
    fn from(e: varlink::Reply) -> Self {
        if varlink::ErrorKind::is_error(&e) {
            return varlink::ErrorKind::from(e).into();
        }
        match e {
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "org.varlink.service.InterfaceNotFound" => match e {
                varlink::Reply {
                    parameters: Some(p),
                    ..
                } => match serde_json::from_value(p) {
                    Ok(v) => ErrorKind::InterfaceNotFound(v),
                    Err(_) => ErrorKind::InterfaceNotFound(None),
                },
                _ => ErrorKind::InterfaceNotFound(None),
            },
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "org.varlink.service.InvalidParameter" => match e {
                varlink::Reply {
                    parameters: Some(p),
                    ..
                } => match serde_json::from_value(p) {
                    Ok(v) => ErrorKind::InvalidParameter(v),
                    Err(_) => ErrorKind::InvalidParameter(None),
                },
                _ => ErrorKind::InvalidParameter(None),
            },
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "org.varlink.service.MethodNotFound" => match e {
                varlink::Reply {
                    parameters: Some(p),
                    ..
                } => match serde_json::from_value(p) {
                    Ok(v) => ErrorKind::MethodNotFound(v),
                    Err(_) => ErrorKind::MethodNotFound(None),
                },
                _ => ErrorKind::MethodNotFound(None),
            },
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "org.varlink.service.MethodNotImplemented" => match e {
                varlink::Reply {
                    parameters: Some(p),
                    ..
                } => match serde_json::from_value(p) {
                    Ok(v) => ErrorKind::MethodNotImplemented(v),
                    Err(_) => ErrorKind::MethodNotImplemented(None),
                },
                _ => ErrorKind::MethodNotImplemented(None),
            },
            _ => ErrorKind::VarlinkReply_Error(e),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GetInfo_Reply {
    pub r#vendor: String,
    pub r#product: String,
    pub r#version: String,
    pub r#url: String,
    pub r#interfaces: Vec<String>,
}

impl varlink::VarlinkReply for GetInfo_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GetInfo_Args {}

pub trait Call_GetInfo: varlink::CallTrait {
    fn reply(
        &mut self,
        r#vendor: String,
        r#product: String,
        r#version: String,
        r#url: String,
        r#interfaces: Vec<String>,
    ) -> varlink::Result<()> {
        self.reply_struct(
            GetInfo_Reply {
                r#vendor,
                r#product,
                r#version,
                r#url,
                r#interfaces,
            }
            .into(),
        )
    }
}

impl<'a> Call_GetInfo for varlink::Call<'a> {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GetInterfaceDescription_Reply {
    pub r#description: String,
}

impl varlink::VarlinkReply for GetInterfaceDescription_Reply {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GetInterfaceDescription_Args {
    pub r#interface: String,
}

pub trait Call_GetInterfaceDescription: varlink::CallTrait {
    fn reply(&mut self, r#description: String) -> varlink::Result<()> {
        self.reply_struct(GetInterfaceDescription_Reply { r#description }.into())
    }
}

impl<'a> Call_GetInterfaceDescription for varlink::Call<'a> {}

pub trait VarlinkInterface {
    fn get_info(&self, call: &mut Call_GetInfo) -> varlink::Result<()>;
    fn get_interface_description(
        &self,
        call: &mut Call_GetInterfaceDescription,
        r#interface: String,
    ) -> varlink::Result<()>;
    fn call_upgraded(
        &self,
        _call: &mut varlink::Call,
        _bufreader: &mut BufRead,
    ) -> varlink::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

pub trait VarlinkClientInterface {
    fn get_info(&mut self) -> varlink::MethodCall<GetInfo_Args, GetInfo_Reply, Error>;
    fn get_interface_description(
        &mut self,
        r#interface: String,
    ) -> varlink::MethodCall<GetInterfaceDescription_Args, GetInterfaceDescription_Reply, Error>;
}

#[allow(dead_code)]
pub struct VarlinkClient {
    connection: Arc<RwLock<varlink::Connection>>,
}

impl VarlinkClient {
    #[allow(dead_code)]
    pub fn new(connection: Arc<RwLock<varlink::Connection>>) -> Self {
        VarlinkClient { connection }
    }
}

impl VarlinkClientInterface for VarlinkClient {
    fn get_info(&mut self) -> varlink::MethodCall<GetInfo_Args, GetInfo_Reply, Error> {
        varlink::MethodCall::<GetInfo_Args, GetInfo_Reply, Error>::new(
            self.connection.clone(),
            "org.varlink.service.GetInfo",
            GetInfo_Args {},
        )
    }
    fn get_interface_description(
        &mut self,
        r#interface: String,
    ) -> varlink::MethodCall<GetInterfaceDescription_Args, GetInterfaceDescription_Reply, Error>
    {
        varlink::MethodCall::<GetInterfaceDescription_Args, GetInterfaceDescription_Reply, Error>::new(self.connection.clone(), "org.varlink.service.GetInterfaceDescription", GetInterfaceDescription_Args { r#interface })
    }
}

#[allow(dead_code)]
pub struct VarlinkInterfaceProxy {
    inner: Box<VarlinkInterface + Send + Sync>,
}

#[allow(dead_code)]
pub fn new(inner: Box<VarlinkInterface + Send + Sync>) -> VarlinkInterfaceProxy {
    VarlinkInterfaceProxy { inner }
}

impl varlink::Interface for VarlinkInterfaceProxy {
    fn get_description(&self) -> &'static str {
        "# The Varlink Service Interface is provided by every varlink service. It\n# describes the service and the interfaces it implements.\ninterface org.varlink.service\n\n# Get a list of all the interfaces a service provides and information\n# about the implementation.\nmethod GetInfo() -> (\n  vendor: string,\n  product: string,\n  version: string,\n  url: string,\n  interfaces: []string\n)\n\n# Get the description of an interface that is implemented by this service.\nmethod GetInterfaceDescription(interface: string) -> (description: string)\n\n# The requested interface was not found.\nerror InterfaceNotFound (interface: string)\n\n# The requested method was not found\nerror MethodNotFound (method: string)\n\n# The interface defines the requested method, but the service does not\n# implement it.\nerror MethodNotImplemented (method: string)\n\n# One of the passed parameters is invalid.\nerror InvalidParameter (parameter: string)\n"
    }
    fn get_name(&self) -> &'static str {
        "org.varlink.service"
    }
    fn call_upgraded(
        &self,
        call: &mut varlink::Call,
        bufreader: &mut BufRead,
    ) -> varlink::Result<Vec<u8>> {
        self.inner.call_upgraded(call, bufreader)
    }
    fn call(&self, call: &mut varlink::Call) -> varlink::Result<()> {
        let req = call.request.unwrap();
        match req.method.as_ref() {
            "org.varlink.service.GetInfo" => self.inner.get_info(call as &mut Call_GetInfo),
            "org.varlink.service.GetInterfaceDescription" => {
                if let Some(args) = req.parameters.clone() {
                    let args: GetInterfaceDescription_Args = match serde_json::from_value(args) {
                        Ok(v) => v,
                        Err(e) => {
                            let es = format!("{}", e);
                            let _ = call.reply_invalid_parameter(es.clone());
                            return Err(
                                varlink::context!(varlink::ErrorKind::SerdeJsonDe(es, e)).into()
                            );
                        }
                    };
                    self.inner.get_interface_description(
                        call as &mut Call_GetInterfaceDescription,
                        args.r#interface,
                    )
                } else {
                    call.reply_invalid_parameter("parameters".into())
                }
            }
            m => call.reply_method_not_found(String::from(m)),
        }
    }
}
