#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#
cimport cython as __cython
from cpython.object cimport PyTypeObject, Py_LT, Py_LE, Py_EQ, Py_NE, Py_GT, Py_GE
from libcpp.memory cimport shared_ptr, make_shared, unique_ptr
from libcpp.optional cimport optional as __optional
from libcpp.string cimport string
from libcpp cimport bool as cbool
from libcpp.iterator cimport inserter as cinserter
from libcpp.utility cimport move as cmove
from cpython cimport bool as pbool
from cython.operator cimport dereference as deref, preincrement as inc, address as ptr_address
import thrift.py3.types
from thrift.py3.types import _IsSet as _fbthrift_IsSet
from thrift.py3.types cimport make_unique
cimport thrift.py3.types
cimport thrift.py3.exceptions
from thrift.py3.std_libcpp cimport sv_to_str as __sv_to_str, string_view as __cstring_view
from thrift.py3.types cimport (
    cSetOp as __cSetOp,
    richcmp as __richcmp,
    set_op as __set_op,
    setcmp as __setcmp,
    list_index as __list_index,
    list_count as __list_count,
    list_slice as __list_slice,
    list_getitem as __list_getitem,
    set_iter as __set_iter,
    map_iter as __map_iter,
    map_contains as __map_contains,
    map_getitem as __map_getitem,
    reference_shared_ptr as __reference_shared_ptr,
    get_field_name_by_index as __get_field_name_by_index,
    reset_field as __reset_field,
    translate_cpp_enum_to_python,
    SetMetaClass as __SetMetaClass,
    const_pointer_cast,
    constant_shared_ptr,
    NOTSET as __NOTSET,
    EnumData as __EnumData,
    EnumFlagsData as __EnumFlagsData,
    UnionTypeEnumData as __UnionTypeEnumData,
    createEnumDataForUnionType as __createEnumDataForUnionType,
)
cimport thrift.py3.serializer as serializer
from thrift.python.protocol cimport Protocol as __Protocol
import folly.iobuf as _fbthrift_iobuf
from folly.optional cimport cOptional
from folly.memory cimport to_shared_ptr as __to_shared_ptr
from folly.range cimport Range as __cRange

import sys
from collections.abc import Sequence, Set, Mapping, Iterable
import weakref as __weakref
import builtins as _builtins
cimport foo.types as _foo_types
import foo.types as _foo_types

import module.types_reflection as _types_reflection



