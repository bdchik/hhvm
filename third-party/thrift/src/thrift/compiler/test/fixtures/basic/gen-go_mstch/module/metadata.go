// @generated by Thrift for [[[ program path ]]]
// This file is probably not the place you want to edit!

package module // [[[ program thrift source path ]]]

import (
    thrift "github.com/facebook/fbthrift/thrift/lib/go/thrift"
    metadata "github.com/facebook/fbthrift/thrift/lib/thrift/metadata"
)

// (needed to ensure safety because of naive import list construction)
var _ = thrift.ZERO
var _ = metadata.GoUnusedProtection__

var structMetadatas = []*metadata.ThriftStruct{
    metadata.NewThriftStruct().
    SetName("module.MyStruct").
    SetIsUnion(false).
    SetFields(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("MyIntField").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_I64_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(2).
    SetName("MyStringField").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_STRING_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(3).
    SetName("MyDataField").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTStruct(
        metadata.NewThriftStructType().
    SetName("module.MyDataItem"),
    ),
    ),
            metadata.NewThriftField().
    SetId(4).
    SetName("myEnum").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTEnum(
        metadata.NewThriftEnumType().
            SetName("module.MyEnum"),
            ),
    ),
            metadata.NewThriftField().
    SetId(5).
    SetName("oneway").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_BOOL_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(6).
    SetName("readonly").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_BOOL_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(7).
    SetName("idempotent").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_BOOL_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(8).
    SetName("floatSet").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTSet(
        metadata.NewThriftSetType().
            SetValueType(metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_FLOAT_TYPE.Ptr(),
            )),
            ),
    ),
            metadata.NewThriftField().
    SetId(9).
    SetName("no_hack_codegen_field").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_STRING_TYPE.Ptr(),
            ),
    ),
        },
    ),
    metadata.NewThriftStruct().
    SetName("module.MyDataItem").
    SetIsUnion(false),
    metadata.NewThriftStruct().
    SetName("module.MyUnion").
    SetIsUnion(true).
    SetFields(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("myEnum").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTEnum(
        metadata.NewThriftEnumType().
            SetName("module.MyEnum"),
            ),
    ),
            metadata.NewThriftField().
    SetId(2).
    SetName("myStruct").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTStruct(
        metadata.NewThriftStructType().
            SetName("module.MyStruct"),
            ),
    ),
            metadata.NewThriftField().
    SetId(3).
    SetName("myDataItem").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTStruct(
        metadata.NewThriftStructType().
            SetName("module.MyDataItem"),
            ),
    ),
            metadata.NewThriftField().
    SetId(4).
    SetName("floatSet").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTSet(
        metadata.NewThriftSetType().
            SetValueType(metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_FLOAT_TYPE.Ptr(),
            )),
            ),
    ),
        },
    ),
    metadata.NewThriftStruct().
    SetName("module.ReservedKeyword").
    SetIsUnion(false).
    SetFields(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("reserved_field").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_I32_TYPE.Ptr(),
            ),
    ),
        },
    ),
    metadata.NewThriftStruct().
    SetName("module.UnionToBeRenamed").
    SetIsUnion(true).
    SetFields(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("reserved_field").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_I32_TYPE.Ptr(),
            ),
    ),
        },
    ),
}

var exceptionMetadatas = []*metadata.ThriftException{
}

var enumMetadatas = []*metadata.ThriftEnum{
    metadata.NewThriftEnum().
    SetName("module.MyEnum").
    SetElements(
        map[int32]string{
            0: "MyValue1",
            1: "MyValue2",
        },
    ),
    metadata.NewThriftEnum().
    SetName("module.HackEnum").
    SetElements(
        map[int32]string{
            0: "Value1",
            1: "Value2",
        },
    ),
}

