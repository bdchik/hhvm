// @generated by hh_manual from manual/hack/03-expressions-and-operators/13-list.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
async function example_snippet_wrapper(): Awaitable<void> {
  $tuple = tuple('a', 'b', 'c', 1, 2, 3);
  list($_, $b, $c, $_, $two, $_) = $tuple;
  echo "b -> {$b}, c -> {$c}, two -> {$two}\n";
}
