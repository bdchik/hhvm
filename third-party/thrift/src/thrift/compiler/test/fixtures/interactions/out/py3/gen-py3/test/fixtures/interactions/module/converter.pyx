
#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#



cdef shared_ptr[_fbthrift_ctypes.cCustomException] CustomException_convert_to_cpp(object inst):
    return (<_fbthrift_ctypes.CustomException?>inst)._cpp_obj


cdef object CustomException_from_cpp(const shared_ptr[_fbthrift_ctypes.cCustomException]& c_struct):
    return _fbthrift_ctypes.CustomException._fbthrift_create(c_struct)