var serviceMetadatas = []*metadata.ThriftService{
    metadata.NewThriftService().
    SetName("module.FooService").
    SetFunctions(
        []*metadata.ThriftFunction{
            metadata.NewThriftFunction().
    SetName("simple_rpc").
    SetIsOneway(false).
    SetReturnType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_VOID_TYPE.Ptr(),
            ),
    ),
        },
    ),
    metadata.NewThriftService().
    SetName("module.FB303Service").
    SetFunctions(
        []*metadata.ThriftFunction{
            metadata.NewThriftFunction().
    SetName("simple_rpc").
    SetIsOneway(false).
    SetReturnType(
        metadata.NewThriftType().
    SetTStruct(
        metadata.NewThriftStructType().
            SetName("module.ReservedKeyword"),
            ),
    ).
    SetArguments(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("int_parameter").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_I32_TYPE.Ptr(),
            ),
    ),
        },
    ),
        },
    ),
    metadata.NewThriftService().
    SetName("module.MyService").
    SetFunctions(
        []*metadata.ThriftFunction{
            metadata.NewThriftFunction().
    SetName("ping").
    SetIsOneway(false).
    SetReturnType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_VOID_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftFunction().
    SetName("getRandomData").
    SetIsOneway(false).
    SetReturnType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_STRING_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftFunction().
    SetName("sink").
    SetIsOneway(false).
    SetReturnType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_VOID_TYPE.Ptr(),
            ),
    ).
    SetArguments(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("sink").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_I64_TYPE.Ptr(),
            ),
    ),
        },
    ),
            metadata.NewThriftFunction().
    SetName("putDataById").
    SetIsOneway(false).
    SetReturnType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_VOID_TYPE.Ptr(),
            ),
    ).
    SetArguments(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("id").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_I64_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(2).
    SetName("data").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_STRING_TYPE.Ptr(),
            ),
    ),
        },
    ),
            metadata.NewThriftFunction().
    SetName("hasDataById").
    SetIsOneway(false).
    SetReturnType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_BOOL_TYPE.Ptr(),
            ),
    ).
    SetArguments(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("id").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_I64_TYPE.Ptr(),
            ),
    ),
        },
    ),
            metadata.NewThriftFunction().
    SetName("getDataById").
    SetIsOneway(false).
    SetReturnType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_STRING_TYPE.Ptr(),
            ),
    ).
    SetArguments(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("id").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_I64_TYPE.Ptr(),
            ),
    ),
        },
    ),
            metadata.NewThriftFunction().
    SetName("deleteDataById").
    SetIsOneway(false).
    SetReturnType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_VOID_TYPE.Ptr(),
            ),
    ).
    SetArguments(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("id").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_I64_TYPE.Ptr(),
            ),
    ),
        },
    ),
            metadata.NewThriftFunction().
    SetName("lobDataById").
    SetIsOneway(true).
    SetReturnType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_VOID_TYPE.Ptr(),
            ),
    ).
    SetArguments(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("id").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_I64_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(2).
    SetName("data").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_STRING_TYPE.Ptr(),
            ),
    ),
        },
    ),
            metadata.NewThriftFunction().
    SetName("invalid_return_for_hack").
    SetIsOneway(false).
    SetReturnType(
        metadata.NewThriftType().
    SetTSet(
        metadata.NewThriftSetType().
            SetValueType(metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_FLOAT_TYPE.Ptr(),
            )),
            ),
    ),
            metadata.NewThriftFunction().
    SetName("rpc_skipped_codegen").
    SetIsOneway(false).
    SetReturnType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_VOID_TYPE.Ptr(),
            ),
    ),
        },
    ),
    metadata.NewThriftService().
    SetName("module.DbMixedStackArguments").
    SetFunctions(
        []*metadata.ThriftFunction{
            metadata.NewThriftFunction().
    SetName("getDataByKey0").
    SetIsOneway(false).
    SetReturnType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_BINARY_TYPE.Ptr(),
            ),
    ).
    SetArguments(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("key").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_STRING_TYPE.Ptr(),
            ),
    ),
        },
    ),
            metadata.NewThriftFunction().
    SetName("getDataByKey1").
    SetIsOneway(false).
    SetReturnType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_BINARY_TYPE.Ptr(),
            ),
    ).
    SetArguments(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("key").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_STRING_TYPE.Ptr(),
            ),
    ),
        },
    ),
        },
    ),
}

// GetThriftMetadata returns complete Thrift metadata for current and imported packages.
func GetThriftMetadata() *metadata.ThriftMetadata {
    includedEnumsMetadatas := []map[string]*metadata.ThriftEnum{
        GetEnumsMetadata(),
    }
    includedStructsMetadatas := []map[string]*metadata.ThriftStruct{
        GetStructsMetadata(),
    }
    includedExceptionsMetadatas := []map[string]*metadata.ThriftException{
        GetExceptionsMetadata(),
    }
    includedServicesMetadatas := []map[string]*metadata.ThriftService{
        GetServicesMetadata(),
    }

	allEnums := make(map[string]*metadata.ThriftEnum)
	allStructs := make(map[string]*metadata.ThriftStruct)
	allExceptions := make(map[string]*metadata.ThriftException)
    allServices := make(map[string]*metadata.ThriftService)

    for _, includedEnumsMetadata := range includedEnumsMetadatas {
        for enumName, thriftEnum := range includedEnumsMetadata {
            allEnums[enumName] = thriftEnum
        }
    }
    for _, includedStructsMetadata := range includedStructsMetadatas {
        for structName, thriftStruct := range includedStructsMetadata {
            allStructs[structName] = thriftStruct
        }
    }
    for _, includedExceptionsMetadata := range includedExceptionsMetadatas {
        for exceptionName, thriftException := range includedExceptionsMetadata {
            allExceptions[exceptionName] = thriftException
        }
    }
    for _, includedServicesMetadata := range includedServicesMetadatas {
        for serviceName, thriftService := range includedServicesMetadata {
            allServices[serviceName] = thriftService
        }
    }

    return metadata.NewThriftMetadata().
        SetEnums(allEnums).
        SetStructs(allStructs).
        SetExceptions(allExceptions).
        SetServices(allServices)
}

// GetStructsMetadata returns Thrift metadata for enums in the current package.
func GetEnumsMetadata() map[string]*metadata.ThriftEnum {
    result := make(map[string]*metadata.ThriftEnum)
    for _, enumMetadata := range enumMetadatas {
        result[enumMetadata.GetName()] = enumMetadata
    }
    return result
}

// GetStructsMetadata returns Thrift metadata for structs in the current package.
func GetStructsMetadata() map[string]*metadata.ThriftStruct {
    result := make(map[string]*metadata.ThriftStruct)
    for _, structMetadata := range structMetadatas {
        result[structMetadata.GetName()] = structMetadata
    }
    return result
}

// GetStructsMetadata returns Thrift metadata for exceptions in the current package.
func GetExceptionsMetadata() map[string]*metadata.ThriftException {
    result := make(map[string]*metadata.ThriftException)
    for _, exceptionMetadata := range exceptionMetadatas {
        result[exceptionMetadata.GetName()] = exceptionMetadata
    }
    return result
}

// GetStructsMetadata returns Thrift metadata for services in the current package.
func GetServicesMetadata() map[string]*metadata.ThriftService {
    result := make(map[string]*metadata.ThriftService)
    for _, serviceMetadata := range serviceMetadatas {
        result[serviceMetadata.GetName()] = serviceMetadata
    }
    return result
}
