import com.google.protobuf.InvalidProtocolBufferException;
import static proto.FooOuterClass.Foo;

public class Main {
  public static void main(String[] args) throws InvalidProtocolBufferException {
    System.out.println(makeMessage("Hello World!"));
  }

  public static Foo makeMessage(String msg) {
    Foo.Builder person = Foo.newBuilder();
    person.setMsg(msg);
    return person.build();
  }
}
