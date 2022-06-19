import init, { main } from "../code/pkg/code.js";

await init();

function submission(event) {
    event.preventDefault();
    console.log(main(event.target.value));
}
