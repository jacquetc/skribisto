; NSIS (Nullsoft Scriptable Install System) script for the Rust + Teksilo Skribisto.
;
; The Rust app is a single self-contained skribisto.exe — no Qt DLLs, no
; sqldrivers, no OpenSSL (rustls), no windeployqt. CI builds the exe with
; `-C target-feature=+crt-static`, so there is no vcruntime140.dll to ship
; either. This script therefore installs exactly one binary plus its icon.
;
; Because there is nothing machine-wide to register — no service, no driver,
; no shared runtime — the installer offers both scopes and lets the user pick:
;
;   All users   $PROGRAMFILES64\Skribisto      HKLM   needs admin
;   Just me     $LOCALAPPDATA\Skribisto        HKCU   no admin required
;
; Caveat on the UAC prompt: MULTIUSER_EXECUTIONLEVEL Highest means Windows
; hands an administrator the elevated token up front, so an admin account sees
; one UAC prompt at launch even if they then choose "Just me". A standard user
; sees no prompt at all and goes straight to a per-user install. Stock
; MultiUser.nsh cannot elevate partway through; avoiding that first prompt for
; admins would need the third-party NsisMultiUser plugin.
;
; Overridable defines:
;   APP_VERSION      - the release version (defaults to a dev placeholder)
;   APP_VERSION_NUM  - the a.b.c.d VERSIONINFO quad (derived from APP_VERSION)
;   SRC_EXE          - path to the built skribisto.exe
;   SRC_ICO          - path to skribisto.ico
;   SRC_LICENSE      - path to the licence shown on the licence page
;   OUT_FILE         - path of the setup .exe to write
;
; IMPORTANT: makensis chdirs to the directory holding this script (see /NOCD),
; so every relative default below is relative to package\windows and stays
; correct however the script is invoked. A path passed on the command line is
; NOT rewritten to match, so pass ABSOLUTE paths when overriding.
;
; This compiles with the Debian/Ubuntu `nsis` package as well as NSIS on
; Windows. Two differences on POSIX: options take a dash rather than a slash
; (-DAPP_VERSION=..., not /DAPP_VERSION=...), and the relative defaults above
; are spelled with backslashes. build.py overrides all four paths with absolute
; native ones, which is the supported way to build this from Linux.
;
; Example (Windows):
;   makensis /DAPP_VERSION=3.0.0 /DOUT_FILE=C:\out\Skribisto-setup.exe ^
;            package\windows\setup.nsi
; Either host, and what CI runs:
;   python3 package/windows/build.py --version 3.0.0
;
; The built installer accepts /S (silent), /AllUsers or /CurrentUser to pick
; the scope non-interactively, and /D=<dir> to override the location.

Unicode true
ManifestDPIAware true

;--------------------------------
; Product metadata

!define APP_NAME      "Skribisto"
!ifndef APP_VERSION
  !define APP_VERSION "3.0.0-alpha5"
!endif
!define APP_PUBLISHER "Skribisto"
!define APP_URL       "https://github.com/jacquetc/skribisto"
!define APP_EXE       "skribisto.exe"
!define APP_ICO       "skribisto.ico"
!define APP_PROGID    "Skribisto"
!define APP_EXT       ".skrib"

!ifndef SRC_EXE
  !define SRC_EXE     "..\..\target\release\skribisto.exe"
!endif
!ifndef OUT_FILE
  !define OUT_FILE    "..\..\..\Output\Skribisto-setup.exe"
!endif
!ifndef SRC_ICO
  !define SRC_ICO     "..\..\resources\windows\skribisto.ico"
!endif
!ifndef SRC_LICENSE
  !define SRC_LICENSE "..\..\COPYING"
!endif

; Where we keep our own settings, and the Add/Remove Programs entry. Both live
; under HKLM or HKCU according to the scope the user picked, which is what the
; SHCTX register below resolves to.
!define APP_REGKEY    "Software\${APP_NAME}"
!define UNINST_KEY    "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}"

