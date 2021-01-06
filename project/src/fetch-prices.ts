const alpha = require('alphavantage')({ key: process.env.ALPHAVANTAGE_API_KEY })
import fetch from 'node-fetch'
import {FetchPricesInput, FetchPricesOutput, assert, getIbmBearerToken, uploadToIbmBucket } from './shared.ts'

const renameObjectKeys = (obj: object) => (
  Object.keys(obj).reduce((acc, key) => ({
    ...acc,
    ...{[key.match(/^(\d+-\d+-\d+).*$/)[1]]: obj[key]}
  }), {})
)

export async function main(params: FetchPricesInput): Promise<FetchPricesOutput> {
  assert(params?.symbol)
  const dailyResults = alpha.util.polish(await alpha.data.daily(params?.symbol))
  const renamedResults = renameObjectKeys(dailyResults.data)
  const token = await getIbmBearerToken()
  const objectKey = `${params.symbol}.json`
  await uploadToIbmBucket(objectKey, token, renamedResults)
  return { symbol: params.symbol, object_key: objectKey }
}

if (require.main === module) {
  const json = process.argv[2]

  if (json) {
    main(JSON.parse(json))
      .then(response => console.log(JSON.stringify(response, null, 2)))
  }
}
