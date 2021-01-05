import {Params, assert } from './shared.ts'
import { createObjectCsvWriter as createCsvWriter } from 'csv-writer'
import fetch from 'node-fetch'
const s3 = require('@auth0/s3')

// const {
  // S3Client,
  // PutObjectCommand,
  // CreateBucketCommand
// } = require("@aws-sdk/client-s3")

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
      {id: 'timestamp', title: 'high'},
      {id: 'high', title: 'target_value'},
    ],
    path: `${symbol}.csv`
  })

  await csvWriter.writeRecords(flat)
}

const awsBucketManagement = async (symbol: string) => {
  // const REGION = 'us-east-1'
//
  // const bucketName = process.env.AWS_FORECAST_BUCKET
  // const bucketParams = { Bucket: bucketName };
//
  // const keyName = `${symbol}.csv`;
  // const objectParams = { Bucket: bucketName, Key: keyName, Body: 'item_id,high,target_value\nmsft,2020-12-28,226.0300' }
//
  // console.log(bucketParams)
//
  // const s3 = new S3Client({
    // region: REGION,
  // });
//
  // try {
    // const data = await s3.send(new CreateBucketCommand(bucketParams));
    // console.log("Success. Bucket created.");
  // } catch (err) {
    // console.log("Error", err);
  // }
  // try {
    // const results = await s3.send(new PutObjectCommand(objectParams));
    // console.log("Successfully uploaded data to " + bucketName + "/" + keyName);
  // } catch (err) {
    // console.log("Error", err);
  // }

  console.log(process.env.AWS_ACCESS_KEY_ID)

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
      // other options supported by putObject, except Body and ContentLength.
      // See: http://docs.aws.amazon.com/AWSJavaScriptSDK/latest/AWS/S3.html#putObject-property
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
