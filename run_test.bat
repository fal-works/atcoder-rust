@echo off
setlocal

:: Call this BAT from each project folder and pass the current directory path
set NAME=%~n1

for %%F in (stdin\*.txt) do (
  echo ----------------------------------------
  echo [ Test case: %%~nF ]
  echo.
  echo input:
  type %%F
  echo.
  echo output:
  type %%F | target\debug\%NAME%.exe
  echo.
)

endlocal
