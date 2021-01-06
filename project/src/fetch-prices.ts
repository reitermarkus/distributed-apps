const alpha = require('alphavantage')({ key: process.env.ALPHA_KEY })
import fetch from 'node-fetch'
import {Params, assert, getIbmBearerToken, uploadToIbmBucket } from './shared.ts'

const renameObjectKeys = (obj: object) => (
  Object.keys(obj).reduce((acc, key) => ({
    ...acc,
    ...{[key.match(/^(\d+-\d+-\d+).*$/)[1]]: obj[key]}
  }), {})
)

async function main(params: Params) {
  assert(params?.symbol)
  const dailyResults = alpha.util.polish(await alpha.data.daily(params?.symbol))
  const renamedResults = renameObjectKeys(dailyResults.data)
  const token = await getIbmBearerToken()
  await uploadToIbmBucket(params?.symbol, token, renamedResults)
}

exports.main = main

