/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

package test.fixtures.basic_swift_bean;

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
@com.facebook.swift.codec.ThriftStruct("MyStruct3")
public final class MyStruct3 implements com.facebook.thrift.payload.ThriftSerializable {

    @ThriftConstructor
    public MyStruct3() {
      this.myInt16 = (short)42;
      this.myInt32 = 422;
      this.myInt64 = 422222222L;
      this.myString = "foo";
      this.myBool = true;
      this.myDouble = (double)42.42;
      this.mySet = com.google.common.collect.Sets.newHashSet(ImmutableSet.<String>builder()
        .add("foo")
        .add("bar")
        .add("baz")
        .build());
      this.myDataItem = new test.fixtures.basic_swift_bean.MyDataItem.Builder().setField1(29).setField2(30).build();
      this.myList = com.google.common.collect.Lists.newArrayList(ImmutableList.<test.fixtures.basic_swift_bean.MyDataItem>builder()
        .add(new test.fixtures.basic_swift_bean.MyDataItem.Builder().setField1(29).setField2(30).build())
        .add(new test.fixtures.basic_swift_bean.MyDataItem.Builder().setField1(31).setField2(32).build())
        .build());
      this.myMapList = com.google.common.collect.Maps.newHashMap(ImmutableMap.<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>>builder()
        .put(1, com.google.common.collect.Lists.newArrayList(ImmutableList.<test.fixtures.basic_swift_bean.MyDataItem>builder()
        .add(new test.fixtures.basic_swift_bean.MyDataItem.Builder().setField1(29).setField2(30).build())
        .add(new test.fixtures.basic_swift_bean.MyDataItem.Builder().setField1(31).setField2(32).build())
        .build()))
        .put(2, com.google.common.collect.Lists.newArrayList(ImmutableList.<test.fixtures.basic_swift_bean.MyDataItem>builder()
        .add(new test.fixtures.basic_swift_bean.MyDataItem.Builder().setField1(33).setField2(34).build())
        .add(new test.fixtures.basic_swift_bean.MyDataItem.Builder().setField1(35).setField2(36).build())
        .build()))
        .build());
      this.myEmptyList = com.google.common.collect.Lists.newArrayList(ImmutableList.<test.fixtures.basic_swift_bean.MyDataItem>builder()
        .build());
      this.myEmptyMapList = com.google.common.collect.Maps.newHashMap(ImmutableMap.<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>>builder()
        .build());
    }
    
    public static class Builder {
    
        private short myInt16 = (short)42;
        private int myInt32 = 422;
        private long myInt64 = 422222222L;
        private String myString = "foo";
        private boolean myBool = true;
        private double myDouble = (double)42.42;
        private Set<String> mySet = com.google.common.collect.Sets.newHashSet(ImmutableSet.<String>builder()
        .add("foo")
        .add("bar")
        .add("baz")
        .build());
        private test.fixtures.basic_swift_bean.MyDataItem myDataItem = new test.fixtures.basic_swift_bean.MyDataItem.Builder().setField1(29).setField2(30).build();
        private List<test.fixtures.basic_swift_bean.MyDataItem> myList = com.google.common.collect.Lists.newArrayList(ImmutableList.<test.fixtures.basic_swift_bean.MyDataItem>builder()
        .add(new test.fixtures.basic_swift_bean.MyDataItem.Builder().setField1(29).setField2(30).build())
        .add(new test.fixtures.basic_swift_bean.MyDataItem.Builder().setField1(31).setField2(32).build())
        .build());
        private Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> myMapList = com.google.common.collect.Maps.newHashMap(ImmutableMap.<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>>builder()
        .put(1, com.google.common.collect.Lists.newArrayList(ImmutableList.<test.fixtures.basic_swift_bean.MyDataItem>builder()
        .add(new test.fixtures.basic_swift_bean.MyDataItem.Builder().setField1(29).setField2(30).build())
        .add(new test.fixtures.basic_swift_bean.MyDataItem.Builder().setField1(31).setField2(32).build())
        .build()))
        .put(2, com.google.common.collect.Lists.newArrayList(ImmutableList.<test.fixtures.basic_swift_bean.MyDataItem>builder()
        .add(new test.fixtures.basic_swift_bean.MyDataItem.Builder().setField1(33).setField2(34).build())
        .add(new test.fixtures.basic_swift_bean.MyDataItem.Builder().setField1(35).setField2(36).build())
        .build()))
        .build());
        private List<test.fixtures.basic_swift_bean.MyDataItem> myEmptyList = com.google.common.collect.Lists.newArrayList(ImmutableList.<test.fixtures.basic_swift_bean.MyDataItem>builder()
        .build());
        private Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> myEmptyMapList = com.google.common.collect.Maps.newHashMap(ImmutableMap.<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>>builder()
        .build());
    
