## tcpbench: a silly tcp benchmarking experiment

I wanted to find out how much extra time my NAT gateway added. So, I've created a simple tool to try and find out how long delay seems to be.

Eventually, the tool could be useful. Right now it just shows a histogram of connection times and RTT. For extra lulz, run it on localhost and see how jittery your network stack is.

Lessons learned:
1. Just setting NO_DELAY on a Linux TCP stream gives a RTT of about 10ms more on a 195ms RTT path.
2. NAT sucks. On my home connection, IPv6 is 20ms faster than IPv4.

Usage: `tcpbench (client | server) host:port`

Will print out histograms of:
- initial connection time
- approximate RTT

## TODO
- report and handle failures more gracefully

## Ideas for improvement
- self-calibrate on localhost first
- Server self-calibrates, sends jitter information
- test bandwidth
