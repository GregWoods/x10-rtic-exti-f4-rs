# x10-rtic-exti-f4-rs

A microcontroller template for the STM32F411 Black Pill board.


This is a template for your own README.


## Step 1 - use ```cargo generate``` to "clone" this template

* Install [cargo generate](https://crates.io/crates/cargo-generate)
* Install the very useful [GitHub cli](https://cli.github.com/)

```
cargo generate --git https://github.com/GregWoods/x08-outputcompare-f4-rs.git --name x10-rtic-exti-f4-rs
gh repo create x10-rtic-exti-f4-rs
cd x10-rtic-exti-f4-rs
git add .
git commit -m "generated from a template by https://github.com/GregWoods"
git push -u origin master
code .
```


## Step 2 - Build and Debug

* Plug your STLink-v2 clone into you PC, with the headers connected correctly to the Blue Pill
* `cargo build` from the VS Code built-in Terminal (Ctrl+')
* Open the debugging sidebar in VS Code (Ctrl+Shift+D)
* Hit "play" Debug (OpenOCD)