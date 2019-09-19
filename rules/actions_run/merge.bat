@echo OFF
set files=

set out=%1

:addparam
shift
if "%1"=="" goto paramscomplete
set files=%files% %1
goto addparam
:paramscomplete

type %files:/=\% > %out:/=\%
