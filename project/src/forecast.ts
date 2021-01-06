import { ForecastInput, ForecastOutput, assert, getIbmBearerToken, getIbmBucketObject, uploadToIbmBucket } from './shared.ts'
import { createObjectCsvWriter as createCsvWriter } from 'csv-writer'
import fetch from 'node-fetch'

const s3 = require('@auth0/s3')
const AWS = require('aws-sdk')
const util = require('util')

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

const createForecast = async (symbol: string) => {
  const forecastService = new AWS.ForecastService({
    apiVersion: '2018-06-26',
    accessKeyId: process.env.AWS_ACCESS_KEY_ID,
    secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY,
    sessionToken: process.env.AWS_SESSION_TOKEN,
    region: 'us-east-1'
  })

  const forecastQueryService = new AWS.ForecastQueryService({
    apiVersion: '2018-06-26',
    accessKeyId: process.env.AWS_ACCESS_KEY_ID,
    secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY,
    sessionToken: process.env.AWS_SESSION_TOKEN,
    region: 'us-east-1'
  })

  const arnPrefix = 'arn:aws:forecast:us-east-1:916751031899'
  const dataSetGroup = `${arnPrefix}:dataset-group/stock_forecast_group`

  const describeDataset = util.promisify(forecastService.describeDatasetGroup.bind(forecastService))
  const datasets = await describeDataset({DatasetGroupArn: dataSetGroup})
  const datasetArns = datasets['DatasetArns']

  const datasetParams = {
    DatasetName: `${symbol}_dataset`,
    DatasetType: 'TARGET_TIME_SERIES',
    Domain: 'CUSTOM',
    Schema: process.env.FORECAST_SCHEMA,
    DataFrequency: 'D',
  }

  try {
    const createDataset = util.promisify(forecastService.createDataset.bind(forecastService))
    await createDataset(datasetParams)
  } catch (e) {
    console.log('dataset already exists\n', e)
  }

  const updateDatasetGroup = util.promisify(forecastService.updateDatasetGroup.bind(forecastService))

  const datasetArn = `${arnPrefix}:dataset/${symbol}_dataset`

  await updateDatasetGroup({
    DatasetArns: [datasetArn],
    DatasetGroupArn: dataSetGroup
  })

  const importJob = util.promisify(forecastService.createDatasetImportJob.bind(forecastService))

  try {
    const importJobResult = await importJob({
      DataSource: {
        S3Config: {
          Path: `s3://${process.env.AWS_FORECAST_BUCKET}/${symbol}.csv`,
            RoleArn: process.env.AWS_FORECAST_ROLE
        }
      },
      DatasetArn: `${arnPrefix}:dataset/${symbol}_dataset`,
      DatasetImportJobName: `${symbol}_dataset_import`,
      TimestampFormat: 'yyyy-MM-dd'
    })
  } catch(e) {
    console.error('import already exists\n', e)
  }

  const datasetImportJobArn = `${arnPrefix}:dataset-import-job/${symbol}_dataset/${symbol}_dataset_import`

  const describeDatasetImportJob = util.promisify(forecastService.describeDatasetImportJob.bind(forecastService))

  const importStatus = await new Promise(async (res, rej) => {
    const interval = setInterval(async () => {
      const importDescription = await describeDatasetImportJob({
        DatasetImportJobArn: datasetImportJobArn
      })

      if (importDescription.Status === 'ACTIVE' || importDescription.Status === 'CREATE_FAILED') {
        clearInterval(interval)
        return res({
          status: 'Creation finished',
          creationTime: importDescription.CreationTime,
          modificationTime: importDescription.LastModificationTime
        })
      }
    }, 2000)
  })

  const predictorName = `${symbol}_predictor`

  const predictor = util.promisify(forecastService.createPredictor.bind(forecastService))

  try {
    const predictorParams = {
      AlgorithmArn: 'arn:aws:forecast:::algorithm/CNN-QR',
      PredictorName: predictorName,
      FeaturizationConfig: {
        ForecastFrequency: 'D'
      },
      ForecastHorizon: 1,
      InputDataConfig: {
        DatasetGroupArn: dataSetGroup
      }
    }

    await predictor(predictorParams)
  } catch (e) {
    console.error('predictor already exists\n', e)
  }

  const predictorArn = `${arnPrefix}:predictor/${symbol}_predictor`

  const describePredictor = util.promisify(forecastService.describePredictor.bind(forecastService))

  const predictorStatus = await new Promise(async (res, rej) => {
    const interval = setInterval(async () => {
      const predictorDescription = await describePredictor({
        PredictorArn: predictorArn
      })

      if (predictorDescription.Status === 'ACTIVE' || predictorDescription.Status === 'CREATE_FAILED') {
        clearInterval(interval)
        return res({
          status: 'Predictor finished',
          creationTime: predictorDescription.CreationTime,
          modificationTime: predictorDescription.LastModificationTime
        })
      }
    }, 2000)
  })

  const forecastName = `${symbol}_forecast`
  const createForecast = util.promisify(forecastService.createForecast.bind(forecastService))

  try {
    await createForecast({
      ForecastName: forecastName,
      PredictorArn: predictorArn
    })
  } catch (e) {
    console.error('forecast already exists\n', e)
  }

  const forecastArn = `${arnPrefix}:forecast/${symbol}_forecast`

  const describeForecast = util.promisify(forecastService.describeForecast.bind(forecastService))

  const forecastStatus = await new Promise(async (res, rej) => {
    const interval = setInterval(async () => {
      const forecastDescription = await describeForecast({
        ForecastArn: forecastArn
      })

      if (forecastDescription.Status === 'ACTIVE' || forecastDescription === 'CREATE_FAILED') {
        clearInterval(interval)
        return res({
          status: 'Forecast finished',
          creationTime: forecastDescription.CreationTime,
          modificationTime: forecastDescription.LastModificationTime
        })
      }
    }, 2000)
  })

  const queryForecast = util.promisify(forecastQueryService.queryForecast.bind(forecastQueryService))
  const forecastResult = await queryForecast({
    Filters: {
      item_id: symbol
    },
    ForecastArn: forecastArn
  })

  const objectKey = `${symbol}.forecast.json`

  const token = await getIbmBearerToken()
  await uploadToIbmBucket(objectKey, token, forecastResult['Forecast']['Predictions'])

  return { symbol, object_key: objectKey }
}

export async function main(params: ForecastInput): Promise<ForecastOutput> {
  assert(params?.symbol)

  const content = await getIbmBucketObject(params?.object_key)
  await symbolDataToCsv(params.symbol, content)
  await awsBucketManagement(params.symbol)

  return await createForecast(params.symbol)
}

if (require.main === module) {
  const json = process.argv[2]

  if (json) {
    main(JSON.parse(json))
      .then(response => console.log(JSON.stringify(response, null, 2)))
  }
}
