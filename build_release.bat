@echo off

:: Get an output dir
if exist .\publish\ rmdir /S /Q .\publish
mkdir publish

:: Release build into dir
cargo build --release
copy target\release\*.exe .\publish\

:: GTK .dll dependencies into the publish folder
copy %VCPKGDIR%\installed\x64-windows\bin\*.dll .\publish\

:: Get The icons copied
mkdir publish\share\icons

:: TODO: Only copy the icons we're actually using....
:: Remming out until then as it's waaay too many files just for minimise icons...
::xcopy %VCPKGDIR%\installed\x64-windows\share\icons publish\share\icons /S

copy publish_readme.txt publish\readme.txt
