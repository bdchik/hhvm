#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#

from libcpp.memory cimport shared_ptr

cimport test.fixtures.interactions.module.types as _fbthrift_ctypes


cdef shared_ptr[_fbthrift_ctypes.cCustomException] CustomException_convert_to_cpp(object inst)
cdef object CustomException_from_cpp(const shared_ptr[_fbthrift_ctypes.cCustomException]& c_struct)

