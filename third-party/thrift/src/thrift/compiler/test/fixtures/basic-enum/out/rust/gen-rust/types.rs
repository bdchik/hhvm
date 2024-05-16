// @generated by Thrift for thrift/compiler/test/fixtures/basic-enum/src/module.thrift
// This file is probably not the place you want to edit!


#![recursion_limit = "100000000"]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_crate_dependencies, clippy::redundant_closure, clippy::type_complexity)]

pub mod consts;
#[doc(inline)]
pub use self::consts::*;
#[allow(unused_imports)]
pub(crate) use crate as types;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MyStruct {
    pub myEnum: crate::types::MyEnum,
    pub myBigEnum: crate::types::MyBigEnum,
    // This field forces `..Default::default()` when instantiating this
    // struct, to make code future-proof against new fields added later to
    // the definition in Thrift. If you don't want this, add the annotation
    // `@rust.Exhaustive` to the Thrift struct to eliminate this field.
    #[doc(hidden)]
    pub _dot_dot_Default_default: self::dot_dot::OtherFields,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EmptyEnum(pub ::std::primitive::i32);

impl EmptyEnum {
}

impl ::fbthrift::ThriftEnum for EmptyEnum {
    fn enumerate() -> &'static [(Self, &'static str)] {
        &[
        ]
    }

    fn variants() -> &'static [&'static str] {
        &[
        ]
    }

    fn variant_values() -> &'static [Self] {
        &[
        ]
    }
}

#[allow(clippy::derivable_impls)]
impl ::std::default::Default for EmptyEnum {
    fn default() -> Self {
        Self(0)
    }
}

impl<'a> ::std::convert::From<&'a EmptyEnum> for ::std::primitive::i32 {
    #[inline]
    fn from(x: &'a EmptyEnum) -> Self {
        x.0
    }
}

impl ::std::convert::From<EmptyEnum> for ::std::primitive::i32 {
    #[inline]
    fn from(x: EmptyEnum) -> Self {
        x.0
    }
}

impl ::std::convert::From<::std::primitive::i32> for EmptyEnum {
    #[inline]
    fn from(x: ::std::primitive::i32) -> Self {
        Self(x)
    }
}

impl ::std::fmt::Display for EmptyEnum {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        static VARIANTS_BY_NUMBER: &[(&::std::primitive::str, ::std::primitive::i32)] = &[
        ];
        ::fbthrift::help::enum_display(VARIANTS_BY_NUMBER, fmt, self.0)
    }
}

impl ::std::fmt::Debug for EmptyEnum {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(fmt, "EmptyEnum::{}", self)
    }
}

impl ::std::str::FromStr for EmptyEnum {
    type Err = ::anyhow::Error;

    fn from_str(string: &::std::primitive::str) -> ::std::result::Result<Self, Self::Err> {
        static VARIANTS_BY_NAME: &[(&::std::primitive::str, ::std::primitive::i32)] = &[
        ];
        ::fbthrift::help::enum_from_str(VARIANTS_BY_NAME, string, "EmptyEnum").map(Self)
    }
}

impl ::fbthrift::GetTType for EmptyEnum {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::I32;
}

impl<P> ::fbthrift::Serialize<P> for EmptyEnum
where
    P: ::fbthrift::ProtocolWriter,
{
    #[inline]
    fn write(&self, p: &mut P) {
        p.write_i32(self.into())
    }
}

impl<P> ::fbthrift::Deserialize<P> for EmptyEnum
where
    P: ::fbthrift::ProtocolReader,
{
    #[inline]
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        ::std::result::Result::Ok(Self::from(::anyhow::Context::context(p.read_i32(), "Expected a number indicating enum variant")?))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MyEnum(pub ::std::primitive::i32);

impl MyEnum {
    pub const ONE: Self = MyEnum(1i32);
    pub const TWO: Self = MyEnum(2i32);
}

impl ::fbthrift::ThriftEnum for MyEnum {
    fn enumerate() -> &'static [(Self, &'static str)] {
        &[
            (Self::ONE, "ONE"),
            (Self::TWO, "TWO"),
        ]
    }

    fn variants() -> &'static [&'static str] {
        &[
            "ONE",
            "TWO",
        ]
    }

    fn variant_values() -> &'static [Self] {
        &[
            Self::ONE,
            Self::TWO,
        ]
    }
}