        @com.facebook.swift.codec.ThriftField(value=1, name="myInt16", requiredness=Requiredness.NONE)
        public Builder setMyInt16(short myInt16) {
            this.myInt16 = myInt16;
            return this;
        }
    
        public short getMyInt16() { return myInt16; }
    
            @com.facebook.swift.codec.ThriftField(value=2, name="myInt32", requiredness=Requiredness.NONE)
        public Builder setMyInt32(int myInt32) {
            this.myInt32 = myInt32;
            return this;
        }
    
        public int getMyInt32() { return myInt32; }
    
            @com.facebook.swift.codec.ThriftField(value=3, name="myInt64", requiredness=Requiredness.NONE)
        public Builder setMyInt64(long myInt64) {
            this.myInt64 = myInt64;
            return this;
        }
    
        public long getMyInt64() { return myInt64; }
    
            @com.facebook.swift.codec.ThriftField(value=4, name="myString", requiredness=Requiredness.NONE)
        public Builder setMyString(String myString) {
            this.myString = myString;
            return this;
        }
    
        public String getMyString() { return myString; }
    
            @com.facebook.swift.codec.ThriftField(value=5, name="myBool", requiredness=Requiredness.NONE)
        public Builder setMyBool(boolean myBool) {
            this.myBool = myBool;
            return this;
        }
    
        public boolean isMyBool() { return myBool; }
    
            @com.facebook.swift.codec.ThriftField(value=6, name="myDouble", requiredness=Requiredness.NONE)
        public Builder setMyDouble(double myDouble) {
            this.myDouble = myDouble;
            return this;
        }
    
        public double getMyDouble() { return myDouble; }
    
            @com.facebook.swift.codec.ThriftField(value=7, name="mySet", requiredness=Requiredness.NONE)
        public Builder setMySet(Set<String> mySet) {
            this.mySet = mySet;
            return this;
        }
    
        public Set<String> getMySet() { return mySet; }
    
            @com.facebook.swift.codec.ThriftField(value=8, name="MyDataItem", requiredness=Requiredness.NONE)
        public Builder setMyDataItem(test.fixtures.basic_swift_bean.MyDataItem myDataItem) {
            this.myDataItem = myDataItem;
            return this;
        }
    
        public test.fixtures.basic_swift_bean.MyDataItem getMyDataItem() { return myDataItem; }
    
            @com.facebook.swift.codec.ThriftField(value=9, name="myList", requiredness=Requiredness.NONE)
        public Builder setMyList(List<test.fixtures.basic_swift_bean.MyDataItem> myList) {
            this.myList = myList;
            return this;
        }
    
        public List<test.fixtures.basic_swift_bean.MyDataItem> getMyList() { return myList; }
    
            @com.facebook.swift.codec.ThriftField(value=10, name="myMapList", requiredness=Requiredness.NONE)
        public Builder setMyMapList(Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> myMapList) {
            this.myMapList = myMapList;
            return this;
        }
    
        public Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> getMyMapList() { return myMapList; }
    
            @com.facebook.swift.codec.ThriftField(value=11, name="myEmptyList", requiredness=Requiredness.NONE)
        public Builder setMyEmptyList(List<test.fixtures.basic_swift_bean.MyDataItem> myEmptyList) {
            this.myEmptyList = myEmptyList;
            return this;
        }
    