; The AppId of the superseded Inno Setup installer. Releases up to and
; including 3.0.0-alpha5 shipped with Inno; without this, upgrading would
; leave a second, orphaned entry in Add/Remove Programs pointing at files
; this installer has already overwritten. That install was always
; per-machine, so only an all-users install can clear it.
!define INNO_UNINST_KEY \
  "Software\Microsoft\Windows\CurrentVersion\Uninstall\{4C264D27-1549-415D-AEA4-0E416409C175}_is1"

;--------------------------------
; VERSIONINFO wants a strict a.b.c.d quad, but APP_VERSION carries a
; pre-release tag ("3.0.0-alpha5"). Split it apart and drop the tag.

!ifndef APP_VERSION_NUM
  ; !searchparse leaves a component undefined when the version is shorter
  ; than the pattern, and defines it empty when the version merely ends
  ; there. VIProductVersion rejects both, so normalise each to "0".
  !macro _VerDefaultZero name
    !ifndef ${name}
      !define ${name} ""
    !endif
    !if "${${name}}" == ""
      !undef ${name}
      !define ${name} "0"
    !endif
  !macroend

  !searchparse /noerrors "${APP_VERSION}." "" _V_MAJOR "." _V_MINOR "." _V_PATCH "." _V_REST
  !insertmacro _VerDefaultZero _V_MAJOR
  !insertmacro _VerDefaultZero _V_MINOR
  !insertmacro _VerDefaultZero _V_PATCH
  ; "0-alpha5" -> "0"
  !searchparse /noerrors "${_V_PATCH}-" "" _V_PATCH_NUM "-" _V_PATCH_TAIL
  !insertmacro _VerDefaultZero _V_PATCH_NUM

  !define APP_VERSION_NUM "${_V_MAJOR}.${_V_MINOR}.${_V_PATCH_NUM}.0"
!endif

VIProductVersion "${APP_VERSION_NUM}"
VIAddVersionKey  "ProductName"     "${APP_NAME}"
VIAddVersionKey  "ProductVersion"  "${APP_VERSION}"
VIAddVersionKey  "FileVersion"     "${APP_VERSION}"
VIAddVersionKey  "FileDescription" "${APP_NAME} Setup"
VIAddVersionKey  "CompanyName"     "${APP_PUBLISHER}"
VIAddVersionKey  "LegalCopyright"  "${APP_PUBLISHER}"

;--------------------------------
; General

Name    "${APP_NAME} ${APP_VERSION}"
OutFile "${OUT_FILE}"

; One 108 MB payload compresses far better as a single solid LZMA block with
; a dictionary large enough to see across it.
SetCompressor /SOLID lzma
SetCompressorDictSize 64

;--------------------------------
; Per-machine / per-user scope.
;
; MultiUser.nsh owns $INSTDIR, the SHCTX registry register, and the shell
; folder context behind $SMPROGRAMS and $DESKTOP, switching all of them to
; match the scope. RequestExecutionLevel is set by MULTIUSER_EXECUTIONLEVEL,
; so it must not be declared separately.

!define MULTIUSER_EXECUTIONLEVEL Highest
!define MULTIUSER_MUI
; Enables the /AllUsers and /CurrentUser switches on the built installer.
!define MULTIUSER_INSTALLMODE_COMMANDLINE
!define MULTIUSER_INSTALLMODE_INSTDIR "${APP_NAME}"
!define MULTIUSER_USE_PROGRAMFILES64
; Reinstalling on top of an existing install keeps that install's directory.
!define MULTIUSER_INSTALLMODE_INSTDIR_REGISTRY_KEY       "${APP_REGKEY}"
!define MULTIUSER_INSTALLMODE_INSTDIR_REGISTRY_VALUENAME "InstallDir"
; Presence of a value under this name in HKCU is how MultiUser.nsh recognises
; a previous per-user install and preselects that scope again; the section
; below writes it as `$MultiUser.InstallMode`.
!define MULTIUSER_INSTALLMODE_DEFAULT_REGISTRY_KEY       "${UNINST_KEY}"
!define MULTIUSER_INSTALLMODE_DEFAULT_REGISTRY_VALUENAME "CurrentUser"
!define MULTIUSER_INSTALLMODE_FUNCTION OnInstallModeChanged

