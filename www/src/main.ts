import "./style.css";

import init, { step, welcome } from "../wasm-build/herb_game";

const terminal = document.getElementById("terminal") as HTMLDivElement;
const terminalWrapper = document.getElementById(
  "terminal-wrapper",
) as HTMLDivElement;

// ----------
// TERMINAL I/O
// ----------

let state: "input" | "output" = "output";
let inputSpan = document.createElement("span");
let cursorSpan: HTMLSpanElement | null = null;
let outputSpan = document.createElement("span");

function prepareInput() {
  const inputP = document.createElement("p");
  inputP.className = "input";
  inputP.innerHTML = `<span class="prompt">&gt; </span><span class="input-span"></span><span class="cursor">█</span>`;
  inputSpan = inputP.querySelector(".input-span")!;
  cursorSpan = inputP.querySelector(".cursor")!;
  terminal.appendChild(inputP);

  scrollToBottom();

  state = "input";
}

function prepareOutput() {
  cursorSpan?.remove();

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

async function doOutput(output: string, gradual = true) {
  if (gradual) {
    // Output one line per second.
    for (let i = 0; i < output.length; i++) {
      outputSpan.innerText += output[i];
      scrollToBottom();
      await new Promise((resolve) => setTimeout(resolve, 1000 / 80));
    }
  } else {
    outputSpan.innerText += output;
  }

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
  resetCursorBlink();
  scrollToBottom();
}

function shouldType(e: KeyboardEvent): boolean {
  return e.key.length === 1 && !e.ctrlKey && !e.metaKey && !e.altKey;
}

function resetCursorBlink(): void {
  if (!cursorSpan) return;

  cursorSpan.getAnimations().forEach((animation) => {
    animation.cancel();
    animation.play();
  });
}

function scrollToBottom() {
  terminalWrapper.scrollTop = terminalWrapper.scrollHeight;
}

// ----------
// RUN COMMAND IN RUST
// ----------

async function runCommand(input: string): Promise<string> {
  return step(input);
}

// ----------
// INITIAL SETUP
// ----------

prepareOutput();
await init();
doOutput(welcome(), false);
