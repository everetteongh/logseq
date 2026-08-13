set shell := ["sh", "-eu", "-c"]

default:
  just --list

doc:
  cargo doc --all-features --no-deps
  git switch pages
  git checkout master -- .gitignore
  rm -rf latest
  mkdir -p latest
  cp -r target/doc/* latest/
