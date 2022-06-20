import init, { main } from "../code/pkg/code.js";

await init();

var line = 0;

export function run(event) {
    event.preventDefault();
    console.log(event.target[0].value);
    var result = main(event.target[0].value);
    console.log(result);
    document.getElementById("result" + line).innerHTML = result;
    document.getElementById("code" + line).readOnly = true;
    line++;
    var div = document.createElement("div");
    var string =
        '<form autocomplete="off" onsubmit="return run(event)"><h4>DEUSCL-USER><input id="code' +
        line +
        '"></input><input type="submit" style="display: none" /></h4></form><h3 id="result' +
        line +
        '"></h3>';
    div.innerHTML = string.trim();
    document.getElementById("root").appendChild(div);
}

var input = document.getElementById("code" + line);
input.addEventListener("keyup", function (event) {
    if ((event.keyCode === 38 || event.which === 38) && line > 0) {
        console.log("here" + line);
        document.getElementById("code" + line).value = document.getElementById(
            "code" + (line - 1)
        ).value;
    }
});

window.run = run;
