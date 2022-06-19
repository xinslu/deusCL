import init, { main } from "../code/pkg/code.js";

await init();

export function run(event) {
    event.preventDefault();
    console.log(event.target[0].value);
    var result = main(event.target[0].value);
    console.log(result);
    document.getElementById("result").innerHTML = result;
}

window.run = run;
