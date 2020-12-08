# Homework 4

## Dependencies

- `node`/`npm`
- `ibmcloud`

## Running

Deploy and run the functions using `./deploy.sh`.

Run using `./run.sh`.

## Results from Homework 4


| Regions                                | Type    | N | k              | Memory | Time      | Failures |
|----------------------------------------|---------|---|----------------|--------|-----------|----------|
| `eu-gb`                                | manual  | 9 |                | 128 MB | 133440 ms | 0        |
| `eu-gb`                                | x2faas  | 9 |                | 128 MB | 123064 ms | 0        |
| `eu-gb`                                | FC      | 9 | 2 (9 workers)  | 128 MB |  48235 ms | 0        |
| `eu-gb`                                | FC      | 9 | 3 (27 workers) | 128 MB |  51979 ms | 0        |
| `eu-gb`                                | FC      | 9 | 4 (81 workers) | 128 MB |  46870 ms | 0        |
| `en-de`, `eu-gb`, `jp-tok`             | FC      | 9 | 2 (9 workers)  | 128 MB |  22396 ms | 0        |
| `en-de`, `eu-gb`, `jp-tok`             | FC      | 9 | 3 (27 workers) | 128 MB |  20004 ms | 0        |
| `en-de`, `eu-gb`, `jp-tok`             | FC      | 9 | 4 (81 workers) | 128 MB |  19701 ms | 0        |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 2 (9 workers)  | 128 MB |  32943 ms | 1        |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 2 (9 workers)  | 128 MB |  33742 ms | 2        |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 2 (9 workers)  | 128 MB |  36617 ms | 3        |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 2 (9 workers)  | 128 MB |  35174 ms | 4        |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 3 (27 workers) | 128 MB |  20691 ms | 1        |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 3 (27 workers) | 128 MB |  20902 ms | 2        |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 3 (27 workers) | 128 MB |  21398 ms | 3        |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 3 (27 workers) | 128 MB |  21952 ms | 4        |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 3 (27 workers) | 128 MB |  23842 ms | 5        |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 3 (27 workers) | 128 MB |  25653 ms | 6        |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 3 (27 workers) | 128 MB |  27801 ms | 7        |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 3 (27 workers) | 128 MB |  30821 ms | 9        |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 3 (27 workers) | 128 MB |  34366 ms | 11       |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 4 (81 workers) | 128 MB |  24469 ms | 13       |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 4 (81 workers) | 128 MB |  24545 ms | 14       |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 4 (81 workers) | 128 MB |  25707 ms | 15       |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 4 (81 workers) | 128 MB |  25463 ms | 16       |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 4 (81 workers) | 128 MB |  30664 ms | 17       |
| `en-de`, `eu-gb`, `jp-tok` + `us-east` | FC + FT | 9 | 4 (81 workers) | 128 MB |  29397 ms | 20       |

As we can see from the results, each additional failure adds an almost linear amount of time:

- For `k = 4`, this is around 250 - 500 ms per failure.
- For `k = 3`, this is around 500 - 1000 ms per failure.
- For `k = 2`, this is around 1000 - 2000 ms per failure.
