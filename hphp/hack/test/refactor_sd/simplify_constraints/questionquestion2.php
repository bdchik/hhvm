<?hh

<<__SupportDynamicType>>
function f(): void {}

<<__SupportDynamicType>>
function g(): void {}

function h(): void {
  $x = Null;
  $x ??= f<>;
  $x upcast dynamic;
}
