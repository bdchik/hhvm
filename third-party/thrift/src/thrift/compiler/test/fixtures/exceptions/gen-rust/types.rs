// @generated by Thrift for thrift/compiler/test/fixtures/exceptions/src/module.thrift
// This file is probably not the place you want to edit!


#![recursion_limit = "100000000"]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_crate_dependencies, clippy::redundant_closure, clippy::type_complexity)]

#[allow(unused_imports)]
pub(crate) use crate as types;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fiery {
    pub message: ::std::string::String,
    // This field forces `..Default::default()` when instantiating this
    // struct, to make code future-proof against new fields added later to
    // the definition in Thrift. If you don't want this, add the annotation
    // `(rust.exhaustive)` to the Thrift struct to eliminate this field.
    #[doc(hidden)]
    pub _dot_dot_Default_default: self::dot_dot::OtherFields,
}

impl ::fbthrift::ExceptionInfo for Fiery {
    fn exn_value(&self) -> String {
        format!("{:?}", self)
    }

    #[inline]
    fn exn_is_declared(&self) -> bool { true }
}

impl ::std::error::Error for Fiery {}

impl ::std::fmt::Display for Fiery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Fiery: {}", self.message)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Serious {
    pub sonnet: ::std::option::Option<::std::string::String>,
    // This field forces `..Default::default()` when instantiating this
    // struct, to make code future-proof against new fields added later to
    // the definition in Thrift. If you don't want this, add the annotation
    // `(rust.exhaustive)` to the Thrift struct to eliminate this field.
    #[doc(hidden)]
    pub _dot_dot_Default_default: self::dot_dot::OtherFields,
}

impl ::fbthrift::ExceptionInfo for Serious {
    fn exn_value(&self) -> String {
        format!("{:?}", self)
    }

    #[inline]
    fn exn_is_declared(&self) -> bool { true }
}

impl ::std::error::Error for Serious {}

impl ::std::fmt::Display for Serious {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Serious: {:?}", self.sonnet)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComplexFieldNames {
    pub error_message: ::std::string::String,
    pub internal_error_message: ::std::string::String,
    // This field forces `..Default::default()` when instantiating this
    // struct, to make code future-proof against new fields added later to
    // the definition in Thrift. If you don't want this, add the annotation
    // `(rust.exhaustive)` to the Thrift struct to eliminate this field.
    #[doc(hidden)]
    pub _dot_dot_Default_default: self::dot_dot::OtherFields,
}

impl ::fbthrift::ExceptionInfo for ComplexFieldNames {
    fn exn_value(&self) -> String {
        format!("{:?}", self)
    }

    #[inline]
    fn exn_is_declared(&self) -> bool { true }
}

impl ::std::error::Error for ComplexFieldNames {}

impl ::std::fmt::Display for ComplexFieldNames {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "ComplexFieldNames: {}", self.internal_error_message)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomFieldNames {
    pub error_message: ::std::string::String,
    pub internal_error_message: ::std::string::String,
    // This field forces `..Default::default()` when instantiating this
    // struct, to make code future-proof against new fields added later to
    // the definition in Thrift. If you don't want this, add the annotation
    // `(rust.exhaustive)` to the Thrift struct to eliminate this field.
    #[doc(hidden)]
    pub _dot_dot_Default_default: self::dot_dot::OtherFields,
}

impl ::fbthrift::ExceptionInfo for CustomFieldNames {
    fn exn_value(&self) -> String {
        format!("{:?}", self)
    }

    #[inline]
    fn exn_is_declared(&self) -> bool { true }
}

impl ::std::error::Error for CustomFieldNames {}

impl ::std::fmt::Display for CustomFieldNames {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "CustomFieldNames: {}", self.internal_error_message)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExceptionWithPrimitiveField {
    pub message: ::std::string::String,
    pub error_code: ::std::primitive::i32,
    // This field forces `..Default::default()` when instantiating this
    // struct, to make code future-proof against new fields added later to
    // the definition in Thrift. If you don't want this, add the annotation
    // `(rust.exhaustive)` to the Thrift struct to eliminate this field.
    #[doc(hidden)]
    pub _dot_dot_Default_default: self::dot_dot::OtherFields,
}

