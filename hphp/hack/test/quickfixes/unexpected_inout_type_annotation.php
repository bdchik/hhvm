<?hh

function takes_param(int $x): void {}

function call_it(): void {
  $x = 1;
  takes_param(inout $x);
}