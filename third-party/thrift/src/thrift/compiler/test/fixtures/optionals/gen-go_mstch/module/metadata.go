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
    SetName("module.Color").
    SetIsUnion(false).
    SetFields(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("red").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_DOUBLE_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(2).
    SetName("green").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_DOUBLE_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(3).
    SetName("blue").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_DOUBLE_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(4).
    SetName("alpha").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_DOUBLE_TYPE.Ptr(),
            ),
    ),
        },
    ),
    metadata.NewThriftStruct().
    SetName("module.Vehicle").
    SetIsUnion(false).
    SetFields(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("color").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTStruct(
        metadata.NewThriftStructType().
            SetName("module.Color"),
            ),
    ),
            metadata.NewThriftField().
    SetId(2).
    SetName("licensePlate").
    SetIsOptional(true).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_STRING_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(3).
    SetName("description").
    SetIsOptional(true).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_STRING_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(4).
    SetName("name").
    SetIsOptional(true).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_STRING_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(5).
    SetName("hasAC").
    SetIsOptional(true).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_BOOL_TYPE.Ptr(),
            ),
    ),
        },
    ),
    metadata.NewThriftStruct().
    SetName("module.Person").
    SetIsUnion(false).
    SetFields(
        []*metadata.ThriftField{
            metadata.NewThriftField().
    SetId(1).
    SetName("id").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTTypedef(
        metadata.NewThriftTypedefType().
            SetName("module.PersonID").
            SetUnderlyingType(
                metadata.NewThriftType().
            SetTPrimitive(
                metadata.ThriftPrimitiveType_THRIFT_I64_TYPE.Ptr(),
            )    ),
            ),
    ),
            metadata.NewThriftField().
    SetId(2).
    SetName("name").
    SetIsOptional(false).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_STRING_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(3).
    SetName("age").
    SetIsOptional(true).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_I16_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(4).
    SetName("address").
    SetIsOptional(true).
    SetType(
        metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_STRING_TYPE.Ptr(),
            ),
    ),
            metadata.NewThriftField().
    SetId(5).
    SetName("favoriteColor").
    SetIsOptional(true).
    SetType(
        metadata.NewThriftType().
    SetTStruct(
        metadata.NewThriftStructType().
            SetName("module.Color"),
            ),
    ),
            metadata.NewThriftField().
    SetId(6).
    SetName("friends").
    SetIsOptional(true).
    SetType(
        metadata.NewThriftType().
    SetTSet(
        metadata.NewThriftSetType().
            SetValueType(metadata.NewThriftType().
    SetTTypedef(
        metadata.NewThriftTypedefType().
            SetName("module.PersonID").
            SetUnderlyingType(
                metadata.NewThriftType().
            SetTPrimitive(
                metadata.ThriftPrimitiveType_THRIFT_I64_TYPE.Ptr(),
            )    ),
            )),
            ),
    ),
            metadata.NewThriftField().
    SetId(7).
    SetName("bestFriend").
    SetIsOptional(true).
    SetType(
        metadata.NewThriftType().
    SetTTypedef(
        metadata.NewThriftTypedefType().
            SetName("module.PersonID").
            SetUnderlyingType(
                metadata.NewThriftType().
            SetTPrimitive(
                metadata.ThriftPrimitiveType_THRIFT_I64_TYPE.Ptr(),
            )    ),
            ),
    ),
            metadata.NewThriftField().
    SetId(8).
    SetName("petNames").
    SetIsOptional(true).
    SetType(
        metadata.NewThriftType().
    SetTMap(
        metadata.NewThriftMapType().
            SetKeyType(metadata.NewThriftType().
    SetTEnum(
        metadata.NewThriftEnumType().
            SetName("module.Animal"),
            )).
            SetValueType(metadata.NewThriftType().
    SetTPrimitive(
        metadata.ThriftPrimitiveType_THRIFT_STRING_TYPE.Ptr(),
            )),
            ),
    ),
            metadata.NewThriftField().
    SetId(9).
    SetName("afraidOfAnimal").
    SetIsOptional(true).
    SetType(
        metadata.NewThriftType().
    SetTEnum(
        metadata.NewThriftEnumType().
            SetName("module.Animal"),
            ),
    ),
            metadata.NewThriftField().
    SetId(10).
    SetName("vehicles").
    SetIsOptional(true).
    SetType(
        metadata.NewThriftType().
    SetTList(
        metadata.NewThriftListType().
            SetValueType(metadata.NewThriftType().
    SetTStruct(
        metadata.NewThriftStructType().
            SetName("module.Vehicle"),
            )),
            ),
    ),
        },
    ),
}

var exceptionMetadatas = []*metadata.ThriftException{
}

var enumMetadatas = []*metadata.ThriftEnum{
    metadata.NewThriftEnum().
    SetName("module.Animal").
    SetElements(
        map[int32]string{
            1: "DOG",
            2: "CAT",
            3: "TARANTULA",
        },
    ),
}

var serviceMetadatas = []*metadata.ThriftService{
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
