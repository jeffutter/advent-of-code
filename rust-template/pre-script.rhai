let day = variable::get("project-name");

if day.starts_with("day0") {
  day.remove("day0");
} else {
  day.remove("day");
};

let year = system::date().year;

variable::set("year", year.to_string());
variable::set("day", day);
