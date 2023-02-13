// @generated by hh_manual from manual/hack/03-expressions-and-operators/25-subscript.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
async function example_snippet_wrapper(): Awaitable<void> {
  $text = "Hello";
  $e = $text[4];    // designates the element with key 4 value "o"
  $text[1] = "?";   // changes the element with key 1 from "e" to "?"
  
  $v = vec[10, 25, -6];
  $e = $v[1];     // designates the element with key 1 value 25
  $v[2] = 44;     // changes the element with key 2 from -6 to 44
  
  $d = dict["red" => 4, "white" =>12, "blue" => 3];
  $e = $d["white"]; // designates the element with key "white" value 12
  $d["red"] = 9;    // changes the element with key "red" from 4 to 9
}
