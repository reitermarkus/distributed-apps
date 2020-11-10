# Homework 3

## Dependencies

- `node`/`npm`
- `ibmcloud`

## Running

Deploy and run the functions using `./deploy.sh`.

## Results

| Region   | Type   | N | k              | Memory | Time    |
|----------|--------|---|----------------|--------|---------|
| `eu-gb`  | manual | 9 |                | 128 MB | 133440 ms |
| `eu-gb`  | x2faas | 9 |                | 128 MB | 123064 ms |
| `eu-gb`  | FC     | 9 | 2 (9 workers)  | 128 MB |  48235 ms |
| `eu-gb`  | FC     | 9 | 3 (27 workers) | 128 MB |  51979 ms |
| `eu-gb`  | FC     | 9 | 4 (81 workers) | 128 MB |  46870 ms |

Looking at the results, we can see that we can more than double the response
time when splitting the processing across multiple function calls using a
function choreography. However, this only scales up to a certain extent on
IBM Cloud. Using 27 workers and 81 workers only slightly change the response
time. This is probably due to function containers having to be spun up when
using more workers or there being a limit on how many concurrent
functions can be run on the free tier. This can be seen in the output, since
the first 17 function calls (for k = 4) finish in about 10 s while the
remaining all take about 40 s to complete.
