import fetch from 'node-fetch'

export interface FetchPricesInput {
  symbol: string,
}

export interface FetchPricesOutput {
  symbol: string,
  object_key: string,
}

export type ForecastInput = FetchPricesOutput
export interface ForecastOutput {
  symbol: string,
  object_key: string,
}

export interface ProcessResultInput {
  symbols: string[],
  object_keys: string[],
}
export interface ProcessResultOutput {
  labels: string[],
  datasets: { label: string, data: number[] }[]
}

export type CreateChartInput = ProcessResultOutput
export interface CreateChartOutput {
  url: string,
}

export interface Token {
  access_token: string,
  refresh_token_expiration: number,
  scope: string,
  token_type: string,
}

export function assert(value: string): asserts value {
  if (!value) {
    throw new Error('A symbol must be specified')
  }
}

export async function getIbmBucketObject(objectKey: string): Promise<any> {
  const endpoint = process.env.IBM_OBJECT_STORAGE_ENDPOINT_URL
  const bucket = process.env.IBM_OBJECT_STORAGE_BUCKET_NAME

  const url = `https://${endpoint}/${bucket}/${objectKey}`

  let object = await fetch(url)
  return await object.json()
}

export const getIbmBearerToken = async (): Promise<string> => {
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

export const uploadToIbmBucket = async (objectKey: string, token: string, results: object): Promise<void> => {
  const endpoint = process.env.IBM_OBJECT_STORAGE_ENDPOINT_URL
  const bucket = process.env.IBM_OBJECT_STORAGE_BUCKET_NAME

  const url = `https://${endpoint}/${bucket}/${objectKey}`

  const res = await fetch(url, {
    method: 'PUT',
    headers: {
      'Authorization': `bearer ${token}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(results),
  })
}