impl ::fbthrift::ExceptionInfo for ExceptionWithPrimitiveField {
    fn exn_value(&self) -> String {
        format!("{:?}", self)
    }

    #[inline]
    fn exn_is_declared(&self) -> bool { true }
}

impl ::std::error::Error for ExceptionWithPrimitiveField {}

impl ::std::fmt::Display for ExceptionWithPrimitiveField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "ExceptionWithPrimitiveField: {}", self.message)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExceptionWithStructuredAnnotation {
    pub message_field: ::std::string::String,
    pub error_code: ::std::primitive::i32,
    // This field forces `..Default::default()` when instantiating this
    // struct, to make code future-proof against new fields added later to
    // the definition in Thrift. If you don't want this, add the annotation
    // `(rust.exhaustive)` to the Thrift struct to eliminate this field.
    #[doc(hidden)]
    pub _dot_dot_Default_default: self::dot_dot::OtherFields,
}

impl ::fbthrift::ExceptionInfo for ExceptionWithStructuredAnnotation {
    fn exn_value(&self) -> String {
        format!("{:?}", self)
    }

    #[inline]
    fn exn_is_declared(&self) -> bool { true }
}

impl ::std::error::Error for ExceptionWithStructuredAnnotation {}

impl ::std::fmt::Display for ExceptionWithStructuredAnnotation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Banal {
    // This field forces `..Default::default()` when instantiating this
    // struct, to make code future-proof against new fields added later to
    // the definition in Thrift. If you don't want this, add the annotation
    // `(rust.exhaustive)` to the Thrift struct to eliminate this field.
    #[doc(hidden)]
    pub _dot_dot_Default_default: self::dot_dot::OtherFields,
}

impl ::fbthrift::ExceptionInfo for Banal {
    fn exn_value(&self) -> String {
        format!("{:?}", self)
    }

    #[inline]
    fn exn_is_declared(&self) -> bool { true }
}

impl ::std::error::Error for Banal {}

impl ::std::fmt::Display for Banal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(clippy::derivable_impls)]
impl ::std::default::Default for self::Fiery {
    fn default() -> Self {
        Self {
            message: ::std::default::Default::default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        }
    }
}

impl ::std::fmt::Debug for self::Fiery {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter
            .debug_struct("Fiery")
            .field("message", &self.message)
            .finish()
    }
}

unsafe impl ::std::marker::Send for self::Fiery {}
unsafe impl ::std::marker::Sync for self::Fiery {}
impl ::std::marker::Unpin for self::Fiery {}

impl ::fbthrift::GetTType for self::Fiery {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for self::Fiery
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("Fiery");
        p.write_field_begin("message", ::fbthrift::TType::String, 1);
        ::fbthrift::Serialize::write(&self.message, p);
        p.write_field_end();
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for self::Fiery
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
            ::fbthrift::Field::new("message", ::fbthrift::TType::String, 1),
        ];
        let mut field_message = ::std::option::Option::None;
        let _ = p.read_struct_begin(|_| ())?;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32) {
                (::fbthrift::TType::Stop, _) => break,
                (::fbthrift::TType::String, 1) => field_message = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (fty, _) => p.skip(fty)?,
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(Self {
            message: field_message.unwrap_or_default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        })
    }
}


impl ::fbthrift::metadata::ThriftAnnotations for Fiery {
    fn get_structured_annotation<T: Sized + 'static>() -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        None
    }

    fn get_field_structured_annotation<T: Sized + 'static>(field_id: i16) -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        match field_id {
            1 => {
            },
            _ => {}
        }

        None
    }
}


#[allow(clippy::derivable_impls)]
impl ::std::default::Default for self::Serious {
    fn default() -> Self {
        Self {
            sonnet: ::std::option::Option::None,
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        }
    }
}

impl ::std::fmt::Debug for self::Serious {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter
            .debug_struct("Serious")
            .field("sonnet", &self.sonnet)
            .finish()
    }
}

unsafe impl ::std::marker::Send for self::Serious {}
unsafe impl ::std::marker::Sync for self::Serious {}
impl ::std::marker::Unpin for self::Serious {}

