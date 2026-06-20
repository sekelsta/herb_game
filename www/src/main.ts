import "./style.css";

const terminal = document.getElementById("terminal") as HTMLDivElement;

// Focus the terminal when you click anywhere within its wrapper.
const terminalWrapper = document.getElementById("terminal-wrapper")!;
terminalWrapper.onclick = () => terminal.focus();

// ----------
// TERMINAL INPUT
// ----------

let textInputState: "input" | "output" = "input";
let pendingInput = "";

terminal.onkeydown = (event) => {
  if (textInputState !== "input") return;

  if (event.key === "Enter") {
    void submitInput();
  } else if (event.key === "Backspace") {
    if (pendingInput.length === 0) return;
    updateInput(pendingInput.slice(0, -1));
  } else if (shouldType(event)) {
    updateInput(pendingInput + event.key);
  }
};

async function submitInput() {
  const input = pendingInput;
  pendingInput = "";
  terminal.innerText += "\n";
  textInputState = "output";

  const output = await processInput(input);
  terminal.innerText += output + "\n> ";
  textInputState = "input";
}

function updateInput(newInput: string): void {
  terminal.innerText =
    terminal.innerText.slice(
      0,
      terminal.innerText.length - pendingInput.length,
    ) + newInput;
  pendingInput = newInput;
}

function shouldType(e: KeyboardEvent): boolean {
  return e.key.length === 1 && !e.ctrlKey && !e.metaKey && !e.altKey;
}

async function processInput(input: string): Promise<string> {
  // TODO
  return "You said: " + input;
}

// ----------
// INITIAL PROMPT
// ----------

// TODO
terminal.innerText = "Top text\n> ";
