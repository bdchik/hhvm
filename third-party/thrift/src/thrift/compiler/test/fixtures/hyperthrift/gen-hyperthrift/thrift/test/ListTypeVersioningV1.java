/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

package ;

import java.util.List;
import java.util.Map;
import java.util.Set;
import javax.annotation.concurrent.Immutable;
import javax.annotation.Nullable;
import com.facebook.hyperthrift.HyperThriftBase;
import com.facebook.hyperthrift.reflect.HyperThriftType;

@Immutable
@HyperThriftType
public class ListTypeVersioningV1 extends HyperThriftBase {
  public static final String TYPE_NAME = "thrift.test.ListTypeVersioningV1";


  @Nullable
  public List<Integer> myints() {
    return getFieldValue(0);
  }

  @Nullable
  public String hello() {
    return getFieldValue(1);
  }



  public static class Builder extends HyperThriftBase.Builder {
    public Builder() {
      super(2);
    }

    public Builder(ListTypeVersioningV1 other) {
      super(other);
    }

    @Nullable
    public List<Integer> myints() {
      return getFieldValue(0);
    }

    public Builder ListTypeVersioningV1( List<Integer> value) {
      setFieldValue(0, value);
      return this;
    }

    @Nullable
    public String hello() {
      return getFieldValue(1);
    }

    public Builder ListTypeVersioningV1( String value) {
      setFieldValue(1, value);
      return this;
    }

    public ListTypeVersioningV1 build() {
      Object[] fields = markBuilt();
      ListTypeVersioningV1 instance = new ListTypeVersioningV1();
      instance.init(TYPE_NAME, fields);
      return instance;
    }
  }
}