        public List<test.fixtures.basic_swift_bean.MyDataItem> getMyEmptyList() { return myEmptyList; }
    
            @com.facebook.swift.codec.ThriftField(value=12, name="myEmptyMapList", requiredness=Requiredness.NONE)
        public Builder setMyEmptyMapList(Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> myEmptyMapList) {
            this.myEmptyMapList = myEmptyMapList;
            return this;
        }
    
        public Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> getMyEmptyMapList() { return myEmptyMapList; }
    
        public Builder() { }
        public Builder(MyStruct3 other) {
            this.myInt16 = other.myInt16;
            this.myInt32 = other.myInt32;
            this.myInt64 = other.myInt64;
            this.myString = other.myString;
            this.myBool = other.myBool;
            this.myDouble = other.myDouble;
            this.mySet = other.mySet;
            this.myDataItem = other.myDataItem;
            this.myList = other.myList;
            this.myMapList = other.myMapList;
            this.myEmptyList = other.myEmptyList;
            this.myEmptyMapList = other.myEmptyMapList;
        }
    
        @ThriftConstructor
        public MyStruct3 build() {
            MyStruct3 result = new MyStruct3();
            result.myInt16 = this.myInt16;
            result.myInt32 = this.myInt32;
            result.myInt64 = this.myInt64;
            result.myString = this.myString;
            result.myBool = this.myBool;
            result.myDouble = this.myDouble;
            result.mySet = this.mySet;
            result.myDataItem = this.myDataItem;
            result.myList = this.myList;
            result.myMapList = this.myMapList;
            result.myEmptyList = this.myEmptyList;
            result.myEmptyMapList = this.myEmptyMapList;
            return result;
        }
    }
                                                    public static final Map<String, Integer> NAMES_TO_IDS = new HashMap();
    public static final Map<String, Integer> THRIFT_NAMES_TO_IDS = new HashMap();
    public static final Map<Integer, TField> FIELD_METADATA = new HashMap<>();
    private static final TStruct STRUCT_DESC = new TStruct("MyStruct3");
    private short myInt16;
    public static final int _MYINT16 = 1;
    private static final TField MY_INT16_FIELD_DESC = new TField("myInt16", TType.I16, (short)1);
    private int myInt32;
    public static final int _MYINT32 = 2;
    private static final TField MY_INT32_FIELD_DESC = new TField("myInt32", TType.I32, (short)2);
    private long myInt64;
    public static final int _MYINT64 = 3;
    private static final TField MY_INT64_FIELD_DESC = new TField("myInt64", TType.I64, (short)3);
    private String myString;
    public static final int _MYSTRING = 4;
    private static final TField MY_STRING_FIELD_DESC = new TField("myString", TType.STRING, (short)4);
    private boolean myBool;
    public static final int _MYBOOL = 5;
    private static final TField MY_BOOL_FIELD_DESC = new TField("myBool", TType.BOOL, (short)5);
    private double myDouble;
    public static final int _MYDOUBLE = 6;
    private static final TField MY_DOUBLE_FIELD_DESC = new TField("myDouble", TType.DOUBLE, (short)6);
    private Set<String> mySet;
    public static final int _MYSET = 7;
    private static final TField MY_SET_FIELD_DESC = new TField("mySet", TType.SET, (short)7);
    private test.fixtures.basic_swift_bean.MyDataItem myDataItem;
    public static final int _MYDATAITEM = 8;
    private static final TField MY_DATA_ITEM_FIELD_DESC = new TField("MyDataItem", TType.STRUCT, (short)8);
    private List<test.fixtures.basic_swift_bean.MyDataItem> myList;
    public static final int _MYLIST = 9;
    private static final TField MY_LIST_FIELD_DESC = new TField("myList", TType.LIST, (short)9);
    private Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> myMapList;
    public static final int _MYMAPLIST = 10;
    private static final TField MY_MAP_LIST_FIELD_DESC = new TField("myMapList", TType.MAP, (short)10);
    private List<test.fixtures.basic_swift_bean.MyDataItem> myEmptyList;
    public static final int _MYEMPTYLIST = 11;
    private static final TField MY_EMPTY_LIST_FIELD_DESC = new TField("myEmptyList", TType.LIST, (short)11);
    private Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> myEmptyMapList;
    public static final int _MYEMPTYMAPLIST = 12;
    private static final TField MY_EMPTY_MAP_LIST_FIELD_DESC = new TField("myEmptyMapList", TType.MAP, (short)12);
static {
      NAMES_TO_IDS.put("myInt16", 1);
      THRIFT_NAMES_TO_IDS.put("myInt16", 1);
      FIELD_METADATA.put(1, MY_INT16_FIELD_DESC);
      NAMES_TO_IDS.put("myInt32", 2);
      THRIFT_NAMES_TO_IDS.put("myInt32", 2);
      FIELD_METADATA.put(2, MY_INT32_FIELD_DESC);
      NAMES_TO_IDS.put("myInt64", 3);
      THRIFT_NAMES_TO_IDS.put("myInt64", 3);
      FIELD_METADATA.put(3, MY_INT64_FIELD_DESC);
      NAMES_TO_IDS.put("myString", 4);
      THRIFT_NAMES_TO_IDS.put("myString", 4);
      FIELD_METADATA.put(4, MY_STRING_FIELD_DESC);
      NAMES_TO_IDS.put("myBool", 5);
      THRIFT_NAMES_TO_IDS.put("myBool", 5);
      FIELD_METADATA.put(5, MY_BOOL_FIELD_DESC);
      NAMES_TO_IDS.put("myDouble", 6);
      THRIFT_NAMES_TO_IDS.put("myDouble", 6);
      FIELD_METADATA.put(6, MY_DOUBLE_FIELD_DESC);
      NAMES_TO_IDS.put("mySet", 7);
      THRIFT_NAMES_TO_IDS.put("mySet", 7);
      FIELD_METADATA.put(7, MY_SET_FIELD_DESC);
      NAMES_TO_IDS.put("myDataItem", 8);
      THRIFT_NAMES_TO_IDS.put("MyDataItem", 8);
      FIELD_METADATA.put(8, MY_DATA_ITEM_FIELD_DESC);
      NAMES_TO_IDS.put("myList", 9);
      THRIFT_NAMES_TO_IDS.put("myList", 9);
      FIELD_METADATA.put(9, MY_LIST_FIELD_DESC);
      NAMES_TO_IDS.put("myMapList", 10);
      THRIFT_NAMES_TO_IDS.put("myMapList", 10);
      FIELD_METADATA.put(10, MY_MAP_LIST_FIELD_DESC);
      NAMES_TO_IDS.put("myEmptyList", 11);
      THRIFT_NAMES_TO_IDS.put("myEmptyList", 11);
      FIELD_METADATA.put(11, MY_EMPTY_LIST_FIELD_DESC);
      NAMES_TO_IDS.put("myEmptyMapList", 12);
      THRIFT_NAMES_TO_IDS.put("myEmptyMapList", 12);
      FIELD_METADATA.put(12, MY_EMPTY_MAP_LIST_FIELD_DESC);
    }
    
    
    @com.facebook.swift.codec.ThriftField(value=1, name="myInt16", requiredness=Requiredness.NONE)
    public short getMyInt16() { return myInt16; }
    
