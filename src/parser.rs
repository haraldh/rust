#![allow(dead_code)]

mod varlink_grammar {
    include!(concat!(env!("OUT_DIR"), "/varlink_grammar.rs"));
}

#[cfg(test)]
use self::varlink_grammar::*;

#[test]
fn test_standard() {
    assert!(interfaces("
org.varlink.service {
  type Type (name: string, typestring: string)
  type Method (
    name: string,
    monitor: bool,
    type_in: string,
    type_out: string
  )
  type Interface (name: string, types: Type[], methods: Method[])
  type Property (key: string, value: string)
  type InterfaceDescription (
    description: string,
    types: string[],
    methods: string[]
  )

  Help() -> (
    description: string,
    properties: Property[],
    interfaces: InterfaceDescription[]
  )
}
")
        .is_ok());
}

#[test]
fn test_one_method() {
    assert!(interfaces("
org.varlink.service {
  Foo() -> (bar : uint64)
}
")
        .is_ok());
}

#[test]
fn test_no_method() {
    assert!(interfaces("
org.varlink.service {
  type Interface (name: string, types: Type[], methods: Method[])
  type Property (key: string, value: string)
}
")
        .is_err());

}