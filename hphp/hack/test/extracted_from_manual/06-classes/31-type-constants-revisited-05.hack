// @generated by hh_manual from manual/hack/06-classes/31-type-constants-revisited.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
abstract class Box {
  abstract const type T;
  public function __construct(private this::T $value) {}
  public function get(): this::T {
    return $this->value;
  }
  public function set(this::T $val): this {
    $this->value = $val;
    return $this;
  }
}

class IntBox extends Box {
  const type T = int;
}

<<__EntryPoint>>
function run(): void {
  $ibox = new IntBox(10);
  \var_dump($ibox);
  $ibox->set(123);
  \var_dump($ibox);
}
