#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#

from apache.thrift.metadata.types cimport cThriftMetadata
from thrift.py3.common cimport (
    cThriftMetadata as __fbthrift_cThriftMetadata,
)

cdef extern from "thrift/compiler/test/fixtures/transitive-deps/gen-py3/b/metadata.h" :
    cdef cThriftMetadata cGetThriftModuleMetadata "::cpp2::b_getThriftModuleMetadata"()
