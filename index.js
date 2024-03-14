// const bindgen = require("./target/release/node_bindgen.node");
const bindgen = require("./target/debug/node_bindgen.node");

console.log(bindgen, "--");

console.time("rust");
console.log("rust-result:", bindgen.fibonacci(35, 10));
console.timeEnd("rust");

console.time("node");
console.log("node-result:", fibonacci_node(40, 10));
console.timeEnd("node");

function fibonacci_node(a, b) {
  return fibonacci(a) + fibonacci(b);
}
function fibonacci(n) {
  if (n <= 1) {
    return n;
  } else {
    return fibonacci(n - 1) + fibonacci(n - 2);
  }
}
