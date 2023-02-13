// @generated by hh_manual from manual/hack/15-asynchronous-operations/10-exceptions.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
async function exception_thrower(): Awaitable<void> {
  throw new \Exception("Return exception handle");
}

async function basic_exception(): Awaitable<void> {
  // the handle does not throw, but result will be an Exception objection.
  // Remember, this is the same as:
  //   $handle = exception_thrower();
  //   await $handle;
  await exception_thrower();
}

<<__EntryPoint>>
function main(): void {
  HH\Asio\join(basic_exception());
}
