// @generated by hh_manual from manual/hack/06-classes/05-properties.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
class HasDefaultValue {
  public int $i = 0;
}

class SetInConstructor {
  public int $i;
  public function __construct() {
    $this->i = 0;
  }
}
