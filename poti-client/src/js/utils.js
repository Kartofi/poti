const sizes = ["bytes", "KB", "MB", "GB", "TB"];

function format_size(input) {
  let current = 0;
  input = Number(input);

  while (input > 1024) {
    input /= 1024;
    current++;
  }
  return input.toFixed(2) + " " + sizes[current];
}

export { format_size };
