// @generated by Thrift for thrift/compiler/test/fixtures/types/src/module.thrift
// This file is probably not the place you want to edit!

#![recursion_limit = "100000000"]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_crate_dependencies, clippy::all)]

pub use self::errors::*;
pub use self::types::*;

pub use :: as types;

#[doc(hidden)]
pub mod dependencies;
pub mod services;
pub mod client;
pub mod server;
pub mod mock;
pub mod errors;

pub(crate) mod r#impl {
    use ref_cast::RefCast;

    #[derive(RefCast)]
    #[repr(transparent)]
    pub(crate) struct LocalImpl<T>(T);

    #[allow(unused)]
    pub(crate) fn write<T, P>(value: &T, p: &mut P)
    where
        LocalImpl<T>: ::fbthrift::Serialize<P>,
        P: ::fbthrift::ProtocolWriter,
    {
        ::fbthrift::Serialize::write(LocalImpl::ref_cast(value), p);
    }

    #[allow(unused)]
    pub(crate) fn read<T, P>(p: &mut P) -> ::anyhow::Result<T>
    where
        LocalImpl<T>: ::fbthrift::Deserialize<P>,
        P: ::fbthrift::ProtocolReader,
    {
        let value: LocalImpl<T> = ::fbthrift::Deserialize::read(p)?;
        ::std::result::Result::Ok(value.0)
    }

    impl<P> ::fbthrift::Serialize<P> for LocalImpl<::sorted_vector_map::SortedVectorMap<::std::primitive::i32, ::std::string::String>>
    where
        P: ::fbthrift::ProtocolWriter,
    {
        fn write(&self, p: &mut P) {
            p.write_map_begin(
                <::std::primitive::i32 as ::fbthrift::GetTType>::TTYPE,
                <::std::string::String as ::fbthrift::GetTType>::TTYPE,
                self.0.len(),
            );
            for (k, v) in &self.0 {
                p.write_map_key_begin();
                ::fbthrift::Serialize::write(k, p);
                p.write_map_value_begin();
                ::fbthrift::Serialize::write(v, p);
            }
            p.write_map_end();
        }
    }

    impl<P> ::fbthrift::Deserialize<P> for LocalImpl<::sorted_vector_map::SortedVectorMap<::std::primitive::i32, ::std::string::String>>
    where
        P: ::fbthrift::ProtocolReader,
    {
        fn read(p: &mut P) -> ::anyhow::Result<Self> {
            let (_key_ty, _val_ty, len) = p.read_map_begin()?;
            let mut map = <crate::types::map_i32_string_1261>::with_capacity(len.unwrap_or_default());

            if let Some(0) = len {
                return Ok(LocalImpl(map));
            }

            let mut idx = 0;
            loop {
                let more = p.read_map_key_begin()?;
                if !more {
                    break;
                }
                let k: ::std::primitive::i32 = ::fbthrift::Deserialize::read(p)?;
                p.read_map_value_begin()?;
                let v: ::std::string::String = ::fbthrift::Deserialize::read(p)?;
                p.read_map_value_end()?;
                map.insert(k, v);

                idx += 1;
                if ::fbthrift::protocol::should_break(len, more, idx) {
                    break;
                }
            }
            p.read_map_end()?;
            ::std::result::Result::Ok(LocalImpl(map))
        }
    }

    impl<P> ::fbthrift::Serialize<P> for LocalImpl<::sorted_vector_map::SortedVectorSet<::std::primitive::i32>>
    where
        P: ::fbthrift::ProtocolWriter,
    {
        fn write(&self, p: &mut P) {
            p.write_set_begin(
                <::std::primitive::i32 as ::fbthrift::GetTType>::TTYPE,
                self.0.len(),
            );
            for v in &self.0 {
                p.write_set_value_begin();
                ::fbthrift::Serialize::write(v, p);
            }
            p.write_set_end();
        }
    }

    impl<P> ::fbthrift::Deserialize<P> for LocalImpl<::sorted_vector_map::SortedVectorSet<::std::primitive::i32>>
    where
        P: ::fbthrift::ProtocolReader,
    {
        fn read(p: &mut P) -> ::anyhow::Result<Self> {
            let (_elem_ty, len) = p.read_set_begin()?;
            let mut set = <crate::types::set_i32_7194>::with_capacity(len.unwrap_or_default());

            if let Some(0) = len {
                return Ok(LocalImpl(set));
            }

            let mut idx = 0;
            loop {
                let more = p.read_set_value_begin()?;
                if !more {
                    break;
                }
                let v: ::std::primitive::i32 = ::fbthrift::Deserialize::read(p)?;
                p.read_set_value_end()?;
                set.insert(v);

                idx += 1;
                if ::fbthrift::protocol::should_break(len, more, idx) {
                    break;
                }
            }
            p.read_set_end()?;
            ::std::result::Result::Ok(LocalImpl(set))
        }
    }
}
