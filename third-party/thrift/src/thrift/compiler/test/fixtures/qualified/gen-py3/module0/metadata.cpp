/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

#include "thrift/compiler/test/fixtures/qualified/gen-py3/module0/metadata.h"

namespace module0 {
::apache::thrift::metadata::ThriftMetadata module0_getThriftModuleMetadata() {
  ::apache::thrift::metadata::ThriftServiceMetadataResponse response;
  ::apache::thrift::metadata::ThriftMetadata& metadata = *response.metadata_ref();
  ::apache::thrift::detail::md::EnumMetadata<Enum>::gen(metadata);
  ::apache::thrift::detail::md::StructMetadata<Struct>::gen(metadata);
  return metadata;
}
} // namespace module0
