//// newmodule.hack
// @generated by hh_manual from manual/hack/17-modules/08-reflection-and-migration.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
new module foo {}

//// foo.hack
module foo;
class Cls {
  <<__SoftInternal>>
  internal function foo_soft(): void {
    echo "Hello from foo_soft\n";
  }
}
