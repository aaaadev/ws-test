# Simple websocket k6 load test
## How it works
- Websocket server (actix-web) sends random data of fixed size (`DATA_SIZE`) whenever 1s.
- Client (k6) accepts this binary data
## Test results
### Environment
Macbook Pro "14 (2023, M2 Max)
### List
| size | throughput | link |
| - | --- | --- |
| 1M  | 100MB/s  | [here](./results/1M.txt) |
## Structure
```mermaid
graph TD
    subgraph "Client"
        Clients(User)
    end

    subgraph "Infra (OVHCloud)"
        subgraph "Application server"
            API(actix-web + poller)
        end
        
        subgraph "Middleware"
            DB(ClickHouse DB)
            Grafana(Grafana)
        end
    end

    Clients -- "WS connection" --> API
    API -- "Polling to query data" --> DB
    API -- "In-memory cache" --> API
    Grafana -- "Query to visualize data" --> DB

```