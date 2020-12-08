function workers(N, k) {
  const regions = 3

  const workers = Math.floor(N / 3)**k
  const workersPerRegion = workers / 3

  const iterations = N**N
  const iterationsPerFunction = Math.ceil(iterations / workers)

  const placementsFrom = Array.from({length: workersPerRegion}, (_, r) => {
    return Array.from({length: workersPerRegion}, (_, i) => (r * workersPerRegion * iterationsPerFunction) + i * iterationsPerFunction)
  })

  return {
    placements_from_first: placementsFrom[0],
    placements_from_second: placementsFrom[1],
    placements_from_third: placementsFrom[2],
    placements_per_function: iterationsPerFunction,
    workers_per_loop: workersPerRegion,
  }
}

function main(params) {
  return workers(params.N, params.k)
}

if (require.main === module) {
  console.log(workers(9, 2))
}
