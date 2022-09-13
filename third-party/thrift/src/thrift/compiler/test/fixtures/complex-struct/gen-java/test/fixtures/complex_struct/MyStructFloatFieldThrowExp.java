/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

package test.fixtures.complex_struct;

import com.facebook.swift.codec.*;
import com.facebook.swift.codec.ThriftField.Requiredness;
import com.facebook.swift.codec.ThriftField.Recursiveness;
import com.google.common.collect.*;
import java.util.*;
import javax.annotation.Nullable;
import org.apache.thrift.*;
import org.apache.thrift.transport.*;
import org.apache.thrift.protocol.*;
import static com.google.common.base.MoreObjects.toStringHelper;
import static com.google.common.base.MoreObjects.ToStringHelper;

@SwiftGenerated
@com.facebook.swift.codec.ThriftStruct(value="MyStructFloatFieldThrowExp", builder=MyStructFloatFieldThrowExp.Builder.class)
public final class MyStructFloatFieldThrowExp implements com.facebook.thrift.payload.ThriftSerializable {

    @ThriftConstructor
    public MyStructFloatFieldThrowExp(
        @com.facebook.swift.codec.ThriftField(value=1, name="myLongField", requiredness=Requiredness.NONE) final long myLongField,
        @com.facebook.swift.codec.ThriftField(value=2, name="MyByteField", requiredness=Requiredness.NONE) final byte myByteField,
        @com.facebook.swift.codec.ThriftField(value=3, name="myStringField", requiredness=Requiredness.NONE) final String myStringField,
        @com.facebook.swift.codec.ThriftField(value=4, name="myFloatField", requiredness=Requiredness.NONE) final float myFloatField
    ) {
        this.myLongField = myLongField;
        this.myByteField = myByteField;
        this.myStringField = myStringField;
        this.myFloatField = myFloatField;
    }
    
    @ThriftConstructor
    protected MyStructFloatFieldThrowExp() {
      this.myLongField = 0L;
      this.myByteField = 0;
      this.myStringField = null;
      this.myFloatField = 0.f;
    }
    
    public static class Builder {
    
        private long myLongField = 0L;
        private byte myByteField = 0;
        private String myStringField = null;
        private float myFloatField = 0.f;
    
        @com.facebook.swift.codec.ThriftField(value=1, name="myLongField", requiredness=Requiredness.NONE)
        public Builder setMyLongField(long myLongField) {
            this.myLongField = myLongField;
            return this;
        }
    
        public long getMyLongField() { return myLongField; }
    
            @com.facebook.swift.codec.ThriftField(value=2, name="MyByteField", requiredness=Requiredness.NONE)
        public Builder setMyByteField(byte myByteField) {
            this.myByteField = myByteField;
            return this;
        }
    
        public byte getMyByteField() { return myByteField; }
    
            @com.facebook.swift.codec.ThriftField(value=3, name="myStringField", requiredness=Requiredness.NONE)
        public Builder setMyStringField(String myStringField) {
            this.myStringField = myStringField;
            return this;
        }
    
        public String getMyStringField() { return myStringField; }
    
            @com.facebook.swift.codec.ThriftField(value=4, name="myFloatField", requiredness=Requiredness.NONE)
        public Builder setMyFloatField(float myFloatField) {
            this.myFloatField = myFloatField;
            return this;
        }
    
        public float getMyFloatField() { return myFloatField; }
    
        public Builder() { }
        public Builder(MyStructFloatFieldThrowExp other) {
            this.myLongField = other.myLongField;
            this.myByteField = other.myByteField;
            this.myStringField = other.myStringField;
            this.myFloatField = other.myFloatField;
        }
    
        @ThriftConstructor
        public MyStructFloatFieldThrowExp build() {
            MyStructFloatFieldThrowExp result = new MyStructFloatFieldThrowExp (
                this.myLongField,
                this.myByteField,
                this.myStringField,
                this.myFloatField
            );
            return result;
        }
    }
                    public static final Map<String, Integer> NAMES_TO_IDS = new HashMap();
    public static final Map<String, Integer> THRIFT_NAMES_TO_IDS = new HashMap();
    public static final Map<Integer, TField> FIELD_METADATA = new HashMap<>();
    private static final TStruct STRUCT_DESC = new TStruct("MyStructFloatFieldThrowExp");
    private final long myLongField;
    public static final int _MYLONGFIELD = 1;
    private static final TField MY_LONG_FIELD_FIELD_DESC = new TField("myLongField", TType.I64, (short)1);
        private final byte myByteField;
    public static final int _MYBYTEFIELD = 2;
    private static final TField MY_BYTE_FIELD_FIELD_DESC = new TField("MyByteField", TType.BYTE, (short)2);
        private final String myStringField;
    public static final int _MYSTRINGFIELD = 3;
    private static final TField MY_STRING_FIELD_FIELD_DESC = new TField("myStringField", TType.STRING, (short)3);
        private final float myFloatField;
    public static final int _MYFLOATFIELD = 4;
    private static final TField MY_FLOAT_FIELD_FIELD_DESC = new TField("myFloatField", TType.FLOAT, (short)4);
    static {
      NAMES_TO_IDS.put("myLongField", 1);
      THRIFT_NAMES_TO_IDS.put("myLongField", 1);
      FIELD_METADATA.put(1, MY_LONG_FIELD_FIELD_DESC);
      NAMES_TO_IDS.put("myByteField", 2);
      THRIFT_NAMES_TO_IDS.put("MyByteField", 2);
      FIELD_METADATA.put(2, MY_BYTE_FIELD_FIELD_DESC);
      NAMES_TO_IDS.put("myStringField", 3);
      THRIFT_NAMES_TO_IDS.put("myStringField", 3);
      FIELD_METADATA.put(3, MY_STRING_FIELD_FIELD_DESC);
      NAMES_TO_IDS.put("myFloatField", 4);
      THRIFT_NAMES_TO_IDS.put("myFloatField", 4);
      FIELD_METADATA.put(4, MY_FLOAT_FIELD_FIELD_DESC);
    }
    
    
    @com.facebook.swift.codec.ThriftField(value=1, name="myLongField", requiredness=Requiredness.NONE)
    public long getMyLongField() { return myLongField; }
    
    
    
