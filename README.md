# Reproducible build

```
# build
docker run --rm -v $PWD:/build -it ellipsislabs/solana:1.16.16 sh -c "cargo build-bpf -- --locked --frozen"

# get sha and size
sha256sum target/deploy/instruction_bundler.so
> d19f558c4f0eb09e390882aa336debfc3f80fe03fbd4890fe61ffd1c193d8179  target/deploy/instruction_bundler.so
ls -l target/deploy/instruction_bundler.so
> -rwxr-xr-x 1 root root 28336 Apr 11 09:43 target/deploy/instruction_bundler.so

# verify (3c72 is the executable data for ixBu)
solana account 3c726Nn5yEA6uyDCCEsLnq7uNWMD1qqW35SRVs7xmupo -o /tmp/program.bytes
tail -c '+46' /tmp/program.bytes | head -c 28336 | sha256sum
> d19f558c4f0eb09e390882aa336debfc3f80fe03fbd4890fe61ffd1c193d8179  -
```
