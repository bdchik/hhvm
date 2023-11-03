// @generated by Thrift for [[[ program path ]]]
// This file is probably not the place you want to edit!

package module // [[[ program thrift source path ]]]

import (
    "fmt"
    "strings"

    thrift "github.com/facebook/fbthrift/thrift/lib/go/thrift"
)

// (needed to ensure safety because of naive import list construction)
var _ = fmt.Printf
var _ = strings.Split
var _ = thrift.ZERO


type Foo struct {
    Field2 int32 `thrift:"field2,1" json:"field2" db:"field2"`
    Field3 int32 `thrift:"field3,2" json:"field3" db:"field3"`
    Field1 int32 `thrift:"field1,3" json:"field1" db:"field1"`
}
// Compile time interface enforcer
var _ thrift.Struct = &Foo{}

func NewFoo() *Foo {
    return (&Foo{}).
        SetField2NonCompat(0).
        SetField3NonCompat(0).
        SetField1NonCompat(0)
}

func (x *Foo) GetField2NonCompat() int32 {
    return x.Field2
}

func (x *Foo) GetField2() int32 {
    return x.Field2
}

func (x *Foo) GetField3NonCompat() int32 {
    return x.Field3
}

func (x *Foo) GetField3() int32 {
    return x.Field3
}

func (x *Foo) GetField1NonCompat() int32 {
    return x.Field1
}

func (x *Foo) GetField1() int32 {
    return x.Field1
}

func (x *Foo) SetField2NonCompat(value int32) *Foo {
    x.Field2 = value
    return x
}

func (x *Foo) SetField2(value int32) *Foo {
    x.Field2 = value
    return x
}

func (x *Foo) SetField3NonCompat(value int32) *Foo {
    x.Field3 = value
    return x
}

func (x *Foo) SetField3(value int32) *Foo {
    x.Field3 = value
    return x
}

func (x *Foo) SetField1NonCompat(value int32) *Foo {
    x.Field1 = value
    return x
}

func (x *Foo) SetField1(value int32) *Foo {
    x.Field1 = value
    return x
}

func (x *Foo) writeField1(p thrift.Protocol) error {  // Field2
    if err := p.WriteFieldBegin("field2", thrift.I32, 1); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field begin error: ", x), err)
    }

    item := x.GetField2NonCompat()
    if err := p.WriteI32(item); err != nil {
    return err
}

    if err := p.WriteFieldEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field end error: ", x), err)
    }
    return nil
}

func (x *Foo) writeField2(p thrift.Protocol) error {  // Field3
    if err := p.WriteFieldBegin("field3", thrift.I32, 2); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field begin error: ", x), err)
    }

    item := x.GetField3NonCompat()
    if err := p.WriteI32(item); err != nil {
    return err
}

    if err := p.WriteFieldEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field end error: ", x), err)
    }
    return nil
}

func (x *Foo) writeField3(p thrift.Protocol) error {  // Field1
    if err := p.WriteFieldBegin("field1", thrift.I32, 3); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field begin error: ", x), err)
    }

    item := x.GetField1NonCompat()
    if err := p.WriteI32(item); err != nil {
    return err
}

    if err := p.WriteFieldEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field end error: ", x), err)
    }
    return nil
}

func (x *Foo) readField1(p thrift.Protocol) error {  // Field2
    result, err := p.ReadI32()
if err != nil {
    return err
}

    x.SetField2NonCompat(result)
    return nil
}

func (x *Foo) readField2(p thrift.Protocol) error {  // Field3
    result, err := p.ReadI32()
if err != nil {
    return err
}

    x.SetField3NonCompat(result)
    return nil
}

func (x *Foo) readField3(p thrift.Protocol) error {  // Field1
    result, err := p.ReadI32()
if err != nil {
    return err
}

    x.SetField1NonCompat(result)
    return nil
}

func (x *Foo) toString1() string {  // Field2
    return fmt.Sprintf("%v", x.GetField2NonCompat())
}

