// @generated by hh_manual from manual/hack/16-readonly/03-subtyping.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
class Foo {}
// promises not to modify $x
function takes_readonly(readonly Foo $x): void {
}

function test(): void {
    $z = new Foo();
    takes_readonly($z); // ok
}
