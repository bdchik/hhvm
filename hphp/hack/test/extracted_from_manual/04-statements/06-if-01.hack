// @generated by hh_manual from manual/hack/04-statements/06-if.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
async function example_snippet_wrapper(): Awaitable<void> {
  $count = 11;
  
  if ($count < 10) {
    echo "small";
  } else if ($count < 20) {
    echo "medium";
  } else {
    echo "large";
  }
}