#[allow(clippy::derivable_impls)]
impl ::std::default::Default for MyEnum {
    fn default() -> Self {
        Self(0)
    }
}

impl<'a> ::std::convert::From<&'a MyEnum> for ::std::primitive::i32 {
    #[inline]
    fn from(x: &'a MyEnum) -> Self {
        x.0
    }
}

impl ::std::convert::From<MyEnum> for ::std::primitive::i32 {
    #[inline]
    fn from(x: MyEnum) -> Self {
        x.0
    }
}

impl ::std::convert::From<::std::primitive::i32> for MyEnum {
    #[inline]
    fn from(x: ::std::primitive::i32) -> Self {
        Self(x)
    }
}

impl ::std::fmt::Display for MyEnum {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        static VARIANTS_BY_NUMBER: &[(&::std::primitive::str, ::std::primitive::i32)] = &[
            ("ONE", 1),
            ("TWO", 2),
        ];
        ::fbthrift::help::enum_display(VARIANTS_BY_NUMBER, fmt, self.0)
    }
}

impl ::std::fmt::Debug for MyEnum {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(fmt, "MyEnum::{}", self)
    }
}

impl ::std::str::FromStr for MyEnum {
    type Err = ::anyhow::Error;

    fn from_str(string: &::std::primitive::str) -> ::std::result::Result<Self, Self::Err> {
        static VARIANTS_BY_NAME: &[(&::std::primitive::str, ::std::primitive::i32)] = &[
            ("ONE", 1),
            ("TWO", 2),
        ];
        ::fbthrift::help::enum_from_str(VARIANTS_BY_NAME, string, "MyEnum").map(Self)
    }
}

impl ::fbthrift::GetTType for MyEnum {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::I32;
}

impl<P> ::fbthrift::Serialize<P> for MyEnum
where
    P: ::fbthrift::ProtocolWriter,
{
    #[inline]
    fn write(&self, p: &mut P) {
        p.write_i32(self.into())
    }
}

