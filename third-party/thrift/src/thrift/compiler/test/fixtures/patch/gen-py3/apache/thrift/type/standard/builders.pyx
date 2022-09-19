#
# Autogenerated by Thrift
#
# DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
#  @generated
#
cdef class UriStruct_Builder(thrift.py3.builder.StructBuilder):
    _struct_type = _apache_thrift_type_standard_types.UriStruct

    def __iter__(self):
        yield "scheme", self.scheme
        yield "domain", self.domain
        yield "path", self.path
        yield "query", self.query
        yield "fragment", self.fragment

cdef class TypeUri_Builder(thrift.py3.builder.StructBuilder):
    _struct_type = _apache_thrift_type_standard_types.TypeUri

    def __iter__(self):
        yield "uri", self.uri
        yield "typeHashPrefixSha2_256", self.typeHashPrefixSha2_256

cdef class TypeName_Builder(thrift.py3.builder.StructBuilder):
    _struct_type = _apache_thrift_type_standard_types.TypeName

    def __iter__(self):
        yield "boolType", self.boolType
        yield "byteType", self.byteType
        yield "i16Type", self.i16Type
        yield "i32Type", self.i32Type
        yield "i64Type", self.i64Type
        yield "floatType", self.floatType
        yield "doubleType", self.doubleType
        yield "stringType", self.stringType
        yield "binaryType", self.binaryType
        yield "enumType", self.enumType
        yield "structType", self.structType
        yield "unionType", self.unionType
        yield "exceptionType", self.exceptionType
        yield "listType", self.listType
        yield "setType", self.setType
        yield "mapType", self.mapType