func (x *Foo) toString2() string {  // Field3
    return fmt.Sprintf("%v", x.GetField3NonCompat())
}

func (x *Foo) toString3() string {  // Field1
    return fmt.Sprintf("%v", x.GetField1NonCompat())
}


// Deprecated: Use "New" constructor and setters to build your structs.
// e.g NewFoo().Set<FieldNameFoo>().Set<FieldNameBar>()
type FooBuilder struct {
    obj *Foo
}

// Deprecated: Use "New" constructor and setters to build your structs.
// e.g NewFoo().Set<FieldNameFoo>().Set<FieldNameBar>()
func NewFooBuilder() *FooBuilder {
    return &FooBuilder{
        obj: NewFoo(),
    }
}

// Deprecated: Use "New" constructor and setters to build your structs.
// e.g NewFoo().Set<FieldNameFoo>().Set<FieldNameBar>()
func (x *FooBuilder) Field2(value int32) *FooBuilder {
    x.obj.Field2 = value
    return x
}

// Deprecated: Use "New" constructor and setters to build your structs.
// e.g NewFoo().Set<FieldNameFoo>().Set<FieldNameBar>()
func (x *FooBuilder) Field3(value int32) *FooBuilder {
    x.obj.Field3 = value
    return x
}

// Deprecated: Use "New" constructor and setters to build your structs.
// e.g NewFoo().Set<FieldNameFoo>().Set<FieldNameBar>()
func (x *FooBuilder) Field1(value int32) *FooBuilder {
    x.obj.Field1 = value
    return x
}

// Deprecated: Use "New" constructor and setters to build your structs.
// e.g NewFoo().Set<FieldNameFoo>().Set<FieldNameBar>()
func (x *FooBuilder) Emit() *Foo {
    var objCopy Foo = *x.obj
    return &objCopy
}

func (x *Foo) Write(p thrift.Protocol) error {
    if err := p.WriteStructBegin("Foo"); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct begin error: ", x), err)
    }

    if err := x.writeField1(p); err != nil {
        return err
    }

    if err := x.writeField2(p); err != nil {
        return err
    }

    if err := x.writeField3(p); err != nil {
        return err
    }

    if err := p.WriteFieldStop(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field stop error: ", x), err)
    }

    if err := p.WriteStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct end error: ", x), err)
    }
    return nil
}

func (x *Foo) Read(p thrift.Protocol) error {
    if _, err := p.ReadStructBegin(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T read error: ", x), err)
    }

    for {
        _, wireType, id, err := p.ReadFieldBegin()
        if err != nil {
            return thrift.PrependError(fmt.Sprintf("%T field %d read error: ", x, id), err)
        }

        if wireType == thrift.STOP {
            break;
        }

        switch {
        case (id == 1 && wireType == thrift.Type(thrift.I32)):  // field2
            if err := x.readField1(p); err != nil {
                return err
            }
        case (id == 2 && wireType == thrift.Type(thrift.I32)):  // field3
            if err := x.readField2(p); err != nil {
                return err
            }
        case (id == 3 && wireType == thrift.Type(thrift.I32)):  // field1
            if err := x.readField3(p); err != nil {
                return err
            }
        default:
            if err := p.Skip(wireType); err != nil {
                return err
            }
        }

        if err := p.ReadFieldEnd(); err != nil {
            return err
        }
    }

    if err := p.ReadStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T read struct end error: ", x), err)
    }

    return nil
}

func (x *Foo) String() string {
    if x == nil {
        return "<nil>"
    }

    var sb strings.Builder

    sb.WriteString("Foo({")
    sb.WriteString(fmt.Sprintf("Field2:%s ", x.toString1()))
    sb.WriteString(fmt.Sprintf("Field3:%s ", x.toString2()))
    sb.WriteString(fmt.Sprintf("Field1:%s", x.toString3()))
    sb.WriteString("})")

    return sb.String()
}

