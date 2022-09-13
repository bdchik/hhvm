/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

package test.fixtures.refs;

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
@com.facebook.swift.codec.ThriftStruct(value="StructWithUnion", builder=StructWithUnion.Builder.class)
public final class StructWithUnion implements com.facebook.thrift.payload.ThriftSerializable {

    @ThriftConstructor
    public StructWithUnion(
        @com.facebook.swift.codec.ThriftField(value=1, name="u", requiredness=Requiredness.NONE) final test.fixtures.refs.MyUnion u,
        @com.facebook.swift.codec.ThriftField(value=2, name="aDouble", requiredness=Requiredness.NONE) final double aDouble,
        @com.facebook.swift.codec.ThriftField(value=3, name="f", requiredness=Requiredness.NONE) final test.fixtures.refs.MyField f
    ) {
        this.u = u;
        this.aDouble = aDouble;
        this.f = f;
    }
    
    @ThriftConstructor
    protected StructWithUnion() {
      this.u = null;
      this.aDouble = 0.;
      this.f = null;
    }
    
    public static class Builder {
    
        private test.fixtures.refs.MyUnion u = null;
        private double aDouble = 0.;
        private test.fixtures.refs.MyField f = null;
    
        @com.facebook.swift.codec.ThriftField(value=1, name="u", requiredness=Requiredness.NONE)
        public Builder setU(test.fixtures.refs.MyUnion u) {
            this.u = u;
            return this;
        }
    
        public test.fixtures.refs.MyUnion getU() { return u; }
    
            @com.facebook.swift.codec.ThriftField(value=2, name="aDouble", requiredness=Requiredness.NONE)
        public Builder setADouble(double aDouble) {
            this.aDouble = aDouble;
            return this;
        }
    
        public double getADouble() { return aDouble; }
    
            @com.facebook.swift.codec.ThriftField(value=3, name="f", requiredness=Requiredness.NONE)
        public Builder setF(test.fixtures.refs.MyField f) {
            this.f = f;
            return this;
        }
    
        public test.fixtures.refs.MyField getF() { return f; }
    
        public Builder() { }
        public Builder(StructWithUnion other) {
            this.u = other.u;
            this.aDouble = other.aDouble;
            this.f = other.f;
        }
    
        @ThriftConstructor
        public StructWithUnion build() {
            StructWithUnion result = new StructWithUnion (
                this.u,
                this.aDouble,
                this.f
            );
            return result;
        }
    }
                public static final Map<String, Integer> NAMES_TO_IDS = new HashMap();
    public static final Map<String, Integer> THRIFT_NAMES_TO_IDS = new HashMap();
    public static final Map<Integer, TField> FIELD_METADATA = new HashMap<>();
    private static final TStruct STRUCT_DESC = new TStruct("StructWithUnion");
    private final test.fixtures.refs.MyUnion u;
    public static final int _U = 1;
    private static final TField U_FIELD_DESC = new TField("u", TType.STRUCT, (short)1);
        private final double aDouble;
    public static final int _ADOUBLE = 2;
    private static final TField A_DOUBLE_FIELD_DESC = new TField("aDouble", TType.DOUBLE, (short)2);
        private final test.fixtures.refs.MyField f;
    public static final int _F = 3;
    private static final TField F_FIELD_DESC = new TField("f", TType.STRUCT, (short)3);
    static {
      NAMES_TO_IDS.put("u", 1);
      THRIFT_NAMES_TO_IDS.put("u", 1);
      FIELD_METADATA.put(1, U_FIELD_DESC);
      NAMES_TO_IDS.put("aDouble", 2);
      THRIFT_NAMES_TO_IDS.put("aDouble", 2);
      FIELD_METADATA.put(2, A_DOUBLE_FIELD_DESC);
      NAMES_TO_IDS.put("f", 3);
      THRIFT_NAMES_TO_IDS.put("f", 3);
      FIELD_METADATA.put(3, F_FIELD_DESC);
    }
    
    @Nullable
    @com.facebook.swift.codec.ThriftField(value=1, name="u", requiredness=Requiredness.NONE)
    public test.fixtures.refs.MyUnion getU() { return u; }
    
    
    
    @com.facebook.swift.codec.ThriftField(value=2, name="aDouble", requiredness=Requiredness.NONE)
    public double getADouble() { return aDouble; }
    
    
    @Nullable
    @com.facebook.swift.codec.ThriftField(value=3, name="f", requiredness=Requiredness.NONE)
    public test.fixtures.refs.MyField getF() { return f; }
    
    @java.lang.Override
    public String toString() {
        ToStringHelper helper = toStringHelper(this);
        helper.add("u", u);
        helper.add("aDouble", aDouble);
        helper.add("f", f);
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
    
        StructWithUnion other = (StructWithUnion)o;
    
        return
            Objects.equals(u, other.u) &&
    Objects.equals(aDouble, other.aDouble) &&
    Objects.equals(f, other.f) &&
            true;
    }
    
    @java.lang.Override
    public int hashCode() {
        return Arrays.deepHashCode(new java.lang.Object[] {
            u,
            aDouble,
            f
        });
    }
    
    
    public static com.facebook.thrift.payload.Reader<StructWithUnion> asReader() {
      return StructWithUnion::read0;
    }
    
    public static StructWithUnion read0(TProtocol oprot) throws TException {
      TField __field;
      oprot.readStructBegin(StructWithUnion.NAMES_TO_IDS, StructWithUnion.THRIFT_NAMES_TO_IDS, StructWithUnion.FIELD_METADATA);
      StructWithUnion.Builder builder = new StructWithUnion.Builder();
      while (true) {
        __field = oprot.readFieldBegin();
        if (__field.type == TType.STOP) { break; }
        switch (__field.id) {
        case _U:
          if (__field.type == TType.STRUCT) {
            test.fixtures.refs.MyUnion u = test.fixtures.refs.MyUnion.read0(oprot);
            builder.setU(u);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _ADOUBLE:
          if (__field.type == TType.DOUBLE) {
            double aDouble = oprot.readDouble();
            builder.setADouble(aDouble);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _F:
          if (__field.type == TType.STRUCT) {
            test.fixtures.refs.MyField f = test.fixtures.refs.MyField.read0(oprot);
            builder.setF(f);
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
      if (u != null) {
        oprot.writeFieldBegin(U_FIELD_DESC);
        this.u.write0(oprot);
        oprot.writeFieldEnd();
      }
      oprot.writeFieldBegin(A_DOUBLE_FIELD_DESC);
      oprot.writeDouble(this.aDouble);
      oprot.writeFieldEnd();
      if (f != null) {
        oprot.writeFieldBegin(F_FIELD_DESC);
        this.f.write0(oprot);
        oprot.writeFieldEnd();
      }
      oprot.writeFieldStop();
      oprot.writeStructEnd();
    }
    
    private static class _StructWithUnionLazy {
        private static final StructWithUnion _DEFAULT = new StructWithUnion.Builder().build();
    }
    
    public static StructWithUnion defaultInstance() {
        return  _StructWithUnionLazy._DEFAULT;
    }
}
