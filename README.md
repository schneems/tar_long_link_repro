## Cannot unpack with @LongLink

## Expected

I have an archive that contains a long link:

```
././@LongLink
jruby-9.4.8.0/lib/ruby/stdlib/bundler/vendor/molinillo/lib/molinillo/modules/specification_provider.
```

You can view it in the `fixtures/` directory.

I expect that when I call `tar::Archive.unpack()` the full file names will be there. Or alternatively that there will be some documented way of unpacking a file with GNU long link names.

## Actual

```
$ git clone https://github.com/schneems/tar_long_link_repro
$ cd tar_long_link_repro
```

```
$ cargo run
   Compiling tar_long_link_repro v0.1.0 (/Users/rschneeman/Documents/projects/tmp/tar_long_link_repro)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/tar_long_link_repro`
thread 'main' panicked at src/main.rs:30:5:
expected ["resolution_state.rb", "specification_provide"] to include "specification_provider.rb" but it did not
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The names such as `specification_provider.rb` are truncated in the unarchived file.

## More debug output

Unarchiving via the `tar` command on Mac (bsdtar 3.5.3)

```
$ rm -rf tmp/cli-extracted; mkdir -p tmp/cli-extracted && cd tmp/cli-extracted && tar xzf ../../fixtures/jruby-dist-9.4.8.0-bin.tar.gz && cd -
```

Shows the file:

```
$ ls tmp/cli-extracted/jruby-9.4.8.0/lib/ruby/stdlib/bundler/vendor/molinillo/lib/molinillo/delegates
resolution_state.rb		specification_provider.rb
```

While the rust version does not:

```
$ ls tmp/rust-extracted/jruby-9.4.8.0/lib/ruby/stdlib/bundler/vendor/molinillo/lib/molinillo/delegates
resolution_state.rb	specification_provide
```
