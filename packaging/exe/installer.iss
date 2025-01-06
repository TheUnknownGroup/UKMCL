#define Name "UKMCL"
#define Publisher "TheUnknownGroup - devonk15"
#define URL "https://github.com/TheUnknownGroup/UKMCL"
#define ExeName "UKMCL.exe
#define Version "v${{ secrets.VERSION }}"

#define PackagingDir "./Release"
#define BaseFilename "UKMCL-Windows-Installer-en_US"

[Setup]
AppName={#Name}
AppVersion={#Version}
AppVerName={#Name}{#Version}
AppPublisher={#Publisher}
AppPublisherURL={#URL}
AppSupportURL={#URL}
AppUpdatesURL={#URL}
DefaultDirName={autopf}\{#Name}
DisableProgramGroupPage=yes
OutputDir=.
OutputBaseFilename={#BaseFilename}
Compression=lzma
SolidCompression=yes
WizardStyle=modern
SetupIconFile="icon.ico"
VersionInfoCompany={#Publisher}
VersionInfoDescription="This is the Minecraft Launcher for and by the Unknown Group."

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}";
Name: "quicklaunchicon"; Description: "{cm:CreateQuickLaunchIcon}"; GroupDescription: "{cm:AdditionalIcons}";

[Files]
Source: "{#PackagingDir}\*"; DestDir: "{app}"; Flags: recursesubdirs createallsubdirs

[Icons]
Name: "{autoprograms}\{#Name}"; Filename: "{app}\{#ExeName}"
Name: "{autodesktop}\{#Name}"; Filename: "{app}\{#ExeName}"; Tasks: desktopicon

[InstallDelete]
Type: files; Name: "{app}\*.exe"
Type: files; Name: "{app}\*.dll"
Type: files; Name: "{app}\*.dat"
Type: files; Name: "{app}\data"

[Run]
Filename: "{app}\{#ExeName}"; Description: "{cm:LaunchProgram,{#StringChange(Name, '&', '&&')}}"; Flags: nowait postinstall

[Code]

procedure CurPageChanged(CurPageID: Integer);
begin
  if CurPageID = wpPreparing then
    begin
      WizardForm.PreparingYesRadio.Visible := False;
      WizardForm.PreparingNoRadio.Visible := False;
    end;
  end;