type Foo2 struct {
    Field2 int32 `thrift:"field2,1" json:"field2" db:"field2"`
    Field3 int32 `thrift:"field3,2" json:"field3" db:"field3"`
    Field1 int32 `thrift:"field1,3" json:"field1" db:"field1"`
}
// Compile time interface enforcer
var _ thrift.Struct = &Foo2{}

func NewFoo2() *Foo2 {
    return (&Foo2{}).
        SetField2NonCompat(0).
        SetField3NonCompat(0).
        SetField1NonCompat(0)
}

func (x *Foo2) GetField2NonCompat() int32 {
    return x.Field2
}

func (x *Foo2) GetField2() int32 {
    return x.Field2
}

func (x *Foo2) GetField3NonCompat() int32 {
    return x.Field3
}

func (x *Foo2) GetField3() int32 {
    return x.Field3
}

func (x *Foo2) GetField1NonCompat() int32 {
    return x.Field1
}

func (x *Foo2) GetField1() int32 {
    return x.Field1
}

func (x *Foo2) SetField2NonCompat(value int32) *Foo2 {
    x.Field2 = value
    return x
}

func (x *Foo2) SetField2(value int32) *Foo2 {
    x.Field2 = value
    return x
}

func (x *Foo2) SetField3NonCompat(value int32) *Foo2 {
    x.Field3 = value
    return x
}

func (x *Foo2) SetField3(value int32) *Foo2 {
    x.Field3 = value
    return x
}

func (x *Foo2) SetField1NonCompat(value int32) *Foo2 {
    x.Field1 = value
    return x
}

func (x *Foo2) SetField1(value int32) *Foo2 {
    x.Field1 = value
    return x
}

func (x *Foo2) writeField1(p thrift.Protocol) error {  // Field2
    if err := p.WriteFieldBegin("field2", thrift.I32, 1); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field begin error: ", x), err)
    }

    item := x.GetField2NonCompat()
    if err := p.WriteI32(item); err != nil {
    return err
}

    if err := p.WriteFieldEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field end error: ", x), err)
    }
    return nil
}

func (x *Foo2) writeField2(p thrift.Protocol) error {  // Field3
    if err := p.WriteFieldBegin("field3", thrift.I32, 2); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field begin error: ", x), err)
    }

    item := x.GetField3NonCompat()
    if err := p.WriteI32(item); err != nil {
    return err
}

    if err := p.WriteFieldEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field end error: ", x), err)
    }
    return nil
}

func (x *Foo2) writeField3(p thrift.Protocol) error {  // Field1
    if err := p.WriteFieldBegin("field1", thrift.I32, 3); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field begin error: ", x), err)
    }

    item := x.GetField1NonCompat()
    if err := p.WriteI32(item); err != nil {
    return err
}

    if err := p.WriteFieldEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field end error: ", x), err)
    }
    return nil
}

func (x *Foo2) readField1(p thrift.Protocol) error {  // Field2
    result, err := p.ReadI32()
if err != nil {
    return err
}

    x.SetField2NonCompat(result)
    return nil
}

func (x *Foo2) readField2(p thrift.Protocol) error {  // Field3
    result, err := p.ReadI32()
if err != nil {
    return err
}

    x.SetField3NonCompat(result)
    return nil
}

func (x *Foo2) readField3(p thrift.Protocol) error {  // Field1
    result, err := p.ReadI32()
if err != nil {
    return err
}

    x.SetField1NonCompat(result)
    return nil
}

func (x *Foo2) toString1() string {  // Field2
    return fmt.Sprintf("%v", x.GetField2NonCompat())
}

func (x *Foo2) toString2() string {  // Field3
    return fmt.Sprintf("%v", x.GetField3NonCompat())
}

func (x *Foo2) toString3() string {  // Field1
    return fmt.Sprintf("%v", x.GetField1NonCompat())
}


