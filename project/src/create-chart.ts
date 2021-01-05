import fetch from 'node-fetch'

interface Params {
  symbols: string[],
  timestamps: string[],
  prices: number[],
}

export async function main(params: Params): Promise<any> {
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
        data: {
          labels: params.symbols,
          datasets: params.timestamps.map((e, i) => ({ label: e, data: [params.prices[i]] })),
        }
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
      .then(response => console.log(response.url))
  }
}
