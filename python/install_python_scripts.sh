#!/bin/bash
if [ !-d "$HOME/.local/bin" ];then
  mkdir -p $HOME/.local/bin
fi

for script in "scripts/*.py";do
  cp $script $HOME/.local/bin
done

echo "Make sure that $HOME/.local/bin is in PATH."
