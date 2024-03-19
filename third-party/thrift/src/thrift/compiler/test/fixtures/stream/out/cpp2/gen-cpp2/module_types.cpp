/**
 * Autogenerated by Thrift for thrift/compiler/test/fixtures/stream/src/module.thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated @nocommit
 */
#include "thrift/compiler/test/fixtures/stream/gen-cpp2/module_types.h"
#include "thrift/compiler/test/fixtures/stream/gen-cpp2/module_types.tcc"

#include <thrift/lib/cpp2/gen/module_types_cpp.h>

#include "thrift/compiler/test/fixtures/stream/gen-cpp2/module_data.h"


namespace apache {
namespace thrift {
namespace detail {

void TccStructTraits<::cpp2::FooStreamEx>::translateFieldName(
    std::string_view _fname,
    int16_t& fid,
    apache::thrift::protocol::TType& _ftype) noexcept {
  using data = apache::thrift::TStructDataStorage<::cpp2::FooStreamEx>;
  static const st::translate_field_name_table table{
      data::fields_size,
      data::fields_names.data(),
      data::fields_ids.data(),
      data::fields_types.data()};
  st::translate_field_name(_fname, fid, _ftype, table);
}

} // namespace detail
} // namespace thrift
} // namespace apache

namespace cpp2 {

std::string_view FooStreamEx::__fbthrift_get_field_name(::apache::thrift::FieldOrdinal ord) {
  if (ord == ::apache::thrift::FieldOrdinal{0}) { return {}; }
  return apache::thrift::TStructDataStorage<FooStreamEx>::fields_names[folly::to_underlying(ord) - 1];
}
std::string_view FooStreamEx::__fbthrift_get_class_name() {
  return apache::thrift::TStructDataStorage<FooStreamEx>::name;
}

FooStreamEx::FooStreamEx(const FooStreamEx&) = default;
FooStreamEx& FooStreamEx::operator=(const FooStreamEx&) = default;
FooStreamEx::FooStreamEx() {
}


FooStreamEx::~FooStreamEx() {}

FooStreamEx::FooStreamEx([[maybe_unused]] FooStreamEx&& other) noexcept{}
FooStreamEx& FooStreamEx::operator=([[maybe_unused]] FooStreamEx&& other) noexcept {
    return *this;
}


FooStreamEx::FooStreamEx(apache::thrift::FragileConstructor) {}


void FooStreamEx::__fbthrift_clear() {
  // clear all fields
}

void FooStreamEx::__fbthrift_clear_terse_fields() {
}

bool FooStreamEx::__fbthrift_is_empty() const {
  return true;
}

bool FooStreamEx::operator==([[maybe_unused]] const FooStreamEx& rhs) const {
  return ::apache::thrift::op::detail::StructEquality{}(*this, rhs);
}

bool FooStreamEx::operator<([[maybe_unused]] const FooStreamEx& rhs) const {
  return ::apache::thrift::op::detail::StructLessThan{}(*this, rhs);
}


void swap([[maybe_unused]] FooStreamEx& a, [[maybe_unused]] FooStreamEx& b) {
  using ::std::swap;
}

template void FooStreamEx::readNoXfer<>(apache::thrift::BinaryProtocolReader*);
template uint32_t FooStreamEx::write<>(apache::thrift::BinaryProtocolWriter*) const;
template uint32_t FooStreamEx::serializedSize<>(apache::thrift::BinaryProtocolWriter const*) const;
template uint32_t FooStreamEx::serializedSizeZC<>(apache::thrift::BinaryProtocolWriter const*) const;
template void FooStreamEx::readNoXfer<>(apache::thrift::CompactProtocolReader*);
template uint32_t FooStreamEx::write<>(apache::thrift::CompactProtocolWriter*) const;
template uint32_t FooStreamEx::serializedSize<>(apache::thrift::CompactProtocolWriter const*) const;
template uint32_t FooStreamEx::serializedSizeZC<>(apache::thrift::CompactProtocolWriter const*) const;


} // cpp2

namespace apache {
namespace thrift {
namespace detail {

void TccStructTraits<::cpp2::FooEx>::translateFieldName(
    std::string_view _fname,
    int16_t& fid,
    apache::thrift::protocol::TType& _ftype) noexcept {
  using data = apache::thrift::TStructDataStorage<::cpp2::FooEx>;
  static const st::translate_field_name_table table{
      data::fields_size,
      data::fields_names.data(),
      data::fields_ids.data(),
      data::fields_types.data()};
  st::translate_field_name(_fname, fid, _ftype, table);
}

} // namespace detail
} // namespace thrift
} // namespace apache

