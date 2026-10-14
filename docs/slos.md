## Site reliability engineering

- SLIs: Service level Indicators: Measurement which is used to evaluate the behaviour of ferrite
- SLOs: Servive level Objective: The target value that the SLIs indicated
- SLAs: Service level Agreements

1. Durability: Every acknowleged durable write must survive any single-node crash

2. Consistency: Every read and write must follow the linear and atomic operations each behaved atomically

3. Availability: The cluster should be accepting operations requiring consensus from the raft or something while the majority is available. A three node server should tolerate the one failed node.

4. Latency: Measure p50, p95 and p99 seperately for read and writes.

5. Recovery: Measure leader election time, follow catch-up time and restart recovery time

6. Throughput: Measure the successful operations per second under the specified concurrency, request-size and read/write mixes

