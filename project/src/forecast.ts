import { Params, assert } from './shared.ts'
import { createObjectCsvWriter as createCsvWriter } from 'csv-writer'
import fetch from 'node-fetch'
const s3 = require('@auth0/s3')

const getIbmBucketContent = async (symbol: string) : Promise<void> => {
  const endpoint = process.env.IBM_OBJECT_STORAGE_ENDPOINT_URL
  const bucket = process.env.IBM_OBJECT_STORAGE_BUCKET_NAME

  const url = `https://${endpoint}/${bucket}/${symbol}.json`

  let content = await fetch(url)
  return await content.json()
}

const symbolDataToCsv = async (symbol: string, bucket_content: any) : Promise<void> => {
  const flat = Object.entries(bucket_content).reduce((acc, [key, val]) => {
    return [...acc, {
      symbol: symbol,
      timestamp: key,
      high: val['high']
    }]
  }, [])

  const csvWriter = createCsvWriter({
    header: [
      {id: 'symbol', title: 'item_id'},
      {id: 'timestamp', title: 'timestamp'},
      {id: 'high', title: 'target_value'},
    ],
    path: `${symbol}.csv`
  })

  await csvWriter.writeRecords(flat)
}

const awsBucketManagement = async (symbol: string) => {
  var client = s3.createClient({
    s3Options: {
      accessKeyId: process.env.AWS_ACCESS_KEY_ID,
      secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY,
      sessionToken: process.env.AWS_SESSION_TOKEN,
      region: 'us-east-1'
    },
  });

  var params = {
    localFile: `${symbol}.csv`,
    s3Params: {
      Bucket: process.env.AWS_FORECAST_BUCKET,
      Key: `${symbol}.csv`,
    },
  };

  const uploader = client.uploadFile(params)
}

async function main(params: Params) {
  assert(params?.symbol)
  const content = await getIbmBucketContent(params?.symbol)
  symbolDataToCsv(params?.symbol, content)
  awsBucketManagement(params?.symbol)
}

(() => main({symbol: 'msft'}))()


exports.main = main
