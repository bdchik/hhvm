// @generated by hh_manual from manual/hack/03-expressions-and-operators/97-assignment.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
async function example_snippet_wrapper(): Awaitable<void> {
  $s = "ab";
  $s[0] = "x"; // in bounds
  $s[3] = "y"; // $s is now "xb y"
}