    @com.facebook.swift.codec.ThriftField
    public MyStruct3 setMyInt16(short myInt16) {
        this.myInt16 = myInt16;
        return this;
    }
    
    
    
    @com.facebook.swift.codec.ThriftField(value=2, name="myInt32", requiredness=Requiredness.NONE)
    public int getMyInt32() { return myInt32; }
    
    @com.facebook.swift.codec.ThriftField
    public MyStruct3 setMyInt32(int myInt32) {
        this.myInt32 = myInt32;
        return this;
    }
    
    
    
    @com.facebook.swift.codec.ThriftField(value=3, name="myInt64", requiredness=Requiredness.NONE)
    public long getMyInt64() { return myInt64; }
    
    @com.facebook.swift.codec.ThriftField
    public MyStruct3 setMyInt64(long myInt64) {
        this.myInt64 = myInt64;
        return this;
    }
    
    
    @Nullable
    @com.facebook.swift.codec.ThriftField(value=4, name="myString", requiredness=Requiredness.NONE)
    public String getMyString() { return myString; }
    
    @com.facebook.swift.codec.ThriftField
    public MyStruct3 setMyString(String myString) {
        this.myString = myString;
        return this;
    }
    
    
    
    @com.facebook.swift.codec.ThriftField(value=5, name="myBool", requiredness=Requiredness.NONE)
    public boolean isMyBool() { return myBool; }
    
