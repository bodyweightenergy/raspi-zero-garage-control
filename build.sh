#!/bin/bash
# Build and post-build script

# Runs command and gets return code (used for cargo)
safeRunCommand() {
  typeset cmnd="$*"
  typeset ret_code

  echo cmnd=$cmnd
  eval $cmnd
  ret_code=$?
  if [ $ret_code != 0 ]; then
    printf "Error : [%d] when executing command: '$cmnd'" $ret_code
    exit $ret_code
  fi
}

# Read config
HOST=$(cat target_host.txt)

# Build
safeRunCommand "cargo build --target=arm-unknown-linux-gnueabihf"

# Post-build
ssh $HOST "pkill garage_control"	# kill previous executable instance
scp target/arm-unknown-linux-gnueabihf/debug/garage_control $HOST:~/remote_build/garage_control
ssh  $HOST "~/remote_build/garage_control"
