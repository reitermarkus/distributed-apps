import fetch from 'node-fetch'

import { CreateChartInput as Input, CreateChartOutput as Output } from './shared.ts'

export async function main(input: Input): Promise<Output> {
  const res = await fetch('https://quickchart.io/chart/create', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      backgroundColor: 'transparent',
      width: 720,
      height: 480,
      format: 'png',
      chart: {
        type: 'bar',
        data: input,
      },
    }),
  })

  const { url } = await res.json();
  return { url: url }
}

if (require.main === module) {
  const json = process.argv[2]

  if (json) {
    main(JSON.parse(json))
      .then(response => console.log(response))
  }
}

