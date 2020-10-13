'use strict'

var num_queens = -1;

function acceptable(queen_rows) {
  for (var i = 0; i < num_queens; i++) {
    for (var j = i + 1; j < num_queens; j++) {
      if (queen_rows[i] == queen_rows[j]) {
        return false
      }
      if (queen_rows[i] - queen_rows[j] == i - j || queen_rows[i] - queen_rows[j] == j - i) {
        return false
      }
    }
  }
  return true
}

function fraction(n, from, to) {
  num_queens = n
  var solutions = 0
  for (var iter = from; iter < to; iter++) {
    var code = iter
    var queen_rows = []
    for (var i = 0; i < num_queens; i++) {
      queen_rows[i] = code % num_queens
      code = Math.floor(code / num_queens)
    }
    if (acceptable(queen_rows)) {
      solutions += 1
      console.log("Found valid placement: ", queen_rows)
    }
  }
  return solutions
}

function main(params) {
  var n = params.board_size

  if (n === null || n === undefined) {
    console.error('Input missing: n')
    return { solutions: null }
  }

  console.log(`Input: ${n}`)

  var max_iter = 1
  for (var i = 0; i < n; i++) {
    max_iter *= n
  }

  var solutions = fraction(n, 0, max_iter)

  console.log(`Solutions: ${solutions}`)

  return { solutions }
}
