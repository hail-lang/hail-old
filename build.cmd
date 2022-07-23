@echo off
setlocal enableextensions

set GCC_ARGS=-Iinclude

call :Compile "hail_syn" , scanner
call :Compile "" , hail_chars
call :Compile "" , hail_io
call :Compile "" , hail_str
call :Compile "" , main

echo Linking "hail_syn"
ar cq target/hail_syn.a target/hail_syn/scanner.o

echo Linking "hail"
gcc target/hail_chars.o target/hail_io.o target/hail_str.o target/main.o target/hail_syn.a -o target\hail

goto :EOF

:: Compile <dir>, <name>
:Compile

set SRC=src/
if not "%~1"=="" set SRC=%SRC%%~1/
set SRC=%SRC%%~2.c

set OUT_DIR=target\
if not "%~1"=="" set OUT_DIR=%OUT_DIR%%~1\
set OUT_DIR=%OUT_DIR%
set OUT_FILE=%OUT_DIR%%~2.o

if "%~1"=="" ( echo Compiling '%~2' ) else echo Compiling '%~1::%~2'
if not exist %OUT_DIR% md %OUT_DIR%

gcc %SRC% %GCC_ARGS% -c -o %OUT_FILE%
:: gcc src/main.c src/hail_chars.c src/hail_io.c src/hail_str.c -Iinclude

endlocal