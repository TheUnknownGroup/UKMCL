!macro NSIS_HOOK_POSTINSTALL
  Var /GLOBAL UkMCLFile
  Var /GLOBAL UkMCLHandle

  FindFirst $UkMCLHandle $UkMCLFile "$INSTDIR\ukmcl-*-windows_x64.exe"

  ${If} $UkMCLFile != ""
    ExecShell "" "$INSTDIR/$UkMCLFile"
  ${EndIf}

  FindClose $UkMCLHandle
!macroend