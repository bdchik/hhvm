// @generated by hh_manual from manual/hack/16-readonly/02-syntax.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
class Bar {}
class Foo {
  private static readonly ?Bar $static_bar = null;
  public function __construct(
    private readonly Bar $bar,
  ){}
}
