# How to run 

You'll need `cargo` along with a Rust environment.
`cargo run` will run the program. 

Upon running, it directly starts to read on stdin, eg
```
cat testfile.txt | cargo run
```

I've modified a little bit the provided snippet to ease with error handling.
There is also 2 testfiles, named `testfile.txt` and `testfile2.txt` containing the subject's examples.


## how to test

`cargo test` executes tests.