impl<P> ::fbthrift::Deserialize<P> for MyEnum
where
    P: ::fbthrift::ProtocolReader,
{
    #[inline]
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        ::std::result::Result::Ok(Self::from(::anyhow::Context::context(p.read_i32(), "Expected a number indicating enum variant")?))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MyBigEnum(pub ::std::primitive::i32);

impl MyBigEnum {
    pub const UNKNOWN: Self = MyBigEnum(0i32);
    pub const ONE: Self = MyBigEnum(1i32);
    pub const TWO: Self = MyBigEnum(2i32);
    pub const THREE: Self = MyBigEnum(3i32);
    pub const FOUR: Self = MyBigEnum(4i32);
    pub const FIVE: Self = MyBigEnum(5i32);
    pub const SIX: Self = MyBigEnum(6i32);
    pub const SEVEN: Self = MyBigEnum(7i32);
    pub const EIGHT: Self = MyBigEnum(8i32);
    pub const NINE: Self = MyBigEnum(9i32);
    pub const TEN: Self = MyBigEnum(10i32);
    pub const ELEVEN: Self = MyBigEnum(11i32);
    pub const TWELVE: Self = MyBigEnum(12i32);
    pub const THIRTEEN: Self = MyBigEnum(13i32);
    pub const FOURTEEN: Self = MyBigEnum(14i32);
    pub const FIFTEEN: Self = MyBigEnum(15i32);
    pub const SIXTEEN: Self = MyBigEnum(16i32);
    pub const SEVENTEEN: Self = MyBigEnum(17i32);
    pub const EIGHTEEN: Self = MyBigEnum(18i32);
    pub const NINETEEN: Self = MyBigEnum(19i32);
}

impl ::fbthrift::ThriftEnum for MyBigEnum {
    fn enumerate() -> &'static [(Self, &'static str)] {
        &[
            (Self::UNKNOWN, "UNKNOWN"),
            (Self::ONE, "ONE"),
            (Self::TWO, "TWO"),
            (Self::THREE, "THREE"),
            (Self::FOUR, "FOUR"),
            (Self::FIVE, "FIVE"),
            (Self::SIX, "SIX"),
            (Self::SEVEN, "SEVEN"),
            (Self::EIGHT, "EIGHT"),
            (Self::NINE, "NINE"),
            (Self::TEN, "TEN"),
            (Self::ELEVEN, "ELEVEN"),
            (Self::TWELVE, "TWELVE"),
            (Self::THIRTEEN, "THIRTEEN"),
            (Self::FOURTEEN, "FOURTEEN"),
            (Self::FIFTEEN, "FIFTEEN"),
            (Self::SIXTEEN, "SIXTEEN"),
            (Self::SEVENTEEN, "SEVENTEEN"),
            (Self::EIGHTEEN, "EIGHTEEN"),
            (Self::NINETEEN, "NINETEEN"),
        ]
    }

    fn variants() -> &'static [&'static str] {
        &[
            "UNKNOWN",
            "ONE",
            "TWO",
            "THREE",
            "FOUR",
            "FIVE",
            "SIX",
            "SEVEN",
            "EIGHT",
            "NINE",
            "TEN",
            "ELEVEN",
            "TWELVE",
            "THIRTEEN",
            "FOURTEEN",
            "FIFTEEN",
            "SIXTEEN",
            "SEVENTEEN",
            "EIGHTEEN",
            "NINETEEN",
        ]
    }

    fn variant_values() -> &'static [Self] {
        &[
            Self::UNKNOWN,
            Self::ONE,
            Self::TWO,
            Self::THREE,
            Self::FOUR,
            Self::FIVE,
            Self::SIX,
            Self::SEVEN,
            Self::EIGHT,
            Self::NINE,
            Self::TEN,
            Self::ELEVEN,
            Self::TWELVE,
            Self::THIRTEEN,
            Self::FOURTEEN,
            Self::FIFTEEN,
            Self::SIXTEEN,
            Self::SEVENTEEN,
            Self::EIGHTEEN,
            Self::NINETEEN,
        ]
    }
}

#[allow(clippy::derivable_impls)]
impl ::std::default::Default for MyBigEnum {
    fn default() -> Self {
        Self(0)
    }
}

impl<'a> ::std::convert::From<&'a MyBigEnum> for ::std::primitive::i32 {
    #[inline]
    fn from(x: &'a MyBigEnum) -> Self {
        x.0
    }
}

impl ::std::convert::From<MyBigEnum> for ::std::primitive::i32 {
    #[inline]
    fn from(x: MyBigEnum) -> Self {
        x.0
    }
}

impl ::std::convert::From<::std::primitive::i32> for MyBigEnum {
    #[inline]
    fn from(x: ::std::primitive::i32) -> Self {
        Self(x)
    }
}

impl ::std::fmt::Display for MyBigEnum {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        static VARIANTS_BY_NUMBER: &[(&::std::primitive::str, ::std::primitive::i32)] = &[
            ("UNKNOWN", 0),
            ("ONE", 1),
            ("TWO", 2),
            ("THREE", 3),
            ("FOUR", 4),
            ("FIVE", 5),
            ("SIX", 6),
            ("SEVEN", 7),
            ("EIGHT", 8),
            ("NINE", 9),
            ("TEN", 10),
            ("ELEVEN", 11),
            ("TWELVE", 12),
            ("THIRTEEN", 13),
            ("FOURTEEN", 14),
            ("FIFTEEN", 15),
            ("SIXTEEN", 16),
            ("SEVENTEEN", 17),
            ("EIGHTEEN", 18),
            ("NINETEEN", 19),
        ];
        ::fbthrift::help::enum_display(VARIANTS_BY_NUMBER, fmt, self.0)
    }
}

impl ::std::fmt::Debug for MyBigEnum {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(fmt, "MyBigEnum::{}", self)
    }
}

impl ::std::str::FromStr for MyBigEnum {
    type Err = ::anyhow::Error;

