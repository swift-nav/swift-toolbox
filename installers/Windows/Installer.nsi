; This file was adapted from:
; https://github.com/mherrmann/fbs/blob/master/fbs/_defaults/src/installer/windows/Installer.nsi
Unicode true
!include "x64.nsh"
!include nsDialogs.nsh
!include WinVer.nsh
!include MUI2.nsh
!include FileFunc.nsh
!include LogicLib.nsh
!addplugindir "NSIS\Plugins\x86-unicode"
!define MUI_ICON "..\..\resources\images\icon.ico"
!define MUI_UNICON "..\..\resources\images\icon.ico"

!searchparse /file "..\..\console_backend\src\version.txt" `` VER_MAJOR `.` VER_MINOR `.` VER_PATCH_UNFILTERED ``
!searchparse /noerrors "${VER_PATCH_UNFILTERED}" `` VER_PATCH `-`
!define VERSION_ORIGINAL "${VER_MAJOR}.${VER_MINOR}.${VER_PATCH_UNFILTERED}"
!define VERSION "${VER_MAJOR}.${VER_MINOR}.${VER_PATCH}.0"
!define app_name "Swift Console"
!define app_executable "swift-console.exe"
!define outfile_prefix "swift-console"
!define installer_dir "py39-dist"
!define company_name "Swift Navigation"

!define vc_redist_url "https://aka.ms/vs/17/release/vc_redist.x64.exe"

ManifestDPIAware true


!define UNINST_KEY \
  "Software\Microsoft\Windows\CurrentVersion\Uninstall\${app_name}"

VIProductVersion "${VERSION}"
VIAddVersionKey "ProductName" "${app_name}"
VIAddVersionKey "FileVersion" "${VERSION}"
VIAddVersionKey "ProductVersion" "${VERSION}"
VIAddVersionKey "LegalCopyright" "(C) ${company_name}"
VIAddVersionKey "FileDescription" "${app_name}"

!define MULTIUSER_EXECUTIONLEVEL Highest ; Switch to "Highest" for All Users when available otherwise current user.

!include MultiUser.nsh

;--------------------------------
;Init

Function .onInit
  ${IfNot} ${AtLeastWin10}
    MessageBox mb_iconStop "This application is only supported for Windows 10 or greater!" 
    Abort
  ${EndIf}
  ${IfNot} ${RunningX64}
    MessageBox MB_OK "This program must be run on an x64 machine."
    Abort
  ${EndIf}
  SetRegView 64
  ${DisableX64FSRedirection}
  ; Do not use InstallDir at all so we can detect empty $InstDir!
  !insertmacro MULTIUSER_INIT
  ${If} $InstDir == "" ; /D not used
    StrCpy $InstDir "$PROGRAMFILES64\${company_name}\${app_name}"
  ${EndIf}
  
FunctionEnd

;--------------------------------
;General

  Name "${app_name}"
  OutFile "${outfile_prefix}_${VERSION_ORIGINAL}_windows.exe"

;--------------------------------
;Interface Settings

  !define MUI_ABORTWARNING

;-------------------------------
;Installer Ask Uninstall Page

  !define UninstallMsg "Warning! By clicking $\"Next$\", this installer will uninstall any previous versions of the Swift Console.$\n$\n\
If this is not desired, please exit the installer now."

  Function uninstallOldVersionsPage
  !insertmacro MUI_HEADER_TEXT "Uninstall Previous Version" "This installer will uninstall any previous versions."

  nsDialogs::Create 1018
  Pop $0
  ${If} $0 == error
      Abort
  ${EndIf}

  ${NSD_CreateLabel} 0 0 100% 40u "${UninstallMsg}"
  Pop $0

  nsDialogs::Show
  FunctionEnd

  Function uninstallOldVersionsPageLeave
  Call Uninstall
  FunctionEnd

;-------------------------------
;Installer Check for VC Redistributable

  !define mvcFoundMsg "Warning! Microsoft Visual C++ 14 Redistributable is not installed.$\n$\n\
By clicking $\"Next$\", this installer will attempt to install the required package during installation.$\n$\n\
If this is not desired, please exit the installer now."

  Function mvcRedistributablePage
  

  ReadRegStr $0 HKLM "SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64" "Version"
  ReadRegStr $1 HKLM "SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64" "Installed"
  ${If} $1 != "1"
    !insertmacro MUI_HEADER_TEXT "Microsoft Visual C++ Redistributable" "Check if required Microsoft Visual C++ Redistributable is installed."

    nsDialogs::Create 1018
    Pop $0
    ${If} $0 == error
        Abort
    ${EndIf}

    ${NSD_CreateLabel} 0 0 100% 40u "${mvcFoundMsg}"
    Pop $0

    nsDialogs::Show
  ${EndIf}
  
  FunctionEnd


