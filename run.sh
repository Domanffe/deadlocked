#!/usr/bin/env bash

fail() {
	echo "Not a git repository (or any of the parent directories): .git.
Do NOT download the repository as a zip file from GitHub.com!
Please download deadlocked by cloning the Git repository: 'git clone https://github.com/Domanffe/deadlocked'"
	exit 1
}

[[ -d '.git' ]] || fail

git pull
cargo run --release
