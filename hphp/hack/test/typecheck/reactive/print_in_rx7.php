<?hh // strict
<<file: __EnableUnstableFeatures('coeffects_provisional')>>

interface Rx {}

class A {
  <<__RxShallow, __OnlyRxIfImpl(Rx::class)>>
  public function f(): void {
    // should be error
    echo 1;
  }
}
