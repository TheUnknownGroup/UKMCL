!macro NSIS_HOOK_POSTINSTALL
  ExecShell "" "$INSTDIR/ukmcl-0.0.1-windows_x64.exe"
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
     RMDir /r "$PROFILE\.ukmcl"
!macroend