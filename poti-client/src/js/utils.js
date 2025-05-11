const sizes = ["bytes", "KB", "MB", "GB", "TB"];

function format_size(input) {
  let current = 0;

  while (input > 1024) {
    input /= 1024;
    current++;
  }
  return input.toFixed(2) + " " + sizes[current];
}
function format_time(input) {
  let date = new Date(input);
  let now = new Date();

  let prefix = "";

  let get_days_diff = (date, date2) =>
    date.getDay() - date2.getDay() + (date.getMonth() - date2.getMonth()) * 30;

  let days_diff = Math.abs(get_days_diff(now, date));

  if (days_diff < 30) {
    if (days_diff == 0) {
      prefix = "Today at ";
    } else if (days_diff == 1) {
      prefix = "Yesterday at ";
    } else {
      prefix = days_diff + " days ago at ";
    }
  } else {
    let months = 0;
    while (days_diff >= 30) {
      months++;
      days_diff -= 30;
    }
    if (months == 1) {
      prefix = months + " month ago at ";
    } else {
      prefix = months + " months ago at ";
    }
  }

  return (
    prefix +
    String(date.getHours()).padStart(2, "0") +
    ":" +
    String(date.getMinutes()).padStart(2, "0")
  );
}

export { format_size, format_time };
