# Battlesnake Minimax Template

Using this template you can create your own snake to play on play.battlesnake.com!


## Usage

```bash
cargo install cargo-generate
cargo generate gh:coreyja/battlesnake-minimax-template
```

## Speed Run Script

Installing `cargo-generate` from source takes too long for a speed run.
So we need to use a version from Github Releases.
And I'm running on OSX, so need the `apple-darwin` version. Unfortunately their latest release
for Apple appears to be broken. Version `v0.11.1` however is working! It's the latest working version I found

```
gh release download v0.11.1 -R cargo-generate/cargo-generate --pattern "*darwin*" -D tmp

tar -xvf tmp/cargo-generate-v0.11.1-x86_64-apple-darwin.tar.gz -C ~/bin cargo-generate

chmod +x ~/bin/cargo-generate

cargo generate coreyja/battlesnake-minimax-template
```
