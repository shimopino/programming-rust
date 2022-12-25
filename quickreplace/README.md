# テキストを置換する CLI

```bash
$ echo "Hello, world" > test.txt
$ cargo run "world" "Rust" test.txt test-modified.txt

$ cat test-modified.txt
Hello, Rust
```

間違った場合には下記のように出力される

```bash
$ cargo run "[[a-z]" "0" test.txt test-modified.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/quickreplace '[[a-z]' 0 test.txt test-modified.txt`

Error: failed to replace text: Syntax(
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
regex parse error:
    [[a-z]
    ^
error: unclosed character class
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
)
```
