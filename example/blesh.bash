#!/usr/bin/env bash

function binman_install {
	link=https://github.com/akinomyoga/ble.sh.git
	git clone --recursive --depth 1 --shallow-submodules $link
	make -C ble.sh install PREFIX=~/.local
	echo 'source -- ~/.local/share/blesh/ble.sh' >>~/.bashrc
}

function binman_update {
	ble-update
}
