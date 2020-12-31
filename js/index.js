import("../pkg/index.js").then(wasm => {
    loadPuzzles(wasm);
}).catch(console.error);

function blurOnEvent(event) {
    event.target.blur();
}

function loadPuzzles(context) {
    var i = 1;
    while (true) {
        const puzzle = document.querySelector("#day" + i);
        if (puzzle == null) {
            break;
        }
        const button = puzzle.querySelector("button");
        const textArea = puzzle.querySelector("textarea");
        const form = puzzle.querySelector("form");
        const ans1 = puzzle.querySelector("#ans1");
        const ans2 = puzzle.querySelector("#ans2");

        const puzzle1Fn = eval("context.day" + i + "puzzle1");
        const puzzle2Fn = eval("context.day" + i + "puzzle2");

        button.addEventListener("mouseout", blurOnEvent);
        form.addEventListener("submit", event => {
            event.preventDefault(); 
            const result1 = puzzle1Fn(textArea.value);
            const result2 = puzzle2Fn(textArea.value);
            
            ans1.textContent = "Answer 1: " + result1;
            ans2.textContent = "Answer 2: " + result2;
        });
        puzzles.push(puzzle);

        i++;
    }
}

var puzzles = []