!include "LogicLib.nsh"
!include "MultiUser.nsh"

;--------------------------------
; Interface

!define MUI_ABORTWARNING
!define MUI_UNABORTWARNING
!define MUI_ICON   "${SRC_ICO}"
!define MUI_UNICON "${SRC_ICO}"

; Offer every language rather than only those matching the user's codepage.
; The choice is a personal preference, so it stays in HKCU in both scopes.
!define MUI_LANGDLL_ALLLANGUAGES
!define MUI_LANGDLL_REGISTRY_ROOT      "HKCU"
!define MUI_LANGDLL_REGISTRY_KEY       "${APP_REGKEY}"
!define MUI_LANGDLL_REGISTRY_VALUENAME "Installer Language"

!include "MUI2.nsh"
!include "x64.nsh"
!include "FileFunc.nsh"
!include "Integration.nsh"

Var StartMenuFolder

;--------------------------------
; Pages

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "${SRC_LICENSE}"
!insertmacro MUI_PAGE_COMPONENTS
; Must precede the directory page: choosing the scope is what decides the
; default directory the user then sees.
!insertmacro MULTIUSER_PAGE_INSTALLMODE
!insertmacro MUI_PAGE_DIRECTORY

; The checkbox to skip the folder entirely is what AllowNoIcons=yes gave us
; under Inno; MUI shows it unless MUI_STARTMENUPAGE_NODISABLE is defined.
!define MUI_STARTMENUPAGE_REGISTRY_ROOT      "SHCTX"
!define MUI_STARTMENUPAGE_REGISTRY_KEY       "${APP_REGKEY}"
!define MUI_STARTMENUPAGE_REGISTRY_VALUENAME "Start Menu Folder"
!define MUI_STARTMENUPAGE_DEFAULTFOLDER      "${APP_NAME}"
!insertmacro MUI_PAGE_STARTMENU Application $StartMenuFolder

!insertmacro MUI_PAGE_INSTFILES

!define MUI_FINISHPAGE_RUN
!define MUI_FINISHPAGE_RUN_FUNCTION LaunchApp
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

;--------------------------------
; Languages (the first one is the fallback)

!insertmacro MUI_LANGUAGE "English"
!insertmacro MUI_LANGUAGE "French"
!insertmacro MUI_LANGUAGE "German"

LangString DESC_SecApp     ${LANG_ENGLISH} "The Skribisto application, its icon, and the .skrib file association."
LangString DESC_SecApp     ${LANG_FRENCH}  "L'application Skribisto, son icône et l'association des fichiers .skrib."
LangString DESC_SecApp     ${LANG_GERMAN}  "Die Skribisto-Anwendung, ihr Symbol und die .skrib-Dateizuordnung."

LangString NAME_SecDesktop ${LANG_ENGLISH} "Desktop shortcut"
LangString NAME_SecDesktop ${LANG_FRENCH}  "Raccourci sur le Bureau"
LangString NAME_SecDesktop ${LANG_GERMAN}  "Desktopverknüpfung"

LangString DESC_SecDesktop ${LANG_ENGLISH} "Put a Skribisto shortcut on the desktop."
LangString DESC_SecDesktop ${LANG_FRENCH}  "Placer un raccourci Skribisto sur le Bureau."
LangString DESC_SecDesktop ${LANG_GERMAN}  "Eine Skribisto-Verknüpfung auf dem Desktop anlegen."

LangString MSG_NeedX64     ${LANG_ENGLISH} "Skribisto requires a 64-bit version of Windows."
LangString MSG_NeedX64     ${LANG_FRENCH}  "Skribisto nécessite une version 64 bits de Windows."
LangString MSG_NeedX64     ${LANG_GERMAN}  "Skribisto benötigt eine 64-Bit-Version von Windows."

