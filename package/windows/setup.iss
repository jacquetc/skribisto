; Inno Setup script for the Rust + Teksilo Skribisto.
;
; The Rust app is a single self-contained skribisto.exe — no Qt DLLs, no
; sqldrivers, no OpenSSL (rustls), no windeployqt. CI builds the exe with
; `-C target-feature=+crt-static`, so there is no vcruntime140.dll to ship
; either. This script therefore installs exactly one binary plus its icon.
;
; Overridable defines (CI passes /D...):
;   MyAppVersion  - the release version (defaults to a dev placeholder)
;   MySourceExe   - path to the built skribisto.exe
; Example:
;   ISCC.exe /DMyAppVersion=3.0.0 /O"dist" /F"Skribisto-setup" package\windows\setup.iss

#define MyAppName "Skribisto"
#ifndef MyAppVersion
  #define MyAppVersion "0.0.1"
#endif
#define MyAppPublisher "Skribisto"
#define MyAppURL "https://github.com/jacquetc/skribisto"
#define MyAppExeName "skribisto.exe"
#ifndef MySourceExe
  #define MySourceExe "..\..\target\x86_64-pc-windows-msvc\release\skribisto.exe"
#endif

[Setup]
; AppId uniquely identifies the application for install/upgrade/uninstall.
AppId={{4C264D27-1549-415D-AEA4-0E416409C175}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
LicenseFile=..\..\COPYING
OutputBaseFilename=Skribisto-setup
Compression=lzma2/ultra64
SolidCompression=yes
OutputDir=..\..\..\Output
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
ChangesAssociations=yes
UninstallDisplayIcon={app}\skribisto.ico
UninstallDisplayName=Skribisto
WizardStyle=modern
SetupIconFile=..\..\resources\windows\skribisto.ico

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "french"; MessagesFile: "compiler:Languages\French.isl"
Name: "german"; MessagesFile: "compiler:Languages\German.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"

[Files]
Source: "{#MySourceExe}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\resources\windows\skribisto.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\skribisto.ico"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\skribisto.ico"; Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[Registry]
; .skrib project-file association (per-machine; installer requires admin).
Root: HKCR; SubKey: ".skrib"; ValueType: string; ValueData: "Skribisto"; Flags: uninsdeletekey
Root: HKCR; SubKey: "Skribisto"; ValueType: string; ValueData: "Skribisto project file"; Flags: uninsdeletekey
Root: HKCR; SubKey: "Skribisto\Shell\Open\Command"; ValueType: string; ValueData: """{app}\{#MyAppExeName}"" ""%1"""; Flags: uninsdeletekey
Root: HKCR; Subkey: "Skribisto\DefaultIcon"; ValueType: string; ValueData: "{app}\skribisto.ico,0"; Flags: uninsdeletevalue
