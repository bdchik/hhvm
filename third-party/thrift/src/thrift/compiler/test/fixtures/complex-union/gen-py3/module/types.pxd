#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#

from libc.stdint cimport (
    int8_t as cint8_t,
    int16_t as cint16_t,
    int32_t as cint32_t,
    int64_t as cint64_t,
    uint32_t as cuint32_t,
)
from libcpp.string cimport string
from libcpp cimport bool as cbool, nullptr, nullptr_t
from cpython cimport bool as pbool
from libcpp.memory cimport shared_ptr, unique_ptr
from libcpp.utility cimport move as cmove
from libcpp.vector cimport vector
from libcpp.set cimport set as cset
from libcpp.map cimport map as cmap, pair as cpair
from thrift.py3.exceptions cimport cTException
cimport folly.iobuf as _fbthrift_iobuf
cimport thrift.py3.exceptions
cimport thrift.py3.types
from thrift.py3.types cimport (
    bstring,
    bytes_to_string,
    field_ref as __field_ref,
    optional_field_ref as __optional_field_ref,
    required_field_ref as __required_field_ref,
    terse_field_ref as __terse_field_ref,
    union_field_ref as __union_field_ref,
    get_union_field_value as __get_union_field_value,
)
from thrift.py3.common cimport (
    RpcOptions as __RpcOptions,
    Protocol as __Protocol,
    cThriftMetadata as __fbthrift_cThriftMetadata,
    MetadataBox as __MetadataBox,
)
from folly.optional cimport cOptional as __cOptional

cimport module.types_fields as _fbthrift_types_fields

cdef extern from "thrift/compiler/test/fixtures/complex-union/gen-py3/module/types.h":
  pass





cdef extern from "thrift/compiler/test/fixtures/complex-union/gen-cpp2/module_metadata.h" namespace "apache::thrift::detail::md":
    cdef cppclass ExceptionMetadata[T]:
        @staticmethod
        void gen(__fbthrift_cThriftMetadata &metadata)
cdef extern from "thrift/compiler/test/fixtures/complex-union/gen-cpp2/module_metadata.h" namespace "apache::thrift::detail::md":
    cdef cppclass StructMetadata[T]:
        @staticmethod
        void gen(__fbthrift_cThriftMetadata &metadata)
