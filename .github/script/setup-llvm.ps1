choco install llvm
echo "LIBCLANG_PATH=$env:SystemDrive\Program Files\LLVM\bin" | Out-File -FilePath $env:GITHUB_ENV -Encoding utf8 -Append
