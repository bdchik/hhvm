// @generated by hh_manual from manual/hack/11-built-in-types/74-nothing.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
type undefined = nothing;

function undefined(): undefined {
  throw new Exception('NOT IMPLEMENTED: `undefined` cannot be produced.');
}

interface MyInterface {
  public function isAmazed(): bool;
}

function do_something(MyInterface $my_interface): bool {
  return $my_interface->isAmazed();
}

<<__EntryPoint>>
async function main_async(): Awaitable<void> {
  $my_interface = undefined();
  // We won't ever reach this line, since `undefined()` will halt the program by throwing.
  // We can't produce a MyInterface just yet, since there are no classes which implement it.
  // `undefined` is a placeholder for now.
  // We can continue writing our business logic and come back to this later.
  if (do_something($my_interface)) {
    // Write the body first, worry about the condition later.
  }
}
