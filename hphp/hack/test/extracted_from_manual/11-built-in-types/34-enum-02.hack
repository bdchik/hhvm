// @generated by hh_manual from manual/hack/11-built-in-types/34-enum.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
enum Permission: string {
  Read = 'R';
  Write = 'W';
  Execute = 'E';
  Delete = 'D';
}
