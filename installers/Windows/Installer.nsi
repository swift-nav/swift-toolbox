; This file was adapted from:
; https://github.com/mherrmann/fbs/blob/master/fbs/_defaults/src/installer/windows/Installer.nsi
!include MUI2.nsh
!include FileFunc.nsh
!include LogicLib.nsh
!define MUI_ICON "..\..\resources\images\icon.ico"
!define MUI_UNICON "..\..\resources\images\icon.ico"

!searchparse /file "..\..\console_backend\src\version.txt" `` VER_MAJOR `.` VER_MINOR `.` VER_PATCH_UNFILTERED ``
!searchparse /noerrors "${VER_PATCH_UNFILTERED}" `` VER_PATCH `-`
!define VERSION "${VER_MAJOR}.${VER_MINOR}.${VER_PATCH}.0"
!define app_name "Swift Navigation Console"
!define app_executable "console.exe"
!define installer_dir "py39-dist"
!define company_name "Swift Navigation"

!define UNINST_KEY \
  "Software\Microsoft\Windows\CurrentVersion\Uninstall\${app_name}"

VIProductVersion "${VERSION}"
VIAddVersionKey "ProductName" "${app_name}"
VIAddVersionKey "FileVersion" "${VERSION}"
VIAddVersionKey "ProductVersion" "${VERSION}"
VIAddVersionKey "LegalCopyright" "(C) ${company_name}"
VIAddVersionKey "FileDescription" "${app_name}"

RequestExecutionLevel user

;--------------------------------
;Init

Function .onInit
  ; Do not use InstallDir at all so we can detect empty $InstDir!
  ${If} $InstDir == "" ; /D not used
      StrCpy $InstDir "$LOCALAPPDATA\${company_name}\${app_name}"
      RMDir /r "$InstDir"
      Delete "$DESKTOP\${app_name}.lnk"
      Delete "$SMPROGRAMS\${app_name}.lnk"
      DeleteRegKey /ifempty SHCTX "Software\${app_name}"
      DeleteRegKey SHCTX "${UNINST_KEY}"
  ${EndIf}
FunctionEnd

;--------------------------------
;General

  Name "${app_name}"
  OutFile "${app_name}-${VERSION}.exe"

;--------------------------------
;Interface Settings

  !define MUI_ABORTWARNING

;--------------------------------
;Pages

  !define MUI_WELCOMEPAGE_TEXT "This wizard will guide you through the installation of ${app_name}.$\r$\n$\r$\n$\r$\nClick Next to continue."
  !insertmacro MUI_PAGE_WELCOME
  !insertmacro MUI_PAGE_DIRECTORY
  !insertmacro MUI_PAGE_INSTFILES
    !define MUI_FINISHPAGE_NOAUTOCLOSE
    !define MUI_FINISHPAGE_RUN
    !define MUI_FINISHPAGE_RUN_CHECKED
    !define MUI_FINISHPAGE_RUN_TEXT "Run ${app_name}"
    !define MUI_FINISHPAGE_RUN_FUNCTION "LaunchAsNonAdmin"
  !insertmacro MUI_PAGE_FINISH

  !insertmacro MUI_UNPAGE_CONFIRM
  !insertmacro MUI_UNPAGE_INSTFILES

;--------------------------------
;Languages

  !insertmacro MUI_LANGUAGE "English"

;--------------------------------
;Installer Sections

Section
  SetOutPath "$InstDir"
  File /r "..\..\${installer_dir}\*"
  WriteRegStr SHCTX "Software\${app_name}" "" $InstDir
  WriteUninstaller "$InstDir\uninstall.exe"
  CreateShortCut "$DESKTOP\${app_name}.lnk" "$InstDir\${app_executable}"
  CreateShortCut "$SMPROGRAMS\${app_name}.lnk" "$InstDir\${app_executable}"
  WriteRegStr SHCTX "${UNINST_KEY}" "DisplayName" "${app_name}"
  WriteRegStr SHCTX "${UNINST_KEY}" "Publisher" "${company_name}"
  WriteRegStr SHCTX "${UNINST_KEY}" "DisplayIcon" "$InstDir\uninstall.exe"
  ${GetSize} "$InstDir" "/S=0K" $0 $1 $2
  IntFmt $0 "0x%08X" $0
  WriteRegDWORD SHCTX "${UNINST_KEY}" "EstimatedSize" "$0"

SectionEnd

;--------------------------------
;Uninstaller Section

Section "Uninstall"

  RMDir /r "$InstDir"
  Delete "$DESKTOP\${app_name}.lnk"
  Delete "$SMPROGRAMS\${app_name}.lnk"
  DeleteRegKey /ifempty SHCTX "Software\${app_name}"
  DeleteRegKey SHCTX "${UNINST_KEY}"

SectionEnd

Function LaunchAsNonAdmin
  Exec '"$WINDIR\explorer.exe" "$InstDir\${app_executable}"'
FunctionEnd
