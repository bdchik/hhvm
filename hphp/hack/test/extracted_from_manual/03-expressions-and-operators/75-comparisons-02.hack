// @generated by hh_manual from manual/hack/03-expressions-and-operators/75-comparisons.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
async function example_snippet_wrapper(): Awaitable<void> {
  "a" < "b";   // true
  "ab" < "b";  // true
  
  "01" < "1";  // false (1 == 1)
}
