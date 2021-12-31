#!/bin/bash
BINARIES=("pos2pot" "ndstat")

if [ ! -d "$HOME/.local/bin" ];then
    echo "Creating $HOME/.local/bin"
    mkdir -p "$HOME/.local/bin"
fi

if [[ $PATH != *"$HOME/.local/bin"* ]]; then
    echo "Adding $HOME/.local/bin to PATH"
    echo "export PATH=$HOME/.local/bin:$PATH" >> "$HOME/.bashrc"
fi

# Install rustup
if ! command -v cargo &> /dev/null; then
    echo "Rust not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

cargo build --release
for bin in ${BINARIES[@]};do
    cp target/release/$bin $HOME/.local/bin/
done


# configure pos2pot
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