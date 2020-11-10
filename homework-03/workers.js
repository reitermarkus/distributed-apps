function workers(N, k) {
  const workers = Math.floor(N / 3)**k

  const iterations = N**N
  const iterationsPerFunction = Math.ceil(iterations / workers)

  return {
    workers,
    placements_from: Array.from({length: workers}, (_, i) => i * iterationsPerFunction),
    placements_per_function: iterationsPerFunction,
  }
}

function main(params) {
  return workers(params.N, params.k)
}

if (require.main === module) {
  console.log(workers(9, 2))
}
