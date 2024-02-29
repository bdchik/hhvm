// @generated by Thrift for thrift/compiler/test/fixtures/interactions/src/module.thrift
// This file is probably not the place you want to edit!

#![recursion_limit = "100000000"]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_crate_dependencies, clippy::all)]

#[allow(unused_imports)]
#[doc(no_inline)]
pub use self::errors::*;
#[doc(inline)]
pub use self::types::*;

#[doc(inline)]
pub use :: as types;

#[doc(inline)]
pub use ::::services as services;

/// Client implementation for each service in `module`.
pub mod client {
    #[doc(inline)]
    pub use ::::*;
}

/// Mock clients for `module`.
pub mod mock {
    #[doc(inline)]
    pub use ::::mock::*;
}

/// Server traits for each service in `module`.
pub mod server {
    #[doc(inline)]
    pub use ::::*;
}
#[doc(inline)]
pub use ::::errors as errors;
