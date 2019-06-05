@echo off
setlocal

:: Pass the EXE file name (excluding the file extension) when calling this BAT file.

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
