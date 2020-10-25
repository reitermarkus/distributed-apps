const nqueens = require('./nqueens')

// IBM Entrypoint
module.exports.main = (event) => {
  return nqueens.fraction({params: event});
}

// Amazon Entrypoint
module.exports.handler = (event, context) => {
  try {
    context.succeed(this.main(event))
  } catch (e) {
    context.fail(e)
  }
}