    fn from_str(string: &::std::primitive::str) -> ::std::result::Result<Self, Self::Err> {
        static VARIANTS_BY_NAME: &[(&::std::primitive::str, ::std::primitive::i32)] = &[
            ("EIGHT", 8),
            ("EIGHTEEN", 18),
            ("ELEVEN", 11),
            ("FIFTEEN", 15),
            ("FIVE", 5),
            ("FOUR", 4),
            ("FOURTEEN", 14),
            ("NINE", 9),
            ("NINETEEN", 19),
            ("ONE", 1),
            ("SEVEN", 7),
            ("SEVENTEEN", 17),
            ("SIX", 6),
            ("SIXTEEN", 16),
            ("TEN", 10),
            ("THIRTEEN", 13),
            ("THREE", 3),
            ("TWELVE", 12),
            ("TWO", 2),
            ("UNKNOWN", 0),
        ];
        ::fbthrift::help::enum_from_str(VARIANTS_BY_NAME, string, "MyBigEnum").map(Self)
    }
}

impl ::fbthrift::GetTType for MyBigEnum {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::I32;
}

impl<P> ::fbthrift::Serialize<P> for MyBigEnum
where
    P: ::fbthrift::ProtocolWriter,
{
    #[inline]
    fn write(&self, p: &mut P) {
        p.write_i32(self.into())
    }
}

impl<P> ::fbthrift::Deserialize<P> for MyBigEnum
where
    P: ::fbthrift::ProtocolReader,
{
    #[inline]
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        ::std::result::Result::Ok(Self::from(::anyhow::Context::context(p.read_i32(), "Expected a number indicating enum variant")?))
    }
}

#[allow(clippy::derivable_impls)]
impl ::std::default::Default for self::MyStruct {
    fn default() -> Self {
        Self {
            myEnum: ::std::default::Default::default(),
            myBigEnum: crate::types::MyBigEnum::ONE,
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        }
    }
}

impl ::std::fmt::Debug for self::MyStruct {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter
            .debug_struct("MyStruct")
            .field("myEnum", &self.myEnum)
            .field("myBigEnum", &self.myBigEnum)
            .finish()
    }
}

unsafe impl ::std::marker::Send for self::MyStruct {}
unsafe impl ::std::marker::Sync for self::MyStruct {}
impl ::std::marker::Unpin for self::MyStruct {}
impl ::std::panic::RefUnwindSafe for self::MyStruct {}
impl ::std::panic::UnwindSafe for self::MyStruct {}

impl ::fbthrift::GetTType for self::MyStruct {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for self::MyStruct
where
    P: ::fbthrift::ProtocolWriter,
{
    #[inline]
    fn write(&self, p: &mut P) {
        p.write_struct_begin("MyStruct");
        p.write_field_begin("myEnum", ::fbthrift::TType::I32, 1);
        ::fbthrift::Serialize::write(&self.myEnum, p);
        p.write_field_end();
        p.write_field_begin("myBigEnum", ::fbthrift::TType::I32, 2);
        ::fbthrift::Serialize::write(&self.myBigEnum, p);
        p.write_field_end();
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for self::MyStruct
where
    P: ::fbthrift::ProtocolReader,
{
    #[inline]
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
            ::fbthrift::Field::new("myBigEnum", ::fbthrift::TType::I32, 2),
            ::fbthrift::Field::new("myEnum", ::fbthrift::TType::I32, 1),
        ];
        let mut field_myEnum = ::std::option::Option::None;
        let mut field_myBigEnum = ::std::option::Option::None;
        let _ = ::anyhow::Context::context(p.read_struct_begin(|_| ()), "Expected a MyStruct")?;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32) {
                (::fbthrift::TType::Stop, _) => break,
                (::fbthrift::TType::I32, 1) => field_myEnum = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::I32, 2) => field_myBigEnum = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (fty, _) => p.skip(fty)?,
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(Self {
            myEnum: field_myEnum.unwrap_or_default(),
            myBigEnum: field_myBigEnum.unwrap_or(crate::types::MyBigEnum::ONE),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        })
    }
}


impl ::fbthrift::metadata::ThriftAnnotations for MyStruct {
    fn get_structured_annotation<T: Sized + 'static>() -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        None
    }

    fn get_field_structured_annotation<T: Sized + 'static>(field_id: i16) -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        #[allow(clippy::match_single_binding)]
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