    @com.facebook.swift.codec.ThriftField(value=2, name="MyByteField", requiredness=Requiredness.NONE)
    public byte getMyByteField() { return myByteField; }
    
    
    @Nullable
    @com.facebook.swift.codec.ThriftField(value=3, name="myStringField", requiredness=Requiredness.NONE)
    public String getMyStringField() { return myStringField; }
    
    
    
    @com.facebook.swift.codec.ThriftField(value=4, name="myFloatField", requiredness=Requiredness.NONE)
    public float getMyFloatField() { return myFloatField; }
    
    @java.lang.Override
    public String toString() {
        ToStringHelper helper = toStringHelper(this);
        helper.add("myLongField", myLongField);
        helper.add("myByteField", myByteField);
        helper.add("myStringField", myStringField);
        helper.add("myFloatField", myFloatField);
        return helper.toString();
    }
    
    @java.lang.Override
    public boolean equals(java.lang.Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }
    
        MyStructFloatFieldThrowExp other = (MyStructFloatFieldThrowExp)o;
    
        return
            Objects.equals(myLongField, other.myLongField) &&
    Objects.equals(myByteField, other.myByteField) &&
    Objects.equals(myStringField, other.myStringField) &&
    Objects.equals(myFloatField, other.myFloatField) &&
            true;
    }
    
    @java.lang.Override
    public int hashCode() {
        return Arrays.deepHashCode(new java.lang.Object[] {
            myLongField,
            myByteField,
            myStringField,
            myFloatField
        });
    }
    
    
    public static com.facebook.thrift.payload.Reader<MyStructFloatFieldThrowExp> asReader() {
      return MyStructFloatFieldThrowExp::read0;
    }
    
    public static MyStructFloatFieldThrowExp read0(TProtocol oprot) throws TException {
      TField __field;
      oprot.readStructBegin(MyStructFloatFieldThrowExp.NAMES_TO_IDS, MyStructFloatFieldThrowExp.THRIFT_NAMES_TO_IDS, MyStructFloatFieldThrowExp.FIELD_METADATA);
      MyStructFloatFieldThrowExp.Builder builder = new MyStructFloatFieldThrowExp.Builder();
      while (true) {
        __field = oprot.readFieldBegin();
        if (__field.type == TType.STOP) { break; }
        switch (__field.id) {
        case _MYLONGFIELD:
          if (__field.type == TType.I64) {
            long myLongField = oprot.readI64();
            builder.setMyLongField(myLongField);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _MYBYTEFIELD:
          if (__field.type == TType.BYTE) {
            byte myByteField = oprot.readByte();
            builder.setMyByteField(myByteField);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _MYSTRINGFIELD:
          if (__field.type == TType.STRING) {
            String myStringField = oprot.readString();
            builder.setMyStringField(myStringField);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _MYFLOATFIELD:
          if (__field.type == TType.FLOAT) {
            float myFloatField = oprot.readFloat();
            builder.setMyFloatField(myFloatField);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        default:
          TProtocolUtil.skip(oprot, __field.type);
          break;
        }
        oprot.readFieldEnd();
      }
      oprot.readStructEnd();
      return builder.build();
    }
    
    public void write0(TProtocol oprot) throws TException {
      oprot.writeStructBegin(STRUCT_DESC);
      oprot.writeFieldBegin(MY_LONG_FIELD_FIELD_DESC);
      oprot.writeI64(this.myLongField);
      oprot.writeFieldEnd();
      oprot.writeFieldBegin(MY_BYTE_FIELD_FIELD_DESC);
      oprot.writeByte(this.myByteField);
      oprot.writeFieldEnd();
      if (myStringField != null) {
        oprot.writeFieldBegin(MY_STRING_FIELD_FIELD_DESC);
        oprot.writeString(this.myStringField);
        oprot.writeFieldEnd();
      }
      oprot.writeFieldBegin(MY_FLOAT_FIELD_FIELD_DESC);
      oprot.writeFloat(this.myFloatField);
      oprot.writeFieldEnd();
      oprot.writeFieldStop();
      oprot.writeStructEnd();
    }
    
    private static class _MyStructFloatFieldThrowExpLazy {
        private static final MyStructFloatFieldThrowExp _DEFAULT = new MyStructFloatFieldThrowExp.Builder().build();
    }
    
    public static MyStructFloatFieldThrowExp defaultInstance() {
        return  _MyStructFloatFieldThrowExpLazy._DEFAULT;
    }
}
