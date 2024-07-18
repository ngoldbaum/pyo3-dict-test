Run like this:

```
± PYTHONPATH=$PWD cargo run
   Compiling test-dict v0.1.0 (/Users/goldbaum/Documents/pyo3-test/test-dict)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/test-dict`
[src/main.rs:11:9] d.get_item("hello").unwrap().unwrap() = 'world'
Error: PyErr { type: <class 'TypeError'>, value: TypeError(), traceback: Some(<traceback object at 0x100d126c0>) }
```
