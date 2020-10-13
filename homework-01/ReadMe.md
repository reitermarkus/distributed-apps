# Homework 1

## Dependencies

- `ibmcloud`
- `java` (>= 14)
- `gradle`

## Running

Deploy and run example function calls using `./deploy.sh`.

Try in a browser (takes approx. 5 seconds):

- https://eu-gb.functions.appdomain.cloud/api/v1/web/62ea098b-618c-465f-8bed-aa659df82e70/default/nqueens.json?board_size=8
- https://jp-tok.functions.appdomain.cloud/api/v1/web/9e571da3-c444-42ab-93bc-8174744a38d5/default/nqueens.json?board_size=8

## Results

| Region   |  k | N | Memory | Total Time (ns) |  Time (ns) |
|----------|----|---|--------|-----------------|------------|
| `eu-gb`  |  2 | 8 |    128 |      6887155135 | 3441473544 |
| `eu-gb`  | 10 | 8 |    128 |     29566067156 | 2956180310 |
| `jp-tok` |  2 | 8 |    128 |      7056027286 | 3526126858 |
| `jp-tok` | 10 | 8 |    128 |     32895996968 | 3289191902 |

Looking at the results, we can see that when invoking the function multiple
times, some of the latency overhead seems to be reduced.

## Bonus Result

| Region   |  k | N | Memory | Total Time (ns) |  Time (ns) |
|----------|----|---|--------|-----------------|------------|
| `eu-gb`  |  2 | 8 |   2048 |      6561338211 | 3278665165 |
| `eu-gb`  | 10 | 8 |   2048 |     29641387456 | 2963687380 |
| `jp-tok` |  2 | 8 |   2048 |      7287299359 | 3641724295 |
| `jp-tok` | 10 | 8 |   2048 |     34209182520 | 3420504571 |

The result when using 2048 MB of memory don't differ substantionally from the
ones using only 128 MB, indicating that the problem is compute bound and not
memory bound.