    @com.facebook.swift.codec.ThriftField
    public MyStruct3 setMyBool(boolean myBool) {
        this.myBool = myBool;
        return this;
    }
    
    
    
    @com.facebook.swift.codec.ThriftField(value=6, name="myDouble", requiredness=Requiredness.NONE)
    public double getMyDouble() { return myDouble; }
    
    @com.facebook.swift.codec.ThriftField
    public MyStruct3 setMyDouble(double myDouble) {
        this.myDouble = myDouble;
        return this;
    }
    
    
    @Nullable
    @com.facebook.swift.codec.ThriftField(value=7, name="mySet", requiredness=Requiredness.NONE)
    public Set<String> getMySet() { return mySet; }
    
    @com.facebook.swift.codec.ThriftField
    public MyStruct3 setMySet(Set<String> mySet) {
        this.mySet = mySet;
        return this;
    }
    
    
    @Nullable
    @com.facebook.swift.codec.ThriftField(value=8, name="MyDataItem", requiredness=Requiredness.NONE)
    public test.fixtures.basic_swift_bean.MyDataItem getMyDataItem() { return myDataItem; }
    
    @com.facebook.swift.codec.ThriftField
    public MyStruct3 setMyDataItem(test.fixtures.basic_swift_bean.MyDataItem myDataItem) {
        this.myDataItem = myDataItem;
        return this;
    }
    
    
    @Nullable
    @com.facebook.swift.codec.ThriftField(value=9, name="myList", requiredness=Requiredness.NONE)
    public List<test.fixtures.basic_swift_bean.MyDataItem> getMyList() { return myList; }
    
    @com.facebook.swift.codec.ThriftField
    public MyStruct3 setMyList(List<test.fixtures.basic_swift_bean.MyDataItem> myList) {
        this.myList = myList;
        return this;
    }
    
    
    @Nullable
    @com.facebook.swift.codec.ThriftField(value=10, name="myMapList", requiredness=Requiredness.NONE)
    public Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> getMyMapList() { return myMapList; }
    
    @com.facebook.swift.codec.ThriftField
    public MyStruct3 setMyMapList(Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> myMapList) {
        this.myMapList = myMapList;
        return this;
    }
    
    
    @Nullable
    @com.facebook.swift.codec.ThriftField(value=11, name="myEmptyList", requiredness=Requiredness.NONE)
    public List<test.fixtures.basic_swift_bean.MyDataItem> getMyEmptyList() { return myEmptyList; }
    
    @com.facebook.swift.codec.ThriftField
    public MyStruct3 setMyEmptyList(List<test.fixtures.basic_swift_bean.MyDataItem> myEmptyList) {
        this.myEmptyList = myEmptyList;
        return this;
    }
    
    
    @Nullable
    @com.facebook.swift.codec.ThriftField(value=12, name="myEmptyMapList", requiredness=Requiredness.NONE)
    public Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> getMyEmptyMapList() { return myEmptyMapList; }
    
    @com.facebook.swift.codec.ThriftField
    public MyStruct3 setMyEmptyMapList(Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> myEmptyMapList) {
        this.myEmptyMapList = myEmptyMapList;
        return this;
    }
    
