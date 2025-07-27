# Simple websocket k6 load test
## How it works
- Websocket server (actix-web) sends random data of fixed size (`DATA_SIZE`) whenever 1s.
- Client (k6) accepts this binary data
## Test results
### Environment
Macbook Pro "14 (2023, M2 Max)
### List
| size | checks_succeeded | link |
| - | --- | --- |
| 1M  | 52.91%  | [here](./results/1M.txt) |
| 512K  | 59.06%  | [here](./results/512K.txt) |
| 256K  | 100%  | [here](./results/256K.txt) |