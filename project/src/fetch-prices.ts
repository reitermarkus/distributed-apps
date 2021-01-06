const alpha = require('alphavantage')({ key: process.env.ALPHA_KEY })
import fetch from 'node-fetch'
import {Params, assert } from './shared.ts'

interface Token {
  access_token: string,
  refresh_token_expiration: number,
  scope: string,
  token_type: string,
}

const renameObjectKeys = (obj: object) => (
  Object.keys(obj).reduce((acc, key) => ({
    ...acc,
    ...{[key.match(/^(\d+-\d+-\d+).*$/)[1]]: obj[key]}
  }), {})
)

const getBearerFetch = async (): Promise<string> => {
  const params = new URLSearchParams();
  params.append('apikey', process.env.IBM_OBJECT_STORAGE_API_KEY)
  params.append('response_type', 'cloud_iam')
  params.append('grant_type', 'urn:ibm:params:oauth:grant-type:apikey')

  const res = await fetch('https://iam.cloud.ibm.com/oidc/token', {
    method: 'POST',
    headers: {
      'Accept': 'application/json'
    },
    body: params,
  })

  const token: Token = await res.json()

  return token.access_token
}

const uploadToBucket = async (symbol: string, token: string, results: object): Promise<void> => {
  const endpoint = process.env.IBM_OBJECT_STORAGE_ENDPOINT_URL
  const bucket = process.env.IBM_OBJECT_STORAGE_BUCKET_NAME

  const url = `https://${endpoint}/${bucket}/${symbol}.json`

  const res = await fetch(url, {
    method: 'PUT',
    headers: {
      'Authorization': `bearer ${token}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(results),
  })
}

async function main(params: Params) {
  assert(params?.symbol)
  const dailyResults = alpha.util.polish(await alpha.data.daily(params?.symbol))
  const renamedResults = renameObjectKeys(dailyResults.data)
  const token = await getBearerFetch()
  await uploadToBucket(params?.symbol, token, renamedResults)
}

exports.main = main

