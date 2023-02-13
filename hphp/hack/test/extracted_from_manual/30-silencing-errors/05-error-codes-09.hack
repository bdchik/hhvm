// @generated by hh_manual from manual/hack/30-silencing-errors/05-error-codes.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
class MyClass {
  public int $x = 0;
  public function foo(): void {}
}

function foo(?MyClass $m): void {
  /* HH_FIXME[4064] Accessing a property on a nullable object. */
  $value = $m->x;

  /* HH_FIXME[4064] Calling a method on a nullable object. */
  $m->foo();
}