impl ::fbthrift::GetTType for self::Serious {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for self::Serious
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("Serious");
        if let ::std::option::Option::Some(some) = &self.sonnet {
            p.write_field_begin("sonnet", ::fbthrift::TType::String, 1);
            ::fbthrift::Serialize::write(some, p);
            p.write_field_end();
        }
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for self::Serious
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
            ::fbthrift::Field::new("sonnet", ::fbthrift::TType::String, 1),
        ];
        let mut field_sonnet = ::std::option::Option::None;
        let _ = p.read_struct_begin(|_| ())?;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32) {
                (::fbthrift::TType::Stop, _) => break,
                (::fbthrift::TType::String, 1) => field_sonnet = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (fty, _) => p.skip(fty)?,
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(Self {
            sonnet: field_sonnet,
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        })
    }
}


impl ::fbthrift::metadata::ThriftAnnotations for Serious {
    fn get_structured_annotation<T: Sized + 'static>() -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        None
    }

    fn get_field_structured_annotation<T: Sized + 'static>(field_id: i16) -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        match field_id {
            1 => {
            },
            _ => {}
        }

        None
    }
}


#[allow(clippy::derivable_impls)]
impl ::std::default::Default for self::ComplexFieldNames {
    fn default() -> Self {
        Self {
            error_message: ::std::default::Default::default(),
            internal_error_message: ::std::default::Default::default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        }
    }
}

impl ::std::fmt::Debug for self::ComplexFieldNames {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter
            .debug_struct("ComplexFieldNames")
            .field("error_message", &self.error_message)
            .field("internal_error_message", &self.internal_error_message)
            .finish()
    }
}

unsafe impl ::std::marker::Send for self::ComplexFieldNames {}
unsafe impl ::std::marker::Sync for self::ComplexFieldNames {}
impl ::std::marker::Unpin for self::ComplexFieldNames {}

impl ::fbthrift::GetTType for self::ComplexFieldNames {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for self::ComplexFieldNames
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("ComplexFieldNames");
        p.write_field_begin("error_message", ::fbthrift::TType::String, 1);
        ::fbthrift::Serialize::write(&self.error_message, p);
        p.write_field_end();
        p.write_field_begin("internal_error_message", ::fbthrift::TType::String, 2);
        ::fbthrift::Serialize::write(&self.internal_error_message, p);
        p.write_field_end();
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for self::ComplexFieldNames
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
            ::fbthrift::Field::new("error_message", ::fbthrift::TType::String, 1),
            ::fbthrift::Field::new("internal_error_message", ::fbthrift::TType::String, 2),
        ];
        let mut field_error_message = ::std::option::Option::None;
        let mut field_internal_error_message = ::std::option::Option::None;
        let _ = p.read_struct_begin(|_| ())?;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32) {
                (::fbthrift::TType::Stop, _) => break,
                (::fbthrift::TType::String, 1) => field_error_message = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::String, 2) => field_internal_error_message = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (fty, _) => p.skip(fty)?,
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(Self {
            error_message: field_error_message.unwrap_or_default(),
            internal_error_message: field_internal_error_message.unwrap_or_default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        })
    }
}


impl ::fbthrift::metadata::ThriftAnnotations for ComplexFieldNames {
    fn get_structured_annotation<T: Sized + 'static>() -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        None
    }

    fn get_field_structured_annotation<T: Sized + 'static>(field_id: i16) -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        match field_id {
            1 => {
            },
            2 => {
            },
            _ => {}
        }

        None
    }
}


#[allow(clippy::derivable_impls)]
impl ::std::default::Default for self::CustomFieldNames {
    fn default() -> Self {
        Self {
            error_message: ::std::default::Default::default(),
            internal_error_message: ::std::default::Default::default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        }
    }
}

impl ::std::fmt::Debug for self::CustomFieldNames {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter
            .debug_struct("CustomFieldNames")
            .field("error_message", &self.error_message)
            .field("internal_error_message", &self.internal_error_message)
            .finish()
    }
}

unsafe impl ::std::marker::Send for self::CustomFieldNames {}
unsafe impl ::std::marker::Sync for self::CustomFieldNames {}
impl ::std::marker::Unpin for self::CustomFieldNames {}