    @java.lang.Override
    public String toString() {
        ToStringHelper helper = toStringHelper(this);
        helper.add("myInt16", myInt16);
        helper.add("myInt32", myInt32);
        helper.add("myInt64", myInt64);
        helper.add("myString", myString);
        helper.add("myBool", myBool);
        helper.add("myDouble", myDouble);
        helper.add("mySet", mySet);
        helper.add("myDataItem", myDataItem);
        helper.add("myList", myList);
        helper.add("myMapList", myMapList);
        helper.add("myEmptyList", myEmptyList);
        helper.add("myEmptyMapList", myEmptyMapList);
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
    
        MyStruct3 other = (MyStruct3)o;
    
        return
            Objects.equals(myInt16, other.myInt16) &&
    Objects.equals(myInt32, other.myInt32) &&
    Objects.equals(myInt64, other.myInt64) &&
    Objects.equals(myString, other.myString) &&
    Objects.equals(myBool, other.myBool) &&
    Objects.equals(myDouble, other.myDouble) &&
    Objects.equals(mySet, other.mySet) &&
    Objects.equals(myDataItem, other.myDataItem) &&
    Objects.equals(myList, other.myList) &&
    Objects.equals(myMapList, other.myMapList) &&
    Objects.equals(myEmptyList, other.myEmptyList) &&
    Objects.equals(myEmptyMapList, other.myEmptyMapList) &&
            true;
    }
    
    @java.lang.Override
    public int hashCode() {
        return Arrays.deepHashCode(new java.lang.Object[] {
            myInt16,
            myInt32,
            myInt64,
            myString,
            myBool,
            myDouble,
            mySet,
            myDataItem,
            myList,
            myMapList,
            myEmptyList,
            myEmptyMapList
        });
    }
    
    
    public static com.facebook.thrift.payload.Reader<MyStruct3> asReader() {
      return MyStruct3::read0;
    }
    