LangString MSG_StillRunning ${LANG_ENGLISH} "Skribisto is still running. Close it, then click Retry."
LangString MSG_StillRunning ${LANG_FRENCH}  "Skribisto est encore en cours d'exécution. Fermez-le, puis cliquez sur Réessayer."
LangString MSG_StillRunning ${LANG_GERMAN}  "Skribisto läuft noch. Beenden Sie das Programm und klicken Sie auf Wiederholen."

LangString DESC_ProgId     ${LANG_ENGLISH} "Skribisto project file"
LangString DESC_ProgId     ${LANG_FRENCH}  "Fichier de projet Skribisto"
LangString DESC_ProgId     ${LANG_GERMAN}  "Skribisto-Projektdatei"

;--------------------------------
; Reserve files
;
; With solid compression everything lands in one block, so the language
; dialog would otherwise wait on the whole 108 MB payload before appearing.

!insertmacro MUI_RESERVEFILE_LANGDLL

;--------------------------------
; Installer sections

Section "!${APP_NAME}" SecApp

  SectionIn RO

  Call CloseRunningApp
  Call UninstallInnoSetupVersion

  SetOutPath "$INSTDIR"
  SetOverwrite ifnewer
  File "/oname=${APP_EXE}" "${SRC_EXE}"
  File "/oname=${APP_ICO}" "${SRC_ICO}"

  WriteRegStr SHCTX "${APP_REGKEY}" "InstallDir" "$INSTDIR"

  !insertmacro MUI_STARTMENU_WRITE_BEGIN Application
    CreateDirectory "$SMPROGRAMS\$StartMenuFolder"
    CreateShortcut "$SMPROGRAMS\$StartMenuFolder\${APP_NAME}.lnk" \
      "$INSTDIR\${APP_EXE}" "" "$INSTDIR\${APP_ICO}" 0
  !insertmacro MUI_STARTMENU_WRITE_END

  ; .skrib project-file association. HKLM\Software\Classes is the machine half
  ; of the HKCR merged view Inno wrote to; HKCU\Software\Classes is the user
  ; half, and takes precedence for that user. SHCTX picks whichever matches
  ; the scope, so a per-user install claims .skrib without touching HKLM.
  WriteRegStr SHCTX "Software\Classes\${APP_EXT}" "" "${APP_PROGID}"
  WriteRegStr SHCTX "Software\Classes\${APP_PROGID}" "" "$(DESC_ProgId)"
  WriteRegStr SHCTX "Software\Classes\${APP_PROGID}\DefaultIcon" "" "$INSTDIR\${APP_ICO},0"
  WriteRegStr SHCTX "Software\Classes\${APP_PROGID}\shell\open\command" "" \
    '"$INSTDIR\${APP_EXE}" "%1"'
  ${NotifyShell_AssocChanged}

  WriteUninstaller "$INSTDIR\uninstall.exe"

  ; Add/Remove Programs entry.
  WriteRegStr   SHCTX "${UNINST_KEY}" "DisplayName"     "${APP_NAME}"
  WriteRegStr   SHCTX "${UNINST_KEY}" "DisplayVersion"  "${APP_VERSION}"
  WriteRegStr   SHCTX "${UNINST_KEY}" "DisplayIcon"     "$INSTDIR\${APP_ICO}"
  WriteRegStr   SHCTX "${UNINST_KEY}" "Publisher"       "${APP_PUBLISHER}"
  WriteRegStr   SHCTX "${UNINST_KEY}" "URLInfoAbout"    "${APP_URL}"
  WriteRegStr   SHCTX "${UNINST_KEY}" "HelpLink"        "${APP_URL}"
  WriteRegStr   SHCTX "${UNINST_KEY}" "URLUpdateInfo"   "${APP_URL}"
  WriteRegStr   SHCTX "${UNINST_KEY}" "InstallLocation" "$INSTDIR"
  WriteRegStr   SHCTX "${UNINST_KEY}" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegStr   SHCTX "${UNINST_KEY}" "QuietUninstallString" '"$INSTDIR\uninstall.exe" /S'
  WriteRegDWORD SHCTX "${UNINST_KEY}" "NoModify" 1
  WriteRegDWORD SHCTX "${UNINST_KEY}" "NoRepair" 1

  ; Records the scope under its own name ("AllUsers" or "CurrentUser"), which
  ; is what MULTIUSER_INSTALLMODE_DEFAULT_REGISTRY_VALUENAME looks for.
  WriteRegStr   SHCTX "${UNINST_KEY}" $MultiUser.InstallMode 1

  ${GetSize} "$INSTDIR" "/S=0K" $0 $1 $2
  IntFmt $0 "0x%08X" $0
  WriteRegDWORD SHCTX "${UNINST_KEY}" "EstimatedSize" $0

