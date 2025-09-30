Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression
# Install main applications
scoop install git neovim-nightly cmake ninja clangd main/gcc
# Clone neovim config
git clone https://github.com/juancamilo0509/Snvim.git
mv Snvim $env:LOCALAPPDATA\nvim
