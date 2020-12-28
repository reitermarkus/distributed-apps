const alpha = require('alphavantage')({ key: process.env.ALPHA_KEY })

interface Params {
  share: string;
}

function assert(value: any): asserts value {
  if (!value) {
    throw new Error('A share must be specified')
  }
}

async function main(params: Params) {
  assert(params?.share)
  return await alpha.data.daily(params?.share)
}

exports.main = main

