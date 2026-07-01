import "./style.css";

import init, {
  load_from_json,
  save_to_json,
  step,
  welcome,
  welcome_on_load,
} from "../wasm-build/herb_game";

const localStorageKey = "herbGameSave";

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
    // Output chars at 30 fps, targeting a total runtime of 3 seconds
    // except with a minimum output rate of 80 chars/second.
    const runtimeMs = Math.min(3000, output.length * (1000 / 80));
    const intervals = runtimeMs / 16;
    const chunkSize = Math.round(output.length / intervals);
    for (let i = 0; i < output.length; i += chunkSize) {
      outputSpan.innerText += output.slice(i, i + chunkSize);
      scrollToBottom();
      await new Promise((resolve) => setTimeout(resolve, 16));
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
// RUN COMMAND
// ----------

let metaCommandState: "quit" | null = null;
const QUIT_WORDS = ["quit", "exit", "restart"];

async function runCommand(input: string): Promise<string> {
  // Meta-commands handled in JS.
  const normalized = normalize(input);
  switch (metaCommandState) {
    case "quit":
      if (normalized.startsWith("y") || QUIT_WORDS.includes(normalized)) {
        // Quit the game and start over.
        localStorage.removeItem(localStorageKey);
        window.location.reload();
        return "";
      } else {
        // Don't quit.
        metaCommandState = null;
        return "Well, excuse you.";
      }
    case null:
      if (QUIT_WORDS.includes(normalized)) {
        metaCommandState = "quit";
        return "Quit your current game and start over - are you sure?";
      }
    // Else fall through to normal commands.
  }

  // Run command in Rust.
  const output = step(input);
  localStorage.setItem(localStorageKey, save_to_json());
  return output;
}

function normalize(command: string): string {
  return command.trim().toLocaleLowerCase();
}

// ----------
// INITIAL SETUP
// ----------

prepareOutput();

// WASM init
await init();

// Try to load saved state.
const loaded = loadSavedState();

// Wait a bit, then display welcome text.
await new Promise((resolve) => setTimeout(resolve, 500));
doOutput(loaded ? welcome_on_load() : welcome());

function loadSavedState(): boolean {
  const savedJson = localStorage.getItem(localStorageKey);
  if (savedJson) {
    try {
      load_from_json(savedJson);
      return true;
    } catch (err) {
      // Remove the offending state, but store a backup.
      localStorage.setItem(localStorageKey + "_corrupted", savedJson);
      localStorage.removeItem(localStorageKey);
      console.error(err);
      // Try to continue normally (with a new game), though it might panic in step().
    }
  }

  return false;
}
