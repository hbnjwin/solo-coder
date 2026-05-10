@echo off
setlocal

set "VSWHERE=C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe"
set "VCVARS="

if exist "%VSWHERE%" (
  for /f "usebackq delims=" %%I in (`"%VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do (
    if exist "%%~I\VC\Auxiliary\Build\vcvars64.bat" (
      set "VCVARS=%%~I\VC\Auxiliary\Build\vcvars64.bat"
    )
  )
)

if not defined VCVARS if exist "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvars64.bat" (
  set "VCVARS=C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
)

if not defined VCVARS (
  echo vcvars64.bat not found. Please install Visual Studio Build Tools with C++ support.
  exit /b 1
)

call "%VCVARS%"
%*