cdef extern from "thrift/compiler/test/fixtures/complex-union/gen-cpp2/module_types_custom_protocol.h" namespace "::cpp2":
    cdef enum cComplexUnion__type "::cpp2::ComplexUnion::Type":
        cComplexUnion__type___EMPTY__ "::cpp2::ComplexUnion::Type::__EMPTY__",
        cComplexUnion__type_intValue "::cpp2::ComplexUnion::Type::intValue",
        cComplexUnion__type_stringValue "::cpp2::ComplexUnion::Type::stringValue",
        cComplexUnion__type_intListValue "::cpp2::ComplexUnion::Type::intListValue",
        cComplexUnion__type_stringListValue "::cpp2::ComplexUnion::Type::stringListValue",
        cComplexUnion__type_typedefValue "::cpp2::ComplexUnion::Type::typedefValue",
        cComplexUnion__type_stringRef "::cpp2::ComplexUnion::Type::stringRef",

    cdef cppclass cComplexUnion "::cpp2::ComplexUnion":
        cComplexUnion() except +
        cComplexUnion(const cComplexUnion&) except +
        bint operator==(cComplexUnion&)
        bint operator!=(cComplexUnion&)
        bint operator<(cComplexUnion&)
        bint operator>(cComplexUnion&)
        bint operator<=(cComplexUnion&)
        bint operator>=(cComplexUnion&)
        cComplexUnion__type getType() const
        const cint64_t& get_intValue "get_intValue" () const
        cint64_t& set_intValue "set_intValue" (const cint64_t&)
        const string& get_stringValue "get_stringValue" () const
        string& set_stringValue "set_stringValue" (const string&)
        const vector[cint64_t]& get_intListValue "get_intListValue" () const
        vector[cint64_t]& set_intListValue "set_intListValue" (const vector[cint64_t]&)
        const vector[string]& get_stringListValue "get_stringListValue" () const
        vector[string]& set_stringListValue "set_stringListValue" (const vector[string]&)
        const cmap[cint16_t,string]& get_typedefValue "get_typedefValue" () const
        cmap[cint16_t,string]& set_typedefValue "set_typedefValue" (const cmap[cint16_t,string]&)
        const unique_ptr[string]& get_stringRef "get_stringRef" () const
        unique_ptr[string]& set_stringRef "set_stringRef" (const string&)

    cdef enum cListUnion__type "::cpp2::ListUnion::Type":
        cListUnion__type___EMPTY__ "::cpp2::ListUnion::Type::__EMPTY__",
        cListUnion__type_intListValue "::cpp2::ListUnion::Type::intListValue",
        cListUnion__type_stringListValue "::cpp2::ListUnion::Type::stringListValue",

    cdef cppclass cListUnion "::cpp2::ListUnion":
        cListUnion() except +
        cListUnion(const cListUnion&) except +
        bint operator==(cListUnion&)
        bint operator!=(cListUnion&)
        bint operator<(cListUnion&)
        bint operator>(cListUnion&)
        bint operator<=(cListUnion&)
        bint operator>=(cListUnion&)
        cListUnion__type getType() const
        const vector[cint64_t]& get_intListValue "get_intListValue" () const
        vector[cint64_t]& set_intListValue "set_intListValue" (const vector[cint64_t]&)
        const vector[string]& get_stringListValue "get_stringListValue" () const
        vector[string]& set_stringListValue "set_stringListValue" (const vector[string]&)

    cdef enum cDataUnion__type "::cpp2::DataUnion::Type":
        cDataUnion__type___EMPTY__ "::cpp2::DataUnion::Type::__EMPTY__",
        cDataUnion__type_binaryData "::cpp2::DataUnion::Type::binaryData",
        cDataUnion__type_stringData "::cpp2::DataUnion::Type::stringData",

    cdef cppclass cDataUnion "::cpp2::DataUnion":
        cDataUnion() except +
        cDataUnion(const cDataUnion&) except +
        bint operator==(cDataUnion&)
        bint operator!=(cDataUnion&)
        bint operator<(cDataUnion&)
        bint operator>(cDataUnion&)
        bint operator<=(cDataUnion&)
        bint operator>=(cDataUnion&)
        cDataUnion__type getType() const
        const string& get_binaryData "get_binaryData" () const
        string& set_binaryData "set_binaryData" (const string&)
        const string& get_stringData "get_stringData" () const
        string& set_stringData "set_stringData" (const string&)


    cdef cppclass cVal "::cpp2::Val":
        cVal() except +
        cVal(const cVal&) except +
        bint operator==(cVal&)
        bint operator!=(cVal&)
        bint operator<(cVal&)
        bint operator>(cVal&)
        bint operator<=(cVal&)
        bint operator>=(cVal&)
        __field_ref[string] strVal_ref "strVal_ref" ()
        __field_ref[cint32_t] intVal_ref "intVal_ref" ()
        __field_ref[cmap[cint16_t,string]] typedefValue_ref "typedefValue_ref" ()

    cdef enum cValUnion__type "::cpp2::ValUnion::Type":
        cValUnion__type___EMPTY__ "::cpp2::ValUnion::Type::__EMPTY__",
        cValUnion__type_v1 "::cpp2::ValUnion::Type::v1",
        cValUnion__type_v2 "::cpp2::ValUnion::Type::v2",

    cdef cppclass cValUnion "::cpp2::ValUnion":
        cValUnion() except +
        cValUnion(const cValUnion&) except +
        bint operator==(cValUnion&)
        bint operator!=(cValUnion&)
        bint operator<(cValUnion&)
        bint operator>(cValUnion&)
        bint operator<=(cValUnion&)
        bint operator>=(cValUnion&)
        cValUnion__type getType() const
        const cVal& get_v1 "get_v1" () const
        cVal& set_v1 "set_v1" (const cVal&)
        const cVal& get_v2 "get_v2" () const
        cVal& set_v2 "set_v2" (const cVal&)

    cdef enum cVirtualComplexUnion__type "::cpp2::VirtualComplexUnion::Type":
        cVirtualComplexUnion__type___EMPTY__ "::cpp2::VirtualComplexUnion::Type::__EMPTY__",
        cVirtualComplexUnion__type_thingOne "::cpp2::VirtualComplexUnion::Type::thingOne",
        cVirtualComplexUnion__type_thingTwo "::cpp2::VirtualComplexUnion::Type::thingTwo",

    cdef cppclass cVirtualComplexUnion "::cpp2::VirtualComplexUnion":
        cVirtualComplexUnion() except +
        cVirtualComplexUnion(const cVirtualComplexUnion&) except +
        bint operator==(cVirtualComplexUnion&)
        bint operator!=(cVirtualComplexUnion&)
        bint operator<(cVirtualComplexUnion&)
        bint operator>(cVirtualComplexUnion&)
        bint operator<=(cVirtualComplexUnion&)
        bint operator>=(cVirtualComplexUnion&)
        cVirtualComplexUnion__type getType() const
        const string& get_thingOne "get_thingOne" () const
        string& set_thingOne "set_thingOne" (const string&)
        const string& get_thingTwo "get_thingTwo" () const
        string& set_thingTwo "set_thingTwo" (const string&)


    cdef cppclass cNonCopyableStruct "::cpp2::NonCopyableStruct":
        cNonCopyableStruct() except +
        bint operator==(cNonCopyableStruct&)
        bint operator!=(cNonCopyableStruct&)
        bint operator<(cNonCopyableStruct&)
        bint operator>(cNonCopyableStruct&)
        bint operator<=(cNonCopyableStruct&)
        bint operator>=(cNonCopyableStruct&)
        __field_ref[cint64_t] num_ref "num_ref" ()

    cdef enum cNonCopyableUnion__type "::cpp2::NonCopyableUnion::Type":
        cNonCopyableUnion__type___EMPTY__ "::cpp2::NonCopyableUnion::Type::__EMPTY__",
        cNonCopyableUnion__type_s "::cpp2::NonCopyableUnion::Type::s",

    cdef cppclass cNonCopyableUnion "::cpp2::NonCopyableUnion":
        cNonCopyableUnion() except +
        bint operator==(cNonCopyableUnion&)
        bint operator!=(cNonCopyableUnion&)
        bint operator<(cNonCopyableUnion&)
        bint operator>(cNonCopyableUnion&)
        bint operator<=(cNonCopyableUnion&)
        bint operator>=(cNonCopyableUnion&)
        cNonCopyableUnion__type getType() const
        const cNonCopyableStruct& get_s "get_s" () const
        cNonCopyableStruct& set_s "set_s" (const cNonCopyableStruct&)


