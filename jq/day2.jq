def trim:
  sub("^\\s*"; "") | sub("\\s*$"; "")
;

def shape_to_int:
  . as $i |
  (" ABC" | index($i)) // (" XYZ" | index($i))
;

def score1:
  .mine +
  if .opponent == .mine then
    3
  elif (.opponent % 3) + 1 == .mine then
    6
  else
    0
  end
;

def score2:
  .mine = (.opponent + .mine) % 3 + 1 |
  score1
;

def game(score):
  map(score) | add
;

$input | trim |
split("\n") |
map(
  split(" ") |
  map(shape_to_int) |
  {
    "opponent": .[0],
    "mine": .[1]
  }
) |
{
  "day1": game(score1),
  "day2": game(score2)
}
