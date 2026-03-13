@echo off
set "VSDIR=D:\VisualStudio\Common7\IDE\VC\Auxiliary\Build"

echo Setting up Visual Studio environment...
call "%VSDIR%\vcvars64.bat"

echo Starting Tauri dev...
npx tauri dev
