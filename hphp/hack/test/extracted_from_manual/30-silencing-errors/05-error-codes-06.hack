// @generated by hh_manual from manual/hack/30-silencing-errors/05-error-codes.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
class MyClass {}

function takes_myclass(MyClass $c): void {
  /* HH_FIXME[4053] No such method. */
  $c->someMethod();
  /* HH_FIXME[4053] No such property. */
  $x = $c->someProperty;
}
