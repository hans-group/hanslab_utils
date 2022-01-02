#!/bin/bash
function install_rust() {
    echo "Rust not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
}

function install_rust_binaires() {
    BINARIES="$(ls ../rust/binary)"
    for bin in ${BINARIES[@]};do
        cargo install --git https://github.com/mjhong0708/hanslab_utils $bin
    done
}

# configure pos2pot
function configure_pos2pot() {
    if [ ! -d "$HOME/.config/pos2pot" ];then
        mkdir -p "$HOME/.config/pos2pot"
    fi
    
    cd "$HOME/.config/pos2pot"
    
    if [ -f "$HOME/.config/pos2pot/potcar.json" ];then
        rm "$HOME/.config/pos2pot/potcar.json"
    fi
    
    if [ -f "$HOME/.config/pos2pot/config.json" ];then
        rm "$HOME/.config/pos2pot/config.json"
    fi
    
    wget "https://mjhong-public.s3.ap-northeast-2.amazonaws.com/pos2pot/potcar.json"
    wget "https://mjhong-public.s3.ap-northeast-2.amazonaws.com/pos2pot/config.json"
}

### Main task starts ###
### Install rustup if not found
if ! command -v cargo &> /dev/null; then
    install_rust()
    source $HOME/.cargo/env
fi
### Install binary crates
install_rust_binaires()
### Configure pos2pot automatically
configure_pos2pot()