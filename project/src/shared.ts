import fetch from 'node-fetch'

export interface Params {
  symbol: string;
}

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
