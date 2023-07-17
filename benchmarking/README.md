# Kakarot Benchmarking

The directory contains scripts which we use to benchmark Kakarot, as of now we are using aritillery to create virtual users and test the RPC.

Steps to benchmark:
- start Madara { the last benchmarks were done on the [following](https://github.com/keep-starknet-strange/madara/tree/c46c02e6b1fe7927143d17f14ed0eec6f62f7031) version of Madara }
- deploy Kakarot:
  - comment the [following](https://github.com/kkrt-labs/kakarot/blob/42521e74a83772715db93f7b53a7df43aa3289fa/src/utils/eth_transaction.cairo#L277) line { disable nonce validation during trasnsaction validation } 
  - complile the contracts
  - deploy Kakarot on Madara
- install node modules: `npm i` 
- run the benchmark: `npm run benchmark`
- a report file will be dumped in `reports` directory, you can check the benchmarking result there.