impl ::fbthrift::GetTType for self::CustomFieldNames {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for self::CustomFieldNames
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("CustomFieldNames");
        p.write_field_begin("error_message", ::fbthrift::TType::String, 1);
        ::fbthrift::Serialize::write(&self.error_message, p);
        p.write_field_end();
        p.write_field_begin("internal_error_message", ::fbthrift::TType::String, 2);
        ::fbthrift::Serialize::write(&self.internal_error_message, p);
        p.write_field_end();
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for self::CustomFieldNames
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
            ::fbthrift::Field::new("error_message", ::fbthrift::TType::String, 1),
            ::fbthrift::Field::new("internal_error_message", ::fbthrift::TType::String, 2),
        ];
        let mut field_error_message = ::std::option::Option::None;
        let mut field_internal_error_message = ::std::option::Option::None;
        let _ = p.read_struct_begin(|_| ())?;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32) {
                (::fbthrift::TType::Stop, _) => break,
                (::fbthrift::TType::String, 1) => field_error_message = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::String, 2) => field_internal_error_message = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (fty, _) => p.skip(fty)?,
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(Self {
            error_message: field_error_message.unwrap_or_default(),
            internal_error_message: field_internal_error_message.unwrap_or_default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        })
    }
}


impl ::fbthrift::metadata::ThriftAnnotations for CustomFieldNames {
    fn get_structured_annotation<T: Sized + 'static>() -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        None
    }

    fn get_field_structured_annotation<T: Sized + 'static>(field_id: i16) -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        match field_id {
            1 => {
            },
            2 => {
            },
            _ => {}
        }

        None
    }
}


#[allow(clippy::derivable_impls)]
impl ::std::default::Default for self::ExceptionWithPrimitiveField {
    fn default() -> Self {
        Self {
            message: ::std::default::Default::default(),
            error_code: ::std::default::Default::default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        }
    }
}

impl ::std::fmt::Debug for self::ExceptionWithPrimitiveField {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter
            .debug_struct("ExceptionWithPrimitiveField")
            .field("message", &self.message)
            .field("error_code", &self.error_code)
            .finish()
    }
}

unsafe impl ::std::marker::Send for self::ExceptionWithPrimitiveField {}
unsafe impl ::std::marker::Sync for self::ExceptionWithPrimitiveField {}
impl ::std::marker::Unpin for self::ExceptionWithPrimitiveField {}

impl ::fbthrift::GetTType for self::ExceptionWithPrimitiveField {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for self::ExceptionWithPrimitiveField
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("ExceptionWithPrimitiveField");
        p.write_field_begin("message", ::fbthrift::TType::String, 1);
        ::fbthrift::Serialize::write(&self.message, p);
        p.write_field_end();
        p.write_field_begin("error_code", ::fbthrift::TType::I32, 2);
        ::fbthrift::Serialize::write(&self.error_code, p);
        p.write_field_end();
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for self::ExceptionWithPrimitiveField
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
            ::fbthrift::Field::new("error_code", ::fbthrift::TType::I32, 2),
            ::fbthrift::Field::new("message", ::fbthrift::TType::String, 1),
        ];
        let mut field_message = ::std::option::Option::None;
        let mut field_error_code = ::std::option::Option::None;
        let _ = p.read_struct_begin(|_| ())?;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32) {
                (::fbthrift::TType::Stop, _) => break,
                (::fbthrift::TType::String, 1) => field_message = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::I32, 2) => field_error_code = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (fty, _) => p.skip(fty)?,
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(Self {
            message: field_message.unwrap_or_default(),
            error_code: field_error_code.unwrap_or_default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        })
    }
}


impl ::fbthrift::metadata::ThriftAnnotations for ExceptionWithPrimitiveField {
    fn get_structured_annotation<T: Sized + 'static>() -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        None
    }

    fn get_field_structured_annotation<T: Sized + 'static>(field_id: i16) -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        match field_id {
            1 => {
            },
            2 => {
            },
            _ => {}
        }

        None
    }
}


