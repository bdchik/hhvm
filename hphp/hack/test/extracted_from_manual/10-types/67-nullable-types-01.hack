// @generated by hh_manual from manual/hack/10-types/67-nullable-types.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
function takes_nullable_str(?string $s): string {
  if ($s is null){
    return "default";
  } else {
    return $s;
  }
}
