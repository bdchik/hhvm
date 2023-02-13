// @generated by hh_manual from manual/hack/08-arrays-and-collections/20-mutating-values.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
function update_value(inout vec<int> $items): void {
  $items[0] = 42;
  $items[] = 100;
}

function demo(): void {
  $v = vec[1, 2];
  update_value(inout $v);

  var_dump($v); // vec[42, 2, 100]
}