@__cython.auto_pickle(False)
cdef class Fields(thrift.py3.types.Struct):
    def __init__(Fields self, **kwargs):
        self._cpp_obj = make_shared[cFields]()
        self._fields_setter = _fbthrift_types_fields.__Fields_FieldsSetter._fbthrift_create(self._cpp_obj.get())
        super().__init__(**kwargs)

    def __call__(Fields self, **kwargs):
        if not kwargs:
            return self
        cdef Fields __fbthrift_inst = Fields.__new__(Fields)
        __fbthrift_inst._cpp_obj = make_shared[cFields](deref(self._cpp_obj))
        __fbthrift_inst._fields_setter = _fbthrift_types_fields.__Fields_FieldsSetter._fbthrift_create(__fbthrift_inst._cpp_obj.get())
        for __fbthrift_name, _fbthrift_value in kwargs.items():
            __fbthrift_inst._fbthrift_set_field(__fbthrift_name, _fbthrift_value)
        return __fbthrift_inst

    cdef void _fbthrift_set_field(self, str name, object value) except *:
        self._fields_setter.set_field(name.encode("utf-8"), value)

    cdef object _fbthrift_isset(self):
        return _fbthrift_IsSet("Fields", {
          "injected_field": deref(self._cpp_obj).injected_field_ref().has_value(),
        })

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cFields] cpp_obj):
        __fbthrift_inst = <Fields>Fields.__new__(Fields)
        __fbthrift_inst._cpp_obj = cmove(cpp_obj)
        return __fbthrift_inst

    cdef inline injected_field_impl(self):

        return (<bytes>deref(self._cpp_obj).injected_field_ref().value()).decode('UTF-8')

    @property
    def injected_field(self):
        return self.injected_field_impl()


    def __hash__(Fields self):
        return super().__hash__()

    def __repr__(Fields self):
        return super().__repr__()

    def __str__(Fields self):
        return super().__str__()


    def __copy__(Fields self):
        cdef shared_ptr[cFields] cpp_obj = make_shared[cFields](
            deref(self._cpp_obj)
        )
        return Fields._fbthrift_create(cmove(cpp_obj))

    def __richcmp__(self, other, int op):
        r = self._fbthrift_cmp_sametype(other, op)
        return __richcmp[cFields](
            self._cpp_obj,
            (<Fields>other)._cpp_obj,
            op,
        ) if r is None else r

    @staticmethod
    def __get_reflection__():
        return _types_reflection.get_reflection__Fields()

    @staticmethod
    def __get_metadata__():
        cdef __fbthrift_cThriftMetadata meta
        StructMetadata[cFields].gen(meta)
        return __MetadataBox.box(cmove(meta))

    @staticmethod
    def __get_thrift_name__():
        return "module.Fields"

    @classmethod
    def _fbthrift_get_field_name_by_index(cls, idx):
        return __sv_to_str(__get_field_name_by_index[cFields](idx))

    @classmethod
    def _fbthrift_get_struct_size(cls):
        return 1

    cdef _fbthrift_iobuf.IOBuf _fbthrift_serialize(Fields self, __Protocol proto):
        cdef unique_ptr[_fbthrift_iobuf.cIOBuf] data
        with nogil:
            data = cmove(serializer.cserialize[cFields](self._cpp_obj.get(), proto))
        return _fbthrift_iobuf.from_unique_ptr(cmove(data))

    cdef cuint32_t _fbthrift_deserialize(Fields self, const _fbthrift_iobuf.cIOBuf* buf, __Protocol proto) except? 0:
        cdef cuint32_t needed
        self._cpp_obj = make_shared[cFields]()
        with nogil:
            needed = serializer.cdeserialize[cFields](buf, self._cpp_obj.get(), proto)
        return needed


    def _to_python(self):
        import importlib
        import thrift.python.converter
        python_types = importlib.import_module(
            "module.thrift_types"
        )
        return thrift.python.converter.to_python_struct(python_types.Fields, self)

    def _to_py3(self):
        return self

    def _to_py_deprecated(self):
        import importlib
        import thrift.util.converter
        py_deprecated_types = importlib.import_module("module.ttypes")
        return thrift.util.converter.to_py_struct(py_deprecated_types.Fields, self)