;--------------------------------
;Pages
  !define MUI_WELCOMEPAGE_TEXT "This wizard will guide you through the installation of ${app_name} version ${VERSION_ORIGINAL}.$\r$\n$\r$\n$\r$\nClick Next to continue."
  !insertmacro MUI_PAGE_WELCOME
  Page custom uninstallOldVersionsPage uninstallOldVersionsPageLeave
  !insertmacro MUI_PAGE_DIRECTORY
  Page custom mvcRedistributablePage
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
  
  ReadRegStr $0 HKLM "SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64" "Version"
  ReadRegStr $1 HKLM "SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64" "Installed"
  ${If} $1 != "1"
    DetailPrint "Microsoft Visual C++ redistributable is not installed."
    inetc::get "${vc_redist_url}" $InstDir\vcredist.exe /END
    Pop $3
    DetailPrint "Download completed (return code: $3)."
    DetailPrint "Installing..."
    ExecWait '"$InstDir\vcredist.exe" /q /norestart'
    ReadRegStr $1 HKLM "SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64" "Installed"
    ${If} $1 != "1"
      DetailPrint "Failed to install redistributable for Microsoft Visual C++!"
      MessageBox MB_YESNO "Installation of redistributable for Microsoft Visual C++ was unsuccessful! Please verify that an Internet connection is available.$\n$\nAlternately, the redistributable can be manually installed by downloading:$\n${vc_redist_url}$\n$\nAttempt download now?" IDYES true IDNO false
      true:
        ExecShell open "${vc_redist_url}"
      false:
      Quit
    ${EndIf}
    DetailPrint "Done"
  ${Else}
    DetailPrint "VC++ Redistributable found, version $0 !"
  ${EndIf}
  
  File /r "..\..\${installer_dir}\*"
  WriteRegStr SHCTX "Software\${app_name}" "" $InstDir
  WriteUninstaller "$InstDir\uninstall.exe"
  CreateShortCut "$DESKTOP\${app_name}.lnk" "$InstDir\${app_executable}"
  CreateShortCut "$SMPROGRAMS\${app_name}.lnk" "$InstDir\${app_executable}"
  WriteRegStr SHCTX "${UNINST_KEY}" "DisplayName" "${app_name}"
  WriteRegStr SHCTX "${UNINST_KEY}" "UninstallString" \
    "$\"$InstDir\uninstall.exe$\" /$MultiUser.InstallMode"
  WriteRegStr SHCTX "${UNINST_KEY}" "QuietUninstallString" \
    "$\"$InstDir\uninstall.exe$\" /$MultiUser.InstallMode /S"
  WriteRegStr SHCTX "${UNINST_KEY}" "Publisher" "${company_name}"
  WriteRegStr SHCTX "${UNINST_KEY}" "DisplayIcon" "$InstDir\uninstall.exe"
  ${GetSize} "$InstDir" "/S=0K" $0 $1 $2
  IntFmt $0 "0x%08X" $0
  WriteRegDWORD SHCTX "${UNINST_KEY}" "EstimatedSize" "$0"
SectionEnd

;--------------------------------
;Uninstaller Section

Section "Uninstall"
  Call un.Uninstall
SectionEnd

Function LaunchAsNonAdmin
  Exec '"$WINDIR\explorer.exe" "$InstDir\${app_executable}"'
FunctionEnd

!macro Uninstall Prefix
Function ${Prefix}Uninstall
  RMDir /r "$InstDir"
  Delete "$DESKTOP\${app_name}.lnk"
  Delete "$SMPROGRAMS\${app_name}.lnk"
  DeleteRegKey HKLM64 "${UNINST_KEY}"
  DeleteRegKey HKLM64 "SOFTWARE\${app_name}"
FunctionEnd
!macroend
!insertmacro Uninstall "" 
!insertmacro Uninstall "un."

Function un.onInit
  !insertmacro MULTIUSER_UNINIT
FunctionEnd
