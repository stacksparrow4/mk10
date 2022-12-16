import * as wasm from "mk10";

const problem = document.getElementById("problem");
const solvebtn = document.getElementById("solvebtn");
const solution = document.getElementById("solution");

solvebtn.onclick = () => {
  const data = problem.value;

  if (!/^[0-9]+$/.test(data)) {
    solution.innerText = "Invalid number!";
  } else {
    solution.innerText = wasm.solve_problem(
      new Int32Array(data.split("").map((x) => parseInt(x)))
    );
  }
};
