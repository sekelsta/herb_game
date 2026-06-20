import "./style.css";

const terminal = document.getElementById("terminal") as HTMLDivElement;
const terminalWrapper = document.getElementById(
  "terminal-wrapper",
) as HTMLDivElement;

// ----------
// TERMINAL I/O
// ----------

let state: "input" | "output" = "output";
let inputSpan = document.createElement("span");
let outputSpan = document.createElement("span");

function prepareInput() {
  const inputP = document.createElement("p");
  inputP.className = "input";
  inputP.innerHTML = `<span class="prompt">&gt; </span><span class="input-span"></span>`;
  inputSpan = inputP.querySelector(".input-span")!;
  terminal.appendChild(inputP);

  scrollToBottom();

  state = "input";
}

function prepareOutput() {
  const outputP = document.createElement("p");
  outputP.className = "output";
  outputP.innerHTML = `<span class="output-span"></span>`;
  outputSpan = outputP.querySelector(".output-span")!;
  terminal.appendChild(outputP);

  scrollToBottom();

  state = "output";
}

function submitInput() {
  const input = inputSpan.innerText;
  prepareOutput();

  void runCommand(input).then(doOutput);
}

function doOutput(output: string): void {
  // TODO: type gradually?
  outputSpan.innerText = output;

  prepareInput();
}

terminalWrapper.onkeydown = (event) => {
  if (state !== "input") return;

  const inputText = inputSpan.innerText;

  if (event.key === "Enter") {
    submitInput();
  } else if (event.key === "Backspace") {
    if (inputText.length === 0) return;
    setInputText(inputText.slice(0, -1));
  } else if (shouldType(event)) {
    setInputText(inputText + event.key);
  }
};

function setInputText(newInputText: string): void {
  inputSpan.innerText = newInputText;
  scrollToBottom();
}

function shouldType(e: KeyboardEvent): boolean {
  return e.key.length === 1 && !e.ctrlKey && !e.metaKey && !e.altKey;
}

function scrollToBottom() {
  terminalWrapper.scrollTop = terminalWrapper.scrollHeight;
}

// ----------
// RUN COMMAND IN RUST
// ----------

async function runCommand(input: string): Promise<string> {
  // TODO
  return "You said: " + input;
}

// ----------
// INITIAL SETUP
// ----------

prepareOutput();
// TODO
doOutput("Top text");