SectionEnd

Section /o "$(NAME_SecDesktop)" SecDesktop

  CreateShortcut "$DESKTOP\${APP_NAME}.lnk" \
    "$INSTDIR\${APP_EXE}" "" "$INSTDIR\${APP_ICO}" 0

SectionEnd

!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SecApp}     $(DESC_SecApp)
  !insertmacro MUI_DESCRIPTION_TEXT ${SecDesktop} $(DESC_SecDesktop)
!insertmacro MUI_FUNCTION_DESCRIPTION_END

;--------------------------------
; Installer functions

Function .onInit

  ; A 32-bit installer on 64-bit Windows otherwise reads and writes the
  ; WOW6432Node reflection of everything below — including
  ; Software\Classes, which would hide the .skrib association from the
  ; 64-bit shell. Must precede MULTIUSER_INIT, which reads the registry.
  SetRegView 64

  !insertmacro MUI_LANGDLL_DISPLAY

  ${IfNot} ${RunningX64}
    MessageBox MB_OK|MB_ICONSTOP "$(MSG_NeedX64)"
    Abort
  ${EndIf}

  !insertmacro MULTIUSER_INIT

FunctionEnd

; Called by MultiUser.nsh once the scope is settled — at startup and again
; whenever the user changes it on the install-mode page — after it has
; assigned $INSTDIR. Runs before the directory page, so overriding $INSTDIR
; here cannot clobber a path the user typed.
Function OnInstallModeChanged

  ${If} $MultiUser.InstallMode == "AllUsers"

    ; Land on top of an Inno-installed copy rather than beside it, unless we
    ; already have an install of our own to honour. Inno stores
    ; InstallLocation with a trailing backslash, which would turn every
    ; "$INSTDIR\file" below into "dir\\file" — harmless to CreateFile, but it
    ; leaks into shortcut targets and the DefaultIcon "path,0" string.
    ReadRegStr $0 SHCTX "${APP_REGKEY}" "InstallDir"
    ${If} $0 == ""
      ReadRegStr $0 HKLM "${INNO_UNINST_KEY}" "InstallLocation"
      ${If} $0 != ""
        StrCpy $1 $0 "" -1
        ${If} $1 == "\"
          StrCpy $0 $0 -1
        ${EndIf}
        StrCpy $INSTDIR $0
      ${EndIf}
    ${EndIf}

  ${EndIf}

  ; Note: picking "Just me" while a machine-wide Skribisto is installed leaves
  ; both in place. A per-user install has no rights to remove the other one,
  ; and Windows permits the pair, so this is allowed rather than blocked.

FunctionEnd

; The image of a running exe is locked against writes, so File would fail
; partway through the install. Ask, and let the user fix it in place.
Function CloseRunningApp

  ${If} ${FileExists} "$INSTDIR\${APP_EXE}"
    retry:
      ClearErrors
      FileOpen $0 "$INSTDIR\${APP_EXE}" a
      ${If} ${Errors}
        ${IfNot} ${Silent}
          MessageBox MB_RETRYCANCEL|MB_ICONEXCLAMATION "$(MSG_StillRunning)" \
            /SD IDCANCEL IDRETRY retry
        ${EndIf}
        Abort
      ${EndIf}
      FileClose $0
  ${EndIf}

