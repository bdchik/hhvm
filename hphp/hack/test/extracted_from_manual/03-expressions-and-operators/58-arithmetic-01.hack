// @generated by hh_manual from manual/hack/03-expressions-and-operators/58-arithmetic.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
async function example_snippet_wrapper(): Awaitable<void> {
  -10 + 100;       // int with value 90
  100 + -3.4e2;    // float with value -240
  9.5 + 23.444;    // float with value 32.944
}
