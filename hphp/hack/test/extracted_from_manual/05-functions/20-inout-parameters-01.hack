// @generated by hh_manual from manual/hack/05-functions/20-inout-parameters.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
function takes_inout(inout int $x): void {
  $x = 1;
}

function call_it(): void {
  $num = 0;
  takes_inout(inout $num);

  // $num is now 1.
}
