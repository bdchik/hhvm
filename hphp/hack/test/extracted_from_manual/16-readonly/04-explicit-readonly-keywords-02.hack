// @generated by hh_manual from manual/hack/16-readonly/04-explicit-readonly-keywords.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/

class Bar {}
class Foo {
  public function __construct(
    public readonly Bar $bar,
  ) {}
}

function test(Foo $f): void {
  $bar = readonly $f->bar; // this is required
}