@__cython.auto_pickle(False)
cdef class FieldsInjectedToEmptyStruct(thrift.py3.types.Struct):
    def __init__(FieldsInjectedToEmptyStruct self, **kwargs):
        self._cpp_obj = make_shared[cFieldsInjectedToEmptyStruct]()
        self._fields_setter = _fbthrift_types_fields.__FieldsInjectedToEmptyStruct_FieldsSetter._fbthrift_create(self._cpp_obj.get())
        super().__init__(**kwargs)

    def __call__(FieldsInjectedToEmptyStruct self, **kwargs):
        if not kwargs:
            return self
        cdef FieldsInjectedToEmptyStruct __fbthrift_inst = FieldsInjectedToEmptyStruct.__new__(FieldsInjectedToEmptyStruct)
        __fbthrift_inst._cpp_obj = make_shared[cFieldsInjectedToEmptyStruct](deref(self._cpp_obj))
        __fbthrift_inst._fields_setter = _fbthrift_types_fields.__FieldsInjectedToEmptyStruct_FieldsSetter._fbthrift_create(__fbthrift_inst._cpp_obj.get())
        for __fbthrift_name, _fbthrift_value in kwargs.items():
            __fbthrift_inst._fbthrift_set_field(__fbthrift_name, _fbthrift_value)
        return __fbthrift_inst

    cdef void _fbthrift_set_field(self, str name, object value) except *:
        self._fields_setter.set_field(name.encode("utf-8"), value)

    cdef object _fbthrift_isset(self):
        return _fbthrift_IsSet("FieldsInjectedToEmptyStruct", {
          "injected_field": deref(self._cpp_obj).injected_field_ref().has_value(),
        })

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cFieldsInjectedToEmptyStruct] cpp_obj):
        __fbthrift_inst = <FieldsInjectedToEmptyStruct>FieldsInjectedToEmptyStruct.__new__(FieldsInjectedToEmptyStruct)
        __fbthrift_inst._cpp_obj = cmove(cpp_obj)
        return __fbthrift_inst

    cdef inline injected_field_impl(self):

        return (<bytes>deref(self._cpp_obj).injected_field_ref().value()).decode('UTF-8')

    @property
    def injected_field(self):
        return self.injected_field_impl()


    def __hash__(FieldsInjectedToEmptyStruct self):
        return super().__hash__()

    def __repr__(FieldsInjectedToEmptyStruct self):
        return super().__repr__()

    def __str__(FieldsInjectedToEmptyStruct self):
        return super().__str__()


    def __copy__(FieldsInjectedToEmptyStruct self):
        cdef shared_ptr[cFieldsInjectedToEmptyStruct] cpp_obj = make_shared[cFieldsInjectedToEmptyStruct](
            deref(self._cpp_obj)
        )
        return FieldsInjectedToEmptyStruct._fbthrift_create(cmove(cpp_obj))

    def __richcmp__(self, other, int op):
        r = self._fbthrift_cmp_sametype(other, op)
        return __richcmp[cFieldsInjectedToEmptyStruct](
            self._cpp_obj,
            (<FieldsInjectedToEmptyStruct>other)._cpp_obj,
            op,
        ) if r is None else r

    @staticmethod
    def __get_reflection__():
        return _types_reflection.get_reflection__FieldsInjectedToEmptyStruct()

    @staticmethod
    def __get_metadata__():
        cdef __fbthrift_cThriftMetadata meta
        StructMetadata[cFieldsInjectedToEmptyStruct].gen(meta)
        return __MetadataBox.box(cmove(meta))

    @staticmethod
    def __get_thrift_name__():
        return "module.FieldsInjectedToEmptyStruct"

    @classmethod
    def _fbthrift_get_field_name_by_index(cls, idx):
        return __sv_to_str(__get_field_name_by_index[cFieldsInjectedToEmptyStruct](idx))

    @classmethod
    def _fbthrift_get_struct_size(cls):
        return 1

    cdef _fbthrift_iobuf.IOBuf _fbthrift_serialize(FieldsInjectedToEmptyStruct self, __Protocol proto):
        cdef unique_ptr[_fbthrift_iobuf.cIOBuf] data
        with nogil:
            data = cmove(serializer.cserialize[cFieldsInjectedToEmptyStruct](self._cpp_obj.get(), proto))
        return _fbthrift_iobuf.from_unique_ptr(cmove(data))

    cdef cuint32_t _fbthrift_deserialize(FieldsInjectedToEmptyStruct self, const _fbthrift_iobuf.cIOBuf* buf, __Protocol proto) except? 0:
        cdef cuint32_t needed
        self._cpp_obj = make_shared[cFieldsInjectedToEmptyStruct]()
        with nogil:
            needed = serializer.cdeserialize[cFieldsInjectedToEmptyStruct](buf, self._cpp_obj.get(), proto)
        return needed


    def _to_python(self):
        import importlib
        import thrift.python.converter
        python_types = importlib.import_module(
            "module.thrift_types"
        )
        return thrift.python.converter.to_python_struct(python_types.FieldsInjectedToEmptyStruct, self)

    def _to_py3(self):
        return self

    def _to_py_deprecated(self):
        import importlib
        import thrift.util.converter
        py_deprecated_types = importlib.import_module("module.ttypes")
        return thrift.util.converter.to_py_struct(py_deprecated_types.FieldsInjectedToEmptyStruct, self)
