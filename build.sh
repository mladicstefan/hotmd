#!/bin/bash

set -e

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}Starting build...{$NC}"

if [[ ! -f "client.lua" ]]; then
  echo -e "${RED} Client.lua not found.${NC}"
  exit 1
fi

mkdir -p ~/.config/nvim/lua/ 

echo "Copying client.lua to nvim config..."
cp client.lua ~/.config/nvim/lua

trap "rm ~/.config/nvim/lua/client.lua" EXIT 

echo "Starting server..."
cargo r -- -f example.md 

echo -e "${GREEN} Built sucessfully. {$NC}"