#[allow(clippy::derivable_impls)]
impl ::std::default::Default for self::ExceptionWithStructuredAnnotation {
    fn default() -> Self {
        Self {
            message_field: ::std::default::Default::default(),
            error_code: ::std::default::Default::default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        }
    }
}

impl ::std::fmt::Debug for self::ExceptionWithStructuredAnnotation {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter
            .debug_struct("ExceptionWithStructuredAnnotation")
            .field("message_field", &self.message_field)
            .field("error_code", &self.error_code)
            .finish()
    }
}

unsafe impl ::std::marker::Send for self::ExceptionWithStructuredAnnotation {}
unsafe impl ::std::marker::Sync for self::ExceptionWithStructuredAnnotation {}
impl ::std::marker::Unpin for self::ExceptionWithStructuredAnnotation {}

impl ::fbthrift::GetTType for self::ExceptionWithStructuredAnnotation {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for self::ExceptionWithStructuredAnnotation
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("ExceptionWithStructuredAnnotation");
        p.write_field_begin("message_field", ::fbthrift::TType::String, 1);
        ::fbthrift::Serialize::write(&self.message_field, p);
        p.write_field_end();
        p.write_field_begin("error_code", ::fbthrift::TType::I32, 2);
        ::fbthrift::Serialize::write(&self.error_code, p);
        p.write_field_end();
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for self::ExceptionWithStructuredAnnotation
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
            ::fbthrift::Field::new("error_code", ::fbthrift::TType::I32, 2),
            ::fbthrift::Field::new("message_field", ::fbthrift::TType::String, 1),
        ];
        let mut field_message_field = ::std::option::Option::None;
        let mut field_error_code = ::std::option::Option::None;
        let _ = p.read_struct_begin(|_| ())?;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32) {
                (::fbthrift::TType::Stop, _) => break,
                (::fbthrift::TType::String, 1) => field_message_field = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::I32, 2) => field_error_code = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (fty, _) => p.skip(fty)?,
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(Self {
            message_field: field_message_field.unwrap_or_default(),
            error_code: field_error_code.unwrap_or_default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        })
    }
}


impl ::fbthrift::metadata::ThriftAnnotations for ExceptionWithStructuredAnnotation {
    fn get_structured_annotation<T: Sized + 'static>() -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        if type_id == ::std::any::TypeId::of::<thrift::types::ExceptionMessage>() {
            let mut tmp = Some(thrift::types::ExceptionMessage {
                field: "message_field".to_owned(),
                ..::std::default::Default::default()
            });
            let r: &mut dyn ::std::any::Any = &mut tmp;
            let r: &mut Option<T> = r.downcast_mut().unwrap();
            return r.take();
        }

        None
    }

    fn get_field_structured_annotation<T: Sized + 'static>(field_id: i16) -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        match field_id {
            1 => {
            },
            2 => {
            },
            _ => {}
        }

        None
    }
}


#[allow(clippy::derivable_impls)]
impl ::std::default::Default for self::Banal {
    fn default() -> Self {
        Self {
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        }
    }
}

impl ::std::fmt::Debug for self::Banal {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter
            .debug_struct("Banal")
            .finish()
    }
}

unsafe impl ::std::marker::Send for self::Banal {}
unsafe impl ::std::marker::Sync for self::Banal {}
impl ::std::marker::Unpin for self::Banal {}

impl ::fbthrift::GetTType for self::Banal {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for self::Banal
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("Banal");
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for self::Banal
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
        ];
        let _ = p.read_struct_begin(|_| ())?;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32) {
                (::fbthrift::TType::Stop, _) => break,
                (fty, _) => p.skip(fty)?,
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(Self {
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        })
    }
}


impl ::fbthrift::metadata::ThriftAnnotations for Banal {
    fn get_structured_annotation<T: Sized + 'static>() -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        None
    }

    fn get_field_structured_annotation<T: Sized + 'static>(field_id: i16) -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        match field_id {
            _ => {}
        }

        None
    }
}


mod dot_dot {
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OtherFields(pub(crate) ());

    #[allow(dead_code)] // if serde isn't being used
    pub(super) fn default_for_serde_deserialize() -> OtherFields {
        OtherFields(())
    }
}

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
}
