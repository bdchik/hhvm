
/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT
 *  @generated
 *
 */

#pragma once

#include <thrift/lib/python/capi/constructor.h>
#include <thrift/lib/python/capi/extractor.h>

#include <thrift/compiler/test/fixtures/includes/gen-cpp2/module_types.h>

namespace apache {
namespace thrift {
namespace python {
namespace capi {
template <>
struct Extractor<::cpp2::MyStruct>
    : public BaseExtractor<::cpp2::MyStruct> {
  ExtractorResult<::cpp2::MyStruct> operator()(PyObject* obj);
  int typeCheck(PyObject* obj);
};

template <>
struct Extractor<::apache::thrift::python::capi::ComposedStruct<
        ::cpp2::MyStruct>>
    : public BaseExtractor<::apache::thrift::python::capi::ComposedStruct<
        ::cpp2::MyStruct>> {
  ExtractorResult<::cpp2::MyStruct> operator()(PyObject* obj);
};

template <>
struct Constructor<::cpp2::MyStruct>
    : public BaseConstructor<::cpp2::MyStruct> {
  PyObject* operator()(const ::cpp2::MyStruct& val);
};

template <>
struct Constructor<::apache::thrift::python::capi::ComposedStruct<
        ::cpp2::MyStruct>>
    : public BaseConstructor<::apache::thrift::python::capi::ComposedStruct<
        ::cpp2::MyStruct>> {
  PyObject* operator()(const ::cpp2::MyStruct& val);
};

} // namespace capi
} // namespace python
} // namespace thrift
} // namespace apache
