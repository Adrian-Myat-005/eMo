#!/bin/bash
# eMo Nexus Integration Test
# Using binaries from ~/.emo/bin

EMO="$HOME/.emo/bin/emo"
HAPPY="$HOME/.emo/bin/happy"

echo -e "\033[1;36m◈ Starting eMo Nexus Integration Test...\033[0m\n"

echo -e "\033[0;34m[1/3] Testing eMo Core...\033[0m"
$EMO run src/logic.emo

echo -e "\n\033[0;34m[2/3] Testing HappyCry Interface...\033[0m"
$HAPPY build src/interface.hpy
./interface

echo -e "\n\033[0;34m[3/3] Testing SadSmile Kernel...\033[0m"
$EMO run src/kernel.ss

echo -e "\n\033[1;32m✔ Nexus Workflow Test Complete.\033[0m"