    public static MyStruct3 read0(TProtocol oprot) throws TException {
      TField __field;
      oprot.readStructBegin(MyStruct3.NAMES_TO_IDS, MyStruct3.THRIFT_NAMES_TO_IDS, MyStruct3.FIELD_METADATA);
      MyStruct3.Builder builder = new MyStruct3.Builder();
      while (true) {
        __field = oprot.readFieldBegin();
        if (__field.type == TType.STOP) { break; }
        switch (__field.id) {
        case _MYINT16:
          if (__field.type == TType.I16) {
            short myInt16 = oprot.readI16();
            builder.setMyInt16(myInt16);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _MYINT32:
          if (__field.type == TType.I32) {
            int myInt32 = oprot.readI32();
            builder.setMyInt32(myInt32);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _MYINT64:
          if (__field.type == TType.I64) {
            long myInt64 = oprot.readI64();
            builder.setMyInt64(myInt64);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _MYSTRING:
          if (__field.type == TType.STRING) {
            String myString = oprot.readString();
            builder.setMyString(myString);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _MYBOOL:
          if (__field.type == TType.BOOL) {
            boolean myBool = oprot.readBool();
            builder.setMyBool(myBool);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _MYDOUBLE:
          if (__field.type == TType.DOUBLE) {
            double myDouble = oprot.readDouble();
            builder.setMyDouble(myDouble);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _MYSET:
          if (__field.type == TType.SET) {
            Set<String> mySet;
            {
            TSet _set = oprot.readSetBegin();
            mySet = new HashSet<String>(Math.max(0, _set.size));
            for (int _i = 0; (_set.size < 0) ? oprot.peekSet() : (_i < _set.size); _i++) {
                
                String _value1 = oprot.readString();
                mySet.add(_value1);
            }
            oprot.readSetEnd();
            }
            builder.setMySet(mySet);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _MYDATAITEM:
          if (__field.type == TType.STRUCT) {
            test.fixtures.basic_swift_bean.MyDataItem myDataItem = test.fixtures.basic_swift_bean.MyDataItem.read0(oprot);
            builder.setMyDataItem(myDataItem);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _MYLIST:
          if (__field.type == TType.LIST) {
            List<test.fixtures.basic_swift_bean.MyDataItem> myList;
            {
            TList _list = oprot.readListBegin();
            myList = new ArrayList<test.fixtures.basic_swift_bean.MyDataItem>(Math.max(0, _list.size));
            for (int _i = 0; (_list.size < 0) ? oprot.peekList() : (_i < _list.size); _i++) {
                
                test.fixtures.basic_swift_bean.MyDataItem _value1 = test.fixtures.basic_swift_bean.MyDataItem.read0(oprot);
                myList.add(_value1);
            }
            oprot.readListEnd();
            }
            builder.setMyList(myList);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _MYMAPLIST:
          if (__field.type == TType.MAP) {
            Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> myMapList;
            {
            TMap _map = oprot.readMapBegin();
            myMapList = new HashMap<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>>(Math.max(0, _map.size));
            for (int _i = 0; (_map.size < 0) ? oprot.peekMap() : (_i < _map.size); _i++) {
                
                int _key1 = oprot.readI32();
                List<test.fixtures.basic_swift_bean.MyDataItem> _value1;
                            {
                            TList _list1 = oprot.readListBegin();
                            _value1 = new ArrayList<test.fixtures.basic_swift_bean.MyDataItem>(Math.max(0, _list1.size));
                            for (int _i1 = 0; (_list1.size < 0) ? oprot.peekList() : (_i1 < _list1.size); _i1++) {
                                
                                
                                test.fixtures.basic_swift_bean.MyDataItem _value2 = test.fixtures.basic_swift_bean.MyDataItem.read0(oprot);
                                
                                
                                _value1.add(_value2);
                                
                            }
                            oprot.readListEnd();
                            }
                                myMapList.put(_key1, _value1);
            }
            }
            oprot.readMapEnd();
            builder.setMyMapList(myMapList);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _MYEMPTYLIST:
          if (__field.type == TType.LIST) {
            List<test.fixtures.basic_swift_bean.MyDataItem> myEmptyList;
            {
            TList _list = oprot.readListBegin();
            myEmptyList = new ArrayList<test.fixtures.basic_swift_bean.MyDataItem>(Math.max(0, _list.size));
            for (int _i = 0; (_list.size < 0) ? oprot.peekList() : (_i < _list.size); _i++) {
                
                test.fixtures.basic_swift_bean.MyDataItem _value1 = test.fixtures.basic_swift_bean.MyDataItem.read0(oprot);
                myEmptyList.add(_value1);
            }
            oprot.readListEnd();
            }
            builder.setMyEmptyList(myEmptyList);
          } else {
            TProtocolUtil.skip(oprot, __field.type);
          }
          break;
        case _MYEMPTYMAPLIST:
          if (__field.type == TType.MAP) {
            Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> myEmptyMapList;
            {
            TMap _map = oprot.readMapBegin();
            myEmptyMapList = new HashMap<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>>(Math.max(0, _map.size));
            for (int _i = 0; (_map.size < 0) ? oprot.peekMap() : (_i < _map.size); _i++) {
                
                int _key1 = oprot.readI32();
                List<test.fixtures.basic_swift_bean.MyDataItem> _value1;
                            {
                            TList _list1 = oprot.readListBegin();
                            _value1 = new ArrayList<test.fixtures.basic_swift_bean.MyDataItem>(Math.max(0, _list1.size));
                            for (int _i1 = 0; (_list1.size < 0) ? oprot.peekList() : (_i1 < _list1.size); _i1++) {
                                
                                
                                test.fixtures.basic_swift_bean.MyDataItem _value2 = test.fixtures.basic_swift_bean.MyDataItem.read0(oprot);
                                
                                
                                _value1.add(_value2);
                                
                            }
                            oprot.readListEnd();
                            }
                                myEmptyMapList.put(_key1, _value1);
            }
            }
            oprot.readMapEnd();
            builder.setMyEmptyMapList(myEmptyMapList);
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
      oprot.writeFieldBegin(MY_INT16_FIELD_DESC);
      oprot.writeI16(this.myInt16);
      oprot.writeFieldEnd();
      oprot.writeFieldBegin(MY_INT32_FIELD_DESC);
      oprot.writeI32(this.myInt32);
      oprot.writeFieldEnd();
      oprot.writeFieldBegin(MY_INT64_FIELD_DESC);
      oprot.writeI64(this.myInt64);
      oprot.writeFieldEnd();
      if (myString != null) {
        oprot.writeFieldBegin(MY_STRING_FIELD_DESC);
        oprot.writeString(this.myString);
        oprot.writeFieldEnd();
      }
      oprot.writeFieldBegin(MY_BOOL_FIELD_DESC);
      oprot.writeBool(this.myBool);
      oprot.writeFieldEnd();
      oprot.writeFieldBegin(MY_DOUBLE_FIELD_DESC);
      oprot.writeDouble(this.myDouble);
      oprot.writeFieldEnd();
      if (mySet != null) {
        oprot.writeFieldBegin(MY_SET_FIELD_DESC);
        Set<String> _iter0 = mySet;
        oprot.writeSetBegin(new TSet(TType.STRING, _iter0.size()));
        for (String _iter1 : _iter0) {
          oprot.writeString(_iter1);
        }
        oprot.writeSetEnd();
        oprot.writeFieldEnd();
      }
      if (myDataItem != null) {
        oprot.writeFieldBegin(MY_DATA_ITEM_FIELD_DESC);
        this.myDataItem.write0(oprot);
        oprot.writeFieldEnd();
      }
      if (myList != null) {
        oprot.writeFieldBegin(MY_LIST_FIELD_DESC);
        List<test.fixtures.basic_swift_bean.MyDataItem> _iter0 = myList;
        oprot.writeListBegin(new TList(TType.STRUCT, _iter0.size()));
        for (test.fixtures.basic_swift_bean.MyDataItem _iter1 : _iter0) {
          _iter1.write0(oprot);
        }
        oprot.writeListEnd();
        oprot.writeFieldEnd();
      }
      if (myMapList != null) {
        oprot.writeFieldBegin(MY_MAP_LIST_FIELD_DESC);
        Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> _iter0 = myMapList;
        oprot.writeMapBegin(new TMap(TType.I32, TType.LIST, _iter0.size()));
        for (Map.Entry<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> _iter1 : _iter0.entrySet()) {
          oprot.writeI32(_iter1.getKey());
          
          oprot.writeListBegin(new TList(TType.STRUCT, _iter1.getValue().size()));
        for (test.fixtures.basic_swift_bean.MyDataItem _iter2 : _iter1.getValue()) {
          _iter2.write0(oprot);
        }
        oprot.writeListEnd();
        }
        oprot.writeMapEnd();
        oprot.writeFieldEnd();
      }
      if (myEmptyList != null) {
        oprot.writeFieldBegin(MY_EMPTY_LIST_FIELD_DESC);
        List<test.fixtures.basic_swift_bean.MyDataItem> _iter0 = myEmptyList;
        oprot.writeListBegin(new TList(TType.STRUCT, _iter0.size()));
        for (test.fixtures.basic_swift_bean.MyDataItem _iter1 : _iter0) {
          _iter1.write0(oprot);
        }
        oprot.writeListEnd();
        oprot.writeFieldEnd();
      }
      if (myEmptyMapList != null) {
        oprot.writeFieldBegin(MY_EMPTY_MAP_LIST_FIELD_DESC);
        Map<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> _iter0 = myEmptyMapList;
        oprot.writeMapBegin(new TMap(TType.I32, TType.LIST, _iter0.size()));
        for (Map.Entry<Integer, List<test.fixtures.basic_swift_bean.MyDataItem>> _iter1 : _iter0.entrySet()) {
          oprot.writeI32(_iter1.getKey());
          
          oprot.writeListBegin(new TList(TType.STRUCT, _iter1.getValue().size()));
        for (test.fixtures.basic_swift_bean.MyDataItem _iter2 : _iter1.getValue()) {
          _iter2.write0(oprot);
        }
        oprot.writeListEnd();
        }
        oprot.writeMapEnd();
        oprot.writeFieldEnd();
      }
      oprot.writeFieldStop();
      oprot.writeStructEnd();
    }
    

}
