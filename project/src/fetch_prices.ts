const alpha = require('alphavantage')({ key: process.env.ALPHA_KEY })

async function main(params: object) {
  return await alpha.data.daily('msft')
}

