// @generated by hh_manual from manual/hack/11-built-in-types/10-float.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
function average_float(float $p1, float $p2): float {
  return ($p1 + $p2) / 2.0;
}

<<__EntryPoint>>
function main(): void {
  $val = 3e6;
  $result = average_float($val, 5.2E-2);
  echo "\$result is ".$result."\n";
}
