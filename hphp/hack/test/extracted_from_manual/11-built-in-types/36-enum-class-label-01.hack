// @generated by hh_manual from manual/hack/11-built-in-types/36-enum-class-label.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
enum E: int {
  A = 42;
  B = 42;
}

function f(E $value): void {
  switch($value) {
    case E::A: echo "A "; break;
    case E::B: echo "B "; break;
  }
  echo $value . "\n";
}