@__cython.auto_pickle(False)
cdef class FieldsInjectedToStruct(thrift.py3.types.Struct):
    def __init__(FieldsInjectedToStruct self, **kwargs):
        self._cpp_obj = make_shared[cFieldsInjectedToStruct]()
        self._fields_setter = _fbthrift_types_fields.__FieldsInjectedToStruct_FieldsSetter._fbthrift_create(self._cpp_obj.get())
        super().__init__(**kwargs)

    def __call__(FieldsInjectedToStruct self, **kwargs):
        if not kwargs:
            return self
        cdef FieldsInjectedToStruct __fbthrift_inst = FieldsInjectedToStruct.__new__(FieldsInjectedToStruct)
        __fbthrift_inst._cpp_obj = make_shared[cFieldsInjectedToStruct](deref(self._cpp_obj))
        __fbthrift_inst._fields_setter = _fbthrift_types_fields.__FieldsInjectedToStruct_FieldsSetter._fbthrift_create(__fbthrift_inst._cpp_obj.get())
        for __fbthrift_name, _fbthrift_value in kwargs.items():
            __fbthrift_inst._fbthrift_set_field(__fbthrift_name, _fbthrift_value)
        return __fbthrift_inst

    cdef void _fbthrift_set_field(self, str name, object value) except *:
        self._fields_setter.set_field(name.encode("utf-8"), value)

    cdef object _fbthrift_isset(self):
        return _fbthrift_IsSet("FieldsInjectedToStruct", {
          "string_field": deref(self._cpp_obj).string_field_ref().has_value(),
          "injected_field": deref(self._cpp_obj).injected_field_ref().has_value(),
        })

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cFieldsInjectedToStruct] cpp_obj):
        __fbthrift_inst = <FieldsInjectedToStruct>FieldsInjectedToStruct.__new__(FieldsInjectedToStruct)
        __fbthrift_inst._cpp_obj = cmove(cpp_obj)
        return __fbthrift_inst

    cdef inline string_field_impl(self):

        return (<bytes>deref(self._cpp_obj).string_field_ref().value()).decode('UTF-8')

    @property
    def string_field(self):
        return self.string_field_impl()

    cdef inline injected_field_impl(self):

        return (<bytes>deref(self._cpp_obj).injected_field_ref().value()).decode('UTF-8')

    @property
    def injected_field(self):
        return self.injected_field_impl()


    def __hash__(FieldsInjectedToStruct self):
        return super().__hash__()

    def __repr__(FieldsInjectedToStruct self):
        return super().__repr__()

    def __str__(FieldsInjectedToStruct self):
        return super().__str__()


    def __copy__(FieldsInjectedToStruct self):
        cdef shared_ptr[cFieldsInjectedToStruct] cpp_obj = make_shared[cFieldsInjectedToStruct](
            deref(self._cpp_obj)
        )
        return FieldsInjectedToStruct._fbthrift_create(cmove(cpp_obj))

    def __richcmp__(self, other, int op):
        r = self._fbthrift_cmp_sametype(other, op)
        return __richcmp[cFieldsInjectedToStruct](
            self._cpp_obj,
            (<FieldsInjectedToStruct>other)._cpp_obj,
            op,
        ) if r is None else r

    @staticmethod
    def __get_reflection__():
        return _types_reflection.get_reflection__FieldsInjectedToStruct()

    @staticmethod
    def __get_metadata__():
        cdef __fbthrift_cThriftMetadata meta
        StructMetadata[cFieldsInjectedToStruct].gen(meta)
        return __MetadataBox.box(cmove(meta))

    @staticmethod
    def __get_thrift_name__():
        return "module.FieldsInjectedToStruct"

    @classmethod
    def _fbthrift_get_field_name_by_index(cls, idx):
        return __sv_to_str(__get_field_name_by_index[cFieldsInjectedToStruct](idx))

    @classmethod
    def _fbthrift_get_struct_size(cls):
        return 2

    cdef _fbthrift_iobuf.IOBuf _fbthrift_serialize(FieldsInjectedToStruct self, __Protocol proto):
        cdef unique_ptr[_fbthrift_iobuf.cIOBuf] data
        with nogil:
            data = cmove(serializer.cserialize[cFieldsInjectedToStruct](self._cpp_obj.get(), proto))
        return _fbthrift_iobuf.from_unique_ptr(cmove(data))

    cdef cuint32_t _fbthrift_deserialize(FieldsInjectedToStruct self, const _fbthrift_iobuf.cIOBuf* buf, __Protocol proto) except? 0:
        cdef cuint32_t needed
        self._cpp_obj = make_shared[cFieldsInjectedToStruct]()
        with nogil:
            needed = serializer.cdeserialize[cFieldsInjectedToStruct](buf, self._cpp_obj.get(), proto)
        return needed


    def _to_python(self):
        import importlib
        import thrift.python.converter
        python_types = importlib.import_module(
            "module.thrift_types"
        )
        return thrift.python.converter.to_python_struct(python_types.FieldsInjectedToStruct, self)

    def _to_py3(self):
        return self

    def _to_py_deprecated(self):
        import importlib
        import thrift.util.converter
        py_deprecated_types = importlib.import_module("module.ttypes")
        return thrift.util.converter.to_py_struct(py_deprecated_types.FieldsInjectedToStruct, self)