cdef class __ComplexUnionType(thrift.py3.types.CompiledEnum):
    pass




cdef class ComplexUnion(thrift.py3.types.Union):
    cdef shared_ptr[cComplexUnion] _cpp_obj
    cdef readonly __ComplexUnionType type
    cdef readonly object value
    cdef _load_cache(ComplexUnion self)

    @staticmethod
    cdef unique_ptr[cComplexUnion] _make_instance(
        cComplexUnion* base_instance,
        object intValue,
        str stringValue,
        object intListValue,
        object stringListValue,
        object typedefValue,
        str stringRef
    ) except *

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cComplexUnion])

cdef class __ListUnionType(thrift.py3.types.CompiledEnum):
    pass




cdef class ListUnion(thrift.py3.types.Union):
    cdef shared_ptr[cListUnion] _cpp_obj
    cdef readonly __ListUnionType type
    cdef readonly object value
    cdef _load_cache(ListUnion self)

    @staticmethod
    cdef unique_ptr[cListUnion] _make_instance(
        cListUnion* base_instance,
        object intListValue,
        object stringListValue
    ) except *

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cListUnion])

cdef class __DataUnionType(thrift.py3.types.CompiledEnum):
    pass




cdef class DataUnion(thrift.py3.types.Union):
    cdef shared_ptr[cDataUnion] _cpp_obj
    cdef readonly __DataUnionType type
    cdef readonly object value
    cdef _load_cache(DataUnion self)

    @staticmethod
    cdef unique_ptr[cDataUnion] _make_instance(
        cDataUnion* base_instance,
        bytes binaryData,
        str stringData
    ) except *

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cDataUnion])



