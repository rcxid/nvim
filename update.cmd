@echo off
cargo build --release
del /Q ".\lua\*.dll"
copy ".\target\release\*.dll" ".\lua\"