@__cython.auto_pickle(False)
cdef class FieldsInjectedWithIncludedStruct(thrift.py3.types.Struct):
    def __init__(FieldsInjectedWithIncludedStruct self, **kwargs):
        self._cpp_obj = make_shared[cFieldsInjectedWithIncludedStruct]()
        self._fields_setter = _fbthrift_types_fields.__FieldsInjectedWithIncludedStruct_FieldsSetter._fbthrift_create(self._cpp_obj.get())
        super().__init__(**kwargs)

    def __call__(FieldsInjectedWithIncludedStruct self, **kwargs):
        if not kwargs:
            return self
        cdef FieldsInjectedWithIncludedStruct __fbthrift_inst = FieldsInjectedWithIncludedStruct.__new__(FieldsInjectedWithIncludedStruct)
        __fbthrift_inst._cpp_obj = make_shared[cFieldsInjectedWithIncludedStruct](deref(self._cpp_obj))
        __fbthrift_inst._fields_setter = _fbthrift_types_fields.__FieldsInjectedWithIncludedStruct_FieldsSetter._fbthrift_create(__fbthrift_inst._cpp_obj.get())
        for __fbthrift_name, _fbthrift_value in kwargs.items():
            __fbthrift_inst._fbthrift_set_field(__fbthrift_name, _fbthrift_value)
        return __fbthrift_inst

    cdef void _fbthrift_set_field(self, str name, object value) except *:
        self._fields_setter.set_field(name.encode("utf-8"), value)

    cdef object _fbthrift_isset(self):
        return _fbthrift_IsSet("FieldsInjectedWithIncludedStruct", {
          "string_field": deref(self._cpp_obj).string_field_ref().has_value(),
          "injected_field": deref(self._cpp_obj).injected_field_ref().has_value(),
          "injected_structured_annotation_field": deref(self._cpp_obj).injected_structured_annotation_field_ref().has_value(),
          "injected_unstructured_annotation_field": deref(self._cpp_obj).injected_unstructured_annotation_field_ref().has_value(),
        })

    @staticmethod
    cdef _fbthrift_create(shared_ptr[cFieldsInjectedWithIncludedStruct] cpp_obj):
        __fbthrift_inst = <FieldsInjectedWithIncludedStruct>FieldsInjectedWithIncludedStruct.__new__(FieldsInjectedWithIncludedStruct)
        __fbthrift_inst._cpp_obj = cmove(cpp_obj)
        return __fbthrift_inst

    cdef inline string_field_impl(self):

        return (<bytes>deref(self._cpp_obj).string_field_ref().value()).decode('UTF-8')

    @property
    def string_field(self):
        return self.string_field_impl()

    cdef inline injected_field_impl(self):

        return (<bytes>deref(self._cpp_obj).injected_field_ref().value()).decode('UTF-8')

    @property
    def injected_field(self):
        return self.injected_field_impl()

    cdef inline injected_structured_annotation_field_impl(self):
        if not deref(self._cpp_obj).injected_structured_annotation_field_ref().has_value():
            return None

        return (<bytes>deref(self._cpp_obj).injected_structured_annotation_field_ref().value()).decode('UTF-8')

    @property
    def injected_structured_annotation_field(self):
        return self.injected_structured_annotation_field_impl()

    cdef inline injected_unstructured_annotation_field_impl(self):
        if not deref(self._cpp_obj).injected_unstructured_annotation_field_ref().has_value():
            return None

        return (<bytes>deref(self._cpp_obj).injected_unstructured_annotation_field_ref().value()).decode('UTF-8')

    @property
    def injected_unstructured_annotation_field(self):
        return self.injected_unstructured_annotation_field_impl()


    def __hash__(FieldsInjectedWithIncludedStruct self):
        return super().__hash__()

    def __repr__(FieldsInjectedWithIncludedStruct self):
        return super().__repr__()

    def __str__(FieldsInjectedWithIncludedStruct self):
        return super().__str__()


    def __copy__(FieldsInjectedWithIncludedStruct self):
        cdef shared_ptr[cFieldsInjectedWithIncludedStruct] cpp_obj = make_shared[cFieldsInjectedWithIncludedStruct](
            deref(self._cpp_obj)
        )
        return FieldsInjectedWithIncludedStruct._fbthrift_create(cmove(cpp_obj))

    def __richcmp__(self, other, int op):
        r = self._fbthrift_cmp_sametype(other, op)
        return __richcmp[cFieldsInjectedWithIncludedStruct](
            self._cpp_obj,
            (<FieldsInjectedWithIncludedStruct>other)._cpp_obj,
            op,
        ) if r is None else r

    @staticmethod
    def __get_reflection__():
        return _types_reflection.get_reflection__FieldsInjectedWithIncludedStruct()

    @staticmethod
    def __get_metadata__():
        cdef __fbthrift_cThriftMetadata meta
        StructMetadata[cFieldsInjectedWithIncludedStruct].gen(meta)
        return __MetadataBox.box(cmove(meta))

    @staticmethod
    def __get_thrift_name__():
        return "module.FieldsInjectedWithIncludedStruct"

    @classmethod
    def _fbthrift_get_field_name_by_index(cls, idx):
        return __sv_to_str(__get_field_name_by_index[cFieldsInjectedWithIncludedStruct](idx))

    @classmethod
    def _fbthrift_get_struct_size(cls):
        return 4

    cdef _fbthrift_iobuf.IOBuf _fbthrift_serialize(FieldsInjectedWithIncludedStruct self, __Protocol proto):
        cdef unique_ptr[_fbthrift_iobuf.cIOBuf] data
        with nogil:
            data = cmove(serializer.cserialize[cFieldsInjectedWithIncludedStruct](self._cpp_obj.get(), proto))
        return _fbthrift_iobuf.from_unique_ptr(cmove(data))

    cdef cuint32_t _fbthrift_deserialize(FieldsInjectedWithIncludedStruct self, const _fbthrift_iobuf.cIOBuf* buf, __Protocol proto) except? 0:
        cdef cuint32_t needed
        self._cpp_obj = make_shared[cFieldsInjectedWithIncludedStruct]()
        with nogil:
            needed = serializer.cdeserialize[cFieldsInjectedWithIncludedStruct](buf, self._cpp_obj.get(), proto)
        return needed


    def _to_python(self):
        import importlib
        import thrift.python.converter
        python_types = importlib.import_module(
            "module.thrift_types"
        )
        return thrift.python.converter.to_python_struct(python_types.FieldsInjectedWithIncludedStruct, self)

    def _to_py3(self):
        return self

    def _to_py_deprecated(self):
        import importlib
        import thrift.util.converter
        py_deprecated_types = importlib.import_module("module.ttypes")
        return thrift.util.converter.to_py_struct(py_deprecated_types.FieldsInjectedWithIncludedStruct, self)
