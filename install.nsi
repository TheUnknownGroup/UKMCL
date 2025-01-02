OutFile "UKMCL-Windows-en_US-v0-0-0-experimental+0.exe"

InstallDir "$PROGRAMFILES\UKMCL"

RequestExecutionLevel admin

Page directory
Page instfiles

Section "Install"
    SetOutPath "$INSTDIR"
    File /r "build\windows\x64\runner\Release\*"
    CreateShortCut "$DESKTOP\UKMCL.lnk" "$INSTDIR\UKMCL.exe"
    CreateShortCut "$SMPROGRAMS\UKMCL\UKMCL.lnk" "$INSTDIR\UKMCL.exe"

SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\*.*"
    RMDir "INSTDIR"

    Delete "$DESKTOP\UKMCL.lnk"
    Delete "$SYPROGRAMS\UKMCL\UKMCL.lnk"
    RMDir "$SYPROGRAMS\UKMCL"

SectionEnd