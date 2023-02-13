// @generated by hh_manual from manual/hack/11-built-in-types/35-enum-class.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
function expect_string(string $str): void {
  echo 'expect_string called with: '.$str."\n";
}

interface IKey {
  public function name(): string;
}

abstract class Key<T> implements IKey {
  public function __construct(private string $name)[] {}
  public function name(): string {
    return $this->name;
  }
  public abstract function coerceTo(mixed $data): T;
}

class IntKey extends Key<int> {
  public function coerceTo(mixed $data): int {
    return $data as int;
  }
}

class StringKey extends Key<string> {
  public function coerceTo(mixed $data): string {
    // random logic can be implemented here
    $s = $data as string;
    // let's make everything in caps
    return Str\capitalize($s);
  }
}

enum class EKeys: IKey {
  // here are a default key, but this could be left empty
  Key<string> NAME = new StringKey('NAME');
}

abstract class DictBase {
  // type of the keys, left abstract for now
  abstract const type TKeys as EKeys;
  // actual data storage
  private dict<string, mixed> $raw_data = dict[];

  // generic code written once which enforces type safety
  public function get<T>(\HH\MemberOf<this::TKeys, Key<T>> $key): ?T {
    $name = $key->name();
    $raw_data = idx($this->raw_data, $name);
    // key might not be set
    if ($raw_data is nonnull) {
      $data = $key->coerceTo($raw_data);
      return $data;
    }
    return null;
  }

  public function set<T>(\HH\MemberOf<this::TKeys, Key<T>> $key, T $data): void {
    $name = $key->name();
    $this->raw_data[$name] = $data;
  }
}

class Foo { /* user code in here */ }

class MyKeyType extends Key<Foo> {
  public function coerceTo(mixed $data): Foo {
    // user code validation
    return $data as Foo;
  }
}

enum class MyKeys: IKey extends EKeys {
  Key<int> AGE = new IntKey('AGE');
  MyKeyType BLI = new MyKeyType('BLI');
}

class MyDict extends DictBase {
  const type TKeys = MyKeys;
}

<<__EntryPoint>>
function main(): void {
  $d = new MyDict();
  $d->set(MyKeys::NAME, 'tony');
  $d->set(MyKeys::BLI, new Foo());
  // $d->set(MyKeys::AGE, new Foo()); // type error
  expect_string($d->get(MyKeys::NAME) as nonnull);
}