// Deprecated: Use "New" constructor and setters to build your structs.
// e.g NewFoo2().Set<FieldNameFoo>().Set<FieldNameBar>()
type Foo2Builder struct {
    obj *Foo2
}

// Deprecated: Use "New" constructor and setters to build your structs.
// e.g NewFoo2().Set<FieldNameFoo>().Set<FieldNameBar>()
func NewFoo2Builder() *Foo2Builder {
    return &Foo2Builder{
        obj: NewFoo2(),
    }
}

// Deprecated: Use "New" constructor and setters to build your structs.
// e.g NewFoo2().Set<FieldNameFoo>().Set<FieldNameBar>()
func (x *Foo2Builder) Field2(value int32) *Foo2Builder {
    x.obj.Field2 = value
    return x
}

// Deprecated: Use "New" constructor and setters to build your structs.
// e.g NewFoo2().Set<FieldNameFoo>().Set<FieldNameBar>()
func (x *Foo2Builder) Field3(value int32) *Foo2Builder {
    x.obj.Field3 = value
    return x
}

// Deprecated: Use "New" constructor and setters to build your structs.
// e.g NewFoo2().Set<FieldNameFoo>().Set<FieldNameBar>()
func (x *Foo2Builder) Field1(value int32) *Foo2Builder {
    x.obj.Field1 = value
    return x
}

// Deprecated: Use "New" constructor and setters to build your structs.
// e.g NewFoo2().Set<FieldNameFoo>().Set<FieldNameBar>()
func (x *Foo2Builder) Emit() *Foo2 {
    var objCopy Foo2 = *x.obj
    return &objCopy
}

func (x *Foo2) Write(p thrift.Protocol) error {
    if err := p.WriteStructBegin("Foo2"); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct begin error: ", x), err)
    }

    if err := x.writeField1(p); err != nil {
        return err
    }

    if err := x.writeField2(p); err != nil {
        return err
    }

    if err := x.writeField3(p); err != nil {
        return err
    }

    if err := p.WriteFieldStop(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field stop error: ", x), err)
    }

    if err := p.WriteStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct end error: ", x), err)
    }
    return nil
}

func (x *Foo2) Read(p thrift.Protocol) error {
    if _, err := p.ReadStructBegin(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T read error: ", x), err)
    }

    for {
        _, wireType, id, err := p.ReadFieldBegin()
        if err != nil {
            return thrift.PrependError(fmt.Sprintf("%T field %d read error: ", x, id), err)
        }

        if wireType == thrift.STOP {
            break;
        }

        switch {
        case (id == 1 && wireType == thrift.Type(thrift.I32)):  // field2
            if err := x.readField1(p); err != nil {
                return err
            }
        case (id == 2 && wireType == thrift.Type(thrift.I32)):  // field3
            if err := x.readField2(p); err != nil {
                return err
            }
        case (id == 3 && wireType == thrift.Type(thrift.I32)):  // field1
            if err := x.readField3(p); err != nil {
                return err
            }
        default:
            if err := p.Skip(wireType); err != nil {
                return err
            }
        }

        if err := p.ReadFieldEnd(); err != nil {
            return err
        }
    }

    if err := p.ReadStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T read struct end error: ", x), err)
    }

    return nil
}

func (x *Foo2) String() string {
    if x == nil {
        return "<nil>"
    }

    var sb strings.Builder

    sb.WriteString("Foo2({")
    sb.WriteString(fmt.Sprintf("Field2:%s ", x.toString1()))
    sb.WriteString(fmt.Sprintf("Field3:%s ", x.toString2()))
    sb.WriteString(fmt.Sprintf("Field1:%s", x.toString3()))
    sb.WriteString("})")

    return sb.String()
}

// RegisterTypes registers types found in this file that have a thrift_uri with the passed in registry.
func RegisterTypes(registry interface {
	  RegisterType(name string, initializer func() any)
}) {

}
