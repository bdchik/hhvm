#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#

from libcpp.memory cimport shared_ptr

cimport module.types as _fbthrift_ctypes


cdef shared_ptr[_fbthrift_ctypes.cFooStreamEx] FooStreamEx_convert_to_cpp(object inst)
cdef object FooStreamEx_from_cpp(const shared_ptr[_fbthrift_ctypes.cFooStreamEx]& c_struct)

cdef shared_ptr[_fbthrift_ctypes.cFooEx] FooEx_convert_to_cpp(object inst)
cdef object FooEx_from_cpp(const shared_ptr[_fbthrift_ctypes.cFooEx]& c_struct)

cdef shared_ptr[_fbthrift_ctypes.cFooEx2] FooEx2_convert_to_cpp(object inst)
cdef object FooEx2_from_cpp(const shared_ptr[_fbthrift_ctypes.cFooEx2]& c_struct)

