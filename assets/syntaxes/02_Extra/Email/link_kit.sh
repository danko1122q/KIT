#!/bin/bash

kit --version || brew install kit # Mac OS only

BAT_PATH=$(kit --config-dir)

mkdir -p "$BAT_PATH/syntaxes"
mkdir -p "$BAT_PATH/themes"

INSTALL_PATH=${BAT_PATH}/syntaxes
ln -siv "$PWD/email.sublime-syntax" "$INSTALL_PATH"

kit cache --build
kit --list-languages | grep "Email"

kit demo/email.eml
