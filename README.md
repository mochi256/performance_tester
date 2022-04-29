# performance_tester
executing performance test by  rust.

## usage

### server start
```bash
git clone git@github.com:mochi256/performance_tester.git
cd performance_tester/server/

cargo run -- -c 100000 -r 1000
```

### client start
```bash
git clone git@github.com:mochi256/performance_tester.git
cd performance_tester/client/

cargo run -- --host 127.0.0.1 -p 8000
```