cdef class Val(thrift.py3.types.Struct):
    cdef shared_ptr[cVal] _cpp_obj
    cdef _fbthrift_types_fields.__Val_FieldsSetter _fields_setter
    cdef inline object strVal_impl(self)
    cdef inline object intVal_impl(self)
    cdef inline object typedefValue_impl(self)
    cdef Map__i16_string __fbthrift_cached_typedefValue

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cVal])

cdef class __ValUnionType(thrift.py3.types.CompiledEnum):
    pass




cdef class ValUnion(thrift.py3.types.Union):
    cdef shared_ptr[cValUnion] _cpp_obj
    cdef readonly __ValUnionType type
    cdef readonly object value
    cdef _load_cache(ValUnion self)

    @staticmethod
    cdef unique_ptr[cValUnion] _make_instance(
        cValUnion* base_instance,
        Val v1,
        Val v2
    ) except *

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cValUnion])

cdef class __VirtualComplexUnionType(thrift.py3.types.CompiledEnum):
    pass




cdef class VirtualComplexUnion(thrift.py3.types.Union):
    cdef shared_ptr[cVirtualComplexUnion] _cpp_obj
    cdef readonly __VirtualComplexUnionType type
    cdef readonly object value
    cdef _load_cache(VirtualComplexUnion self)

    @staticmethod
    cdef unique_ptr[cVirtualComplexUnion] _make_instance(
        cVirtualComplexUnion* base_instance,
        str thingOne,
        str thingTwo
    ) except *

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cVirtualComplexUnion])



cdef class NonCopyableStruct(thrift.py3.types.Struct):
    cdef shared_ptr[cNonCopyableStruct] _cpp_obj
    cdef _fbthrift_types_fields.__NonCopyableStruct_FieldsSetter _fields_setter
    cdef inline object num_impl(self)

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cNonCopyableStruct])

cdef class __NonCopyableUnionType(thrift.py3.types.CompiledEnum):
    pass




cdef class NonCopyableUnion(thrift.py3.types.Union):
    cdef shared_ptr[cNonCopyableUnion] _cpp_obj
    cdef readonly __NonCopyableUnionType type
    cdef readonly object value
    cdef _load_cache(NonCopyableUnion self)

    @staticmethod
    cdef unique_ptr[cNonCopyableUnion] _make_instance(
        cNonCopyableUnion* base_instance,
        NonCopyableStruct s
    ) except *

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cNonCopyableUnion])


cdef class List__i64(thrift.py3.types.List):
    cdef shared_ptr[vector[cint64_t]] _cpp_obj
    @staticmethod
    cdef _fbthrift_create(shared_ptr[vector[cint64_t]])
    @staticmethod
    cdef shared_ptr[vector[cint64_t]] _make_instance(object items) except *

cdef class List__string(thrift.py3.types.List):
    cdef shared_ptr[vector[string]] _cpp_obj
    @staticmethod
    cdef _fbthrift_create(shared_ptr[vector[string]])
    @staticmethod
    cdef shared_ptr[vector[string]] _make_instance(object items) except *

cdef class Map__i16_string(thrift.py3.types.Map):
    cdef shared_ptr[cmap[cint16_t,string]] _cpp_obj
    @staticmethod
    cdef _fbthrift_create(shared_ptr[cmap[cint16_t,string]])
    @staticmethod
    cdef shared_ptr[cmap[cint16_t,string]] _make_instance(object items) except *


