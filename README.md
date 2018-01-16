tcpbench: a silly tcp benchmarking experiment


I wanted to find out how much extra time my NAT gateway added. So, I've created a simple tool to try and find out how long delay seems to be.

Usage: `tcpbench (client | server) server:port`

Server mode currently doesn't work; use: 
```
while true; do; echo -n 1 | nc -ltp 9999; done
```

Will print out:
- initial connection time
- best estimate of RTT (todo)
