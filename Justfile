export RUST_BACKTRACE := "full"

# Lists all the available commands
default:
  @just --list

# Run cargo check for all the projects
cargo_check:
  just for_each_project "cargo check"

# Run cargo fmt for all the projects
cargo_fmt:
  just for_each_project "cargo fmt"

# Run cargo clippy for all the projects
cargo_clippy:
  just for_each_project "cargo clippy"

# Run cargo test for all the projects
cargo_test:
  just for_each_project "cargo test"

# Run cargo build for all the projects
cargo_build:
  just for_each_project "cargo build"

# Run cargo build release for all the projects
cargo_build_release:
  just for_each_project "cargo build --release"


[private]
for_each_project command:
  #!/usr/bin/env bash
  set -e
  set -x

  declare -a projects=(
      "boing-ggez"
      "bunner-macroquad"
      "catacomb_2-sdl2"
      "cavern-macroquad"
      "flappy-bird-bevy"
      "invaders-bevy"
      "pong-macroquad"
      "rusty_roguelike-macroquad"
      "soccer-fyrox"
      "space-shooter-macroquad"
  )

  for i in "${projects[@]}"
  do
      LINE_SEPARATOR='--------------------------------------------------------'

      cd $i
      {{command}}
      cd ..

  done