FunctionEnd

; Releases through 3.0.0-alpha5 used an Inno Setup installer. Run its
; uninstaller first so its Add/Remove Programs entry and its own registry
; keys go away; ours replace them a few lines later. That install was
; per-machine, so a per-user install has no rights to touch it.
Function UninstallInnoSetupVersion

  ${If} $MultiUser.InstallMode != "AllUsers"
    Return
  ${EndIf}

  ReadRegStr $0 HKLM "${INNO_UNINST_KEY}" "UninstallString"
  ${If} $0 == ""
    Return
  ${EndIf}

  ; Inno quotes the path in the registry; ExecWait needs it unquoted so it
  ; can tell the exe from the switches that follow.
  StrCpy $1 $0 1
  ${If} $1 == '"'
    StrCpy $0 $0 "" 1
    StrCpy $0 $0 -1
  ${EndIf}

  ${IfNot} ${FileExists} "$0"
    ; Stale entry, nothing left to run. Drop it so it stops haunting ARP.
    DeleteRegKey HKLM "${INNO_UNINST_KEY}"
    Return
  ${EndIf}

  DetailPrint "Removing the previous ${APP_NAME} installation..."
  ExecWait '"$0" /VERYSILENT /SUPPRESSMSGBOXES /NORESTART' $2

  ; An Inno uninstaller re-execs itself out of %TEMP% so it can delete its
  ; own directory, and the process we just waited on exits before that copy
  ; has finished. Without this the old uninstaller would still be deleting
  ; files while we write the new ones over them. Poll for it to disappear,
  ; but do not hang the install if it never does.
  StrCpy $3 0
  inno_wait:
    ${IfNot} ${FileExists} "$0"
      Goto inno_done
    ${EndIf}
    ${If} $3 >= 60
      DetailPrint "Timed out waiting for the previous uninstaller; continuing."
      Goto inno_done
    ${EndIf}
    Sleep 500
    IntOp $3 $3 + 1
    Goto inno_wait
  inno_done:

  DeleteRegKey HKLM "${INNO_UNINST_KEY}"

FunctionEnd

; Exec here would hand the app the installer's token, which for an all-users
; install is the elevated one. Going through Explorer drops it back to the
; desktop user's integrity level.
Function LaunchApp

  Exec '"$WINDIR\explorer.exe" "$INSTDIR\${APP_EXE}"'

FunctionEnd

;--------------------------------
; Uninstaller

Section "Uninstall"

  Delete "$INSTDIR\${APP_EXE}"
  Delete "$INSTDIR\${APP_ICO}"
  Delete "$INSTDIR\uninstall.exe"
  RMDir  "$INSTDIR"

  Delete "$DESKTOP\${APP_NAME}.lnk"

  !insertmacro MUI_STARTMENU_GETFOLDER Application $StartMenuFolder
  Delete "$SMPROGRAMS\$StartMenuFolder\${APP_NAME}.lnk"
  RMDir  "$SMPROGRAMS\$StartMenuFolder"

  ; Only give .skrib back if it is still ours — another app may have claimed
  ; it since, and stomping that would be worse than leaving a stray key.
  ReadRegStr $0 SHCTX "Software\Classes\${APP_EXT}" ""
  ${If} $0 == "${APP_PROGID}"
    DeleteRegKey SHCTX "Software\Classes\${APP_EXT}"
  ${EndIf}
  DeleteRegKey SHCTX "Software\Classes\${APP_PROGID}"
  ${NotifyShell_AssocChanged}

  DeleteRegKey SHCTX "${UNINST_KEY}"
  DeleteRegKey SHCTX "${APP_REGKEY}"
  ; The installer language is remembered per-user in both scopes.
  DeleteRegKey HKCU "${APP_REGKEY}"

SectionEnd

Function un.onInit

  SetRegView 64

  !insertmacro MUI_UNGETLANGUAGE
  !insertmacro MULTIUSER_UNINIT

FunctionEnd
