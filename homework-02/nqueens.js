'use strict'

const external = require('./external')

exports.fraction = async function(ev, res) {
  const num_queens = parseInt(ev.params.num_queens)
  const from = parseInt(ev.params.from)
  const to = parseInt(ev.params.to)

  // l name(nqueens) require(./external.js as external) vars(num_queens, from, to) return(result)

  let solutions = 0
  for (let iter = from; iter < to; iter++) {
    let code = iter
    let queen_rows = []
    for (let i = 0; i < num_queens; i++) {
      queen_rows[i] = code % num_queens
      code = Math.floor(code / num_queens)
    }

    if (external.acceptable(num_queens, queen_rows)) {
      solutions += 1
      console.log("Found valid placement: ", queen_rows)
    }
  }

  let result = { solutions: solutions }
  // lend

  return result
}

if (require.main === module) {
  const args = process.argv
  if (args.length > 2) {
    const num_queens = parseInt(args[2])
    const from = parseInt(args[3])
    const to = parseInt(args[4])
    console.log("Running for placement range ", from, " to ", to)
    this.fraction({
        params: {
          num_queens: num_queens,
          from: from,
          to: to,
        }
      })
      .then(console.log)
  } else {
    console.log("USAGE: node index.js NUM_QUEENS FROM TO")
  }
}