namespace cpp2 {

std::string_view FooEx::__fbthrift_get_field_name(::apache::thrift::FieldOrdinal ord) {
  if (ord == ::apache::thrift::FieldOrdinal{0}) { return {}; }
  return apache::thrift::TStructDataStorage<FooEx>::fields_names[folly::to_underlying(ord) - 1];
}
std::string_view FooEx::__fbthrift_get_class_name() {
  return apache::thrift::TStructDataStorage<FooEx>::name;
}

FooEx::FooEx(const FooEx&) = default;
FooEx& FooEx::operator=(const FooEx&) = default;
FooEx::FooEx() {
}


FooEx::~FooEx() {}

FooEx::FooEx([[maybe_unused]] FooEx&& other) noexcept{}
FooEx& FooEx::operator=([[maybe_unused]] FooEx&& other) noexcept {
    return *this;
}


FooEx::FooEx(apache::thrift::FragileConstructor) {}


void FooEx::__fbthrift_clear() {
  // clear all fields
}

void FooEx::__fbthrift_clear_terse_fields() {
}

bool FooEx::__fbthrift_is_empty() const {
  return true;
}

bool FooEx::operator==([[maybe_unused]] const FooEx& rhs) const {
  return ::apache::thrift::op::detail::StructEquality{}(*this, rhs);
}

bool FooEx::operator<([[maybe_unused]] const FooEx& rhs) const {
  return ::apache::thrift::op::detail::StructLessThan{}(*this, rhs);
}


void swap([[maybe_unused]] FooEx& a, [[maybe_unused]] FooEx& b) {
  using ::std::swap;
}

template void FooEx::readNoXfer<>(apache::thrift::BinaryProtocolReader*);
template uint32_t FooEx::write<>(apache::thrift::BinaryProtocolWriter*) const;
template uint32_t FooEx::serializedSize<>(apache::thrift::BinaryProtocolWriter const*) const;
template uint32_t FooEx::serializedSizeZC<>(apache::thrift::BinaryProtocolWriter const*) const;
template void FooEx::readNoXfer<>(apache::thrift::CompactProtocolReader*);
template uint32_t FooEx::write<>(apache::thrift::CompactProtocolWriter*) const;
template uint32_t FooEx::serializedSize<>(apache::thrift::CompactProtocolWriter const*) const;
template uint32_t FooEx::serializedSizeZC<>(apache::thrift::CompactProtocolWriter const*) const;


} // cpp2

namespace apache {
namespace thrift {
namespace detail {

void TccStructTraits<::cpp2::FooEx2>::translateFieldName(
    std::string_view _fname,
    int16_t& fid,
    apache::thrift::protocol::TType& _ftype) noexcept {
  using data = apache::thrift::TStructDataStorage<::cpp2::FooEx2>;
  static const st::translate_field_name_table table{
      data::fields_size,
      data::fields_names.data(),
      data::fields_ids.data(),
      data::fields_types.data()};
  st::translate_field_name(_fname, fid, _ftype, table);
}

} // namespace detail
} // namespace thrift
} // namespace apache

namespace cpp2 {

std::string_view FooEx2::__fbthrift_get_field_name(::apache::thrift::FieldOrdinal ord) {
  if (ord == ::apache::thrift::FieldOrdinal{0}) { return {}; }
  return apache::thrift::TStructDataStorage<FooEx2>::fields_names[folly::to_underlying(ord) - 1];
}
std::string_view FooEx2::__fbthrift_get_class_name() {
  return apache::thrift::TStructDataStorage<FooEx2>::name;
}

FooEx2::FooEx2(const FooEx2&) = default;
FooEx2& FooEx2::operator=(const FooEx2&) = default;
FooEx2::FooEx2() {
}


FooEx2::~FooEx2() {}

FooEx2::FooEx2([[maybe_unused]] FooEx2&& other) noexcept{}
FooEx2& FooEx2::operator=([[maybe_unused]] FooEx2&& other) noexcept {
    return *this;
}


FooEx2::FooEx2(apache::thrift::FragileConstructor) {}


void FooEx2::__fbthrift_clear() {
  // clear all fields
}

void FooEx2::__fbthrift_clear_terse_fields() {
}

bool FooEx2::__fbthrift_is_empty() const {
  return true;
}

bool FooEx2::operator==([[maybe_unused]] const FooEx2& rhs) const {
  return ::apache::thrift::op::detail::StructEquality{}(*this, rhs);
}

bool FooEx2::operator<([[maybe_unused]] const FooEx2& rhs) const {
  return ::apache::thrift::op::detail::StructLessThan{}(*this, rhs);
}


void swap([[maybe_unused]] FooEx2& a, [[maybe_unused]] FooEx2& b) {
  using ::std::swap;
}

template void FooEx2::readNoXfer<>(apache::thrift::BinaryProtocolReader*);
template uint32_t FooEx2::write<>(apache::thrift::BinaryProtocolWriter*) const;
template uint32_t FooEx2::serializedSize<>(apache::thrift::BinaryProtocolWriter const*) const;
template uint32_t FooEx2::serializedSizeZC<>(apache::thrift::BinaryProtocolWriter const*) const;
template void FooEx2::readNoXfer<>(apache::thrift::CompactProtocolReader*);
template uint32_t FooEx2::write<>(apache::thrift::CompactProtocolWriter*) const;
template uint32_t FooEx2::serializedSize<>(apache::thrift::CompactProtocolWriter const*) const;
template uint32_t FooEx2::serializedSizeZC<>(apache::thrift::CompactProtocolWriter const*) const;


} // cpp2

namespace cpp2 { namespace {
[[maybe_unused]] FOLLY_ERASE void validateAdapters() {
}
}} // cpp2
namespace apache::thrift::detail::annotation {
}
