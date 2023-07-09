
REM Set the workdir variable below to the workdir where your dna and happ folders are.
REM See https://github.com/holochain-open-dev/wiki/wiki/Installing-Holochain-Natively-On-Windows for more info...

SET workdir=BUILD

@ECHO OFF
ECHO.
ECHO *************************************************
ECHO NextGen Software Holochain hApp Builder v1.1
ECHO *************************************************
ECHO.
ECHO Building hApp...
ECHO Compiling Zomes...
cargo build --release --target wasm32-unknown-unknown

REM if exist %workdir% goto CREATEDNAFOLDER
REM ECHO Creating Workdir...
REM md  %workdir%
REM IF %ERRORLEVEL% NEQ 0 (Echo Error building the hApp &Exit /b 1)

:CREATEDNAFOLDER
if exist %workdir%\dna\ goto PACKDNA

echo Initializing DNA Folder...
hc dna init %workdir%/dna 
IF %ERRORLEVEL% NEQ 0 (Echo Error building the hApp &Exit /b 1)

:PACKDNA
ECHO Packing DNA...
hc dna pack %workdir%/dna
IF %ERRORLEVEL% NEQ 0 (Echo Error building the hApp &Exit /b 1)

if exist %workdir%\happ\ goto PACKHAPP

echo Initializing HAPP Folder...
hc app init %workdir%/happ
IF %ERRORLEVEL% NEQ 0 (Echo Error building the hApp &Exit /b 1)

:PACKHAPP
ECHO Packing HAPP...
hc app pack %workdir%/happ
IF %ERRORLEVEL% NEQ 0 (Echo Please update the correct path for the DNA in the happ.yaml found in the happ folder in %workdir%. Error building the hApp &Exit /b 1)

ECHO Cleaning Sandbox...
hc sandbox clean
ECHO.
ECHO hApp Built Successfully. 
ECHO.
pause
ECHO.
ECHO Running hApp...
ECHO.
hc sandbox generate %workdir%/happ --run=8888 

