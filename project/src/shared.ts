export interface Params {
  symbol: string;
}

export function assert(value: string): asserts value {
  if (!value) {
    throw new Error('A symbol must be specified')
  }
}
