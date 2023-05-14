def trim:
  sub("^\\s*"; "") | sub("\\s*$"; "")
;

$input | trim | 
split("\n\n") | 
map(
  split("\n") |
  map(tonumber) |
  add
) |
{
  "day1": max,
  "day2": 
    sort |
    .[-3:] |
    add
}
