```
oha -n 1000 -c 100 \
  -m POST \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{"command":"pwd","cwd":"/home"}' \
  http://localhost:3000/terminal
```

Before:

* Average: **174.1 ms**
* Requests/sec: **525.6**

After:

* Average: **174.6 ms**
* Requests/sec: **522.0**


**Problem**

* Lesson progression performed an unnecessary database lookup to retrieve the current lesson before advancing.

**Optimization**

* Reused the lesson already loaded during validation instead of querying the database again, eliminating one redundant database read per successful lesson progression.

**Measurement**

| Metric                |      Before |       After |
| --------------------- | ----------: | ----------: |
| Success Rate          |        100% |        100% |
| Average Response Time |    174.1 ms |    174.6 ms |
| Throughput            | 525.6 req/s | 522.0 req/s |

**Explanation**

> Although the measured latency remained effectively unchanged under this workload, the optimization reduces one database read for every successful lesson completion. Under larger workloads or more complex lesson queries, this reduces unnecessary database operations and improves scalability.

