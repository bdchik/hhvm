// @generated by hh_manual from manual/hack/08-arrays-and-collections/10-object-collections.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
// Creating an ImmMap.
function get_items(): ImmMap<string, int> {
  $items = ImmMap {'a' => 1, 'b' => 3};
  return $items;
}
