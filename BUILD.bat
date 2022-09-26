
REM Set the workdir variable below to the workdir where your dna and happ folders are.
REM See https://github.com/holochain-open-dev/wiki/wiki/Installing-Holochain-Natively-On-Windows for more info...

SET workdir=zomes/workdir

@ECHO OFF
ECHO.
ECHO *************************************************
ECHO NextGen Software Holochain hApp Builder v1.0
ECHO *************************************************
ECHO.
ECHO Building hApp...
ECHO Compiling Zomes...
cargo build --release --target wasm32-unknown-unknown
ECHO Packing DNA..
hc dna pack %workdir%/dna
ECHO Packing HAPP...
hc app pack %workdir%/happ
ECHO Cleaning Sanbox...
hc sandbox clean
ECHO.
ECHO hApp Built Successfully. 
ECHO.
pause
ECHO.
ECHO Running hApp...
ECHO.
hc sandbox generate %workdir%/happ --run=8888 