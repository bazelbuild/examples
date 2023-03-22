@echo off

set librarian_dir=%~dp0
set librarian_dir=%librarian_dir:~0,-1%
python "%librarian_dir%\librarian.py" %* || ( exit /b )
