import { getIbmBucketObject, ProcessResultInput as Input, ProcessResultOutput as Output } from './shared.ts'

export async function main(input: Input): Promise<Output> {
  input.symbols = input.symbols.map(symbol => symbol.replace(/^"?(.*?)"?$/, "$1"))
  input.object_keys = input.object_keys.map(key => key.replace(/^"?(.*?)"?$/, "$1"))

  const timestamps = {}

  const objects = await Promise.all(input.object_keys.map(async (key, i) => await getIbmBucketObject(key)))

  objects.forEach((object, i) => {
    const symbol = input.symbols[i]

    object.p90.forEach(d => {
      timestamps[d.Timestamp] = (timestamps[d.Timestamp] || {})
      timestamps[d.Timestamp][symbol] = d.Value
    })
  })

  return {
    labels: Object.keys(timestamps),
    datasets: input.symbols.map(symbol => ({
      label: symbol,
      data: Object.values(timestamps).map(v => v[symbol])
    }))
  }
}

if (require.main === module) {
  const json = process.argv[2]

  if (json) {
    main(JSON.parse(json))
      .then(response => console.log(JSON.stringify(response, null, 2)))
  }
}
