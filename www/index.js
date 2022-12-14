import * as wasm from "mk10";

const problem = document.getElementById("problem");
const solvebtn = document.getElementById("solvebtn");
const solution = document.getElementById("solution");

solvebtn.onclick = () => {
  const data = problem.value;

  if (!/^[0-9]{4}$/.test(data)) {
    solution.innerText = "Invalid number!";
  } else {
    solution.innerText = wasm.solve_problem(
      parseInt(data[0]),
      parseInt(data[1]),
      parseInt(data[2]),
      parseInt(data[3])
    );
  }
};
