<h1 align="center">RS-RUN</h1>

<div align="center">
🎉 Run `hello_world.rs` without manually calling the compiler like a script 🦀
</div>

## Example

```text
#> cat << EOF > hello_world.rs
fn main() {
    println!("Hello RS-RUN!");
}
EOF
#> chmod +x hello_world.rs
#> ./hello_world.rs
Hello RS-RUN!
```

- ❌ No this is NOT an elegant implementation.
- ✅ But it works :)
- 😜 And it's definitely not the first implementation, nor will be the last.

## How to use

*You'll need rust toolchain available in your path!*

On systemd enabled hosts:

- build and copy the binary to `/usr/local/bin/rs-run`
- write the config in `/etc/binfmt.d/rs-run.conf`
    - content: `:rs-run:E::rs::/usr/local/bin/rs-run:P`
- and then restart `systemd-binfmt.service`
- now you can execute `.rs` files directly, if files are given execution permission

## How it works

- binfmt(binfmt_misc)
    - kernel handles which "interpreter" to use to run the executables
    - `:rs-run:E::rs::/usr/local/bin/rs-run:P`
        - `:rs-run`: rule name
        - `:E`: match by extension
        - `:rs`: match ".rs"
        - `:/usr/local/bin/rs-run`: interpreter path
        - `:P`: the "script" path should be passed as the first argument
        - read more at https://docs.kernel.org/admin-guide/binfmt-misc.html
- binary cache
    - `rs-run` calls rustc on demand, and stores the binary to cache dir
    - located at `$HOME/.cache/rs-run-binaries` or `./.cache/rs-run-binaries`
    - the lookup key is sha256sum of the source code file's full path

## Limitations

- only std can be used
    - for more complex projects you should go with cargo
    - but if you insist...
- only on linux, because it's kernel assisted

## Todos

- implement command line interface
    - build command, compile single file to binary
    - clean command, clean binary cache
- implement inline dependency config
    - that's to say, build with cargo
    - mimic the behavior of `uv run`
- implement automatic configuration

## Notes

### Clean the binary cache

Instead of deleting the whole cache folder, you can delete old binaries only. To be integrated.

```bash
# This is bash script
current_date=$(date +%s)
time_diff=2592000 # 30 days in second
((old_time=current_date-time_diff))
find "$HOME/.cache/rs-run-binaries" -type f | while read -r line; do
    modified=$(stat -c %Y "$line")
    if [ $modified -lt $old_time ]; then
        printf "Removing %s\n" "$line"
        rm "$line"
    fi
done
```

### How other programs work

- Who use binfmt(kernel assisted)
    - Wine
    - QEMU
    - Appimage(only appimagetool)
- Who use shebang(shell assisted)
    - Most shells including bash
    - Python(just compatible, because its comment style)
