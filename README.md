# PassGen

A super simple password generator in Rust.

I realise this is pretty messy and not really optimised. There are giant arrays of the values that could probably be in external files but, it worked and didn't require anything fancy and produces a relatively small binary.

## Why?
Because I wanted to. In all seriousness this seemed like a good opportunity to learn a little more Rust and get something out of it that I could actually use.

## How?
Download one of the binaries and call it:

|Command|Output|
|-------|------|
|`pass_gen 5 --numbers --upper`| `TranquilSituationShiftStampLay7719941`|
|`pass_gen 5`|`oiljokescribbleassumptionduce`|
|`pass_gen 8 --numbers --upper`|`RinseCriticizeContextGoodWhineHighwayPreferPhoto1140204`

Or alternatively, clone the repo and build it:

`cargo build --release` and call it from the repo directory: `./target/release/pass_gen 5 --numbers --upper`
