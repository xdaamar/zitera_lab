; =====================================================================
; ZITERA_LAB Windows Per-User Distribution Installer Script (Inno Setup)
; Contract V3 / Phase 20 Release Engineering
; Target: Windows 10 1809+ / Windows 11 x64 (Non-Admin Standard User)
; =====================================================================

#define MyAppName "ZITERA_LAB"
#define MyAppVersion "2.0.0"
#define MyAppPublisher "Zitera Security Research"
#define MyAppURL "https://github.com/xdaamar/zitera_lab"
#define MyAppExeName "bin\zitera-engine.exe"

[Setup]
; Unique application GUID for per-user installation tracking
AppId={{5E98B4A2-22AC-4FE5-8D88-E2C3D325E81A}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}

; Strictly per-user installation: ZERO administrative elevation required
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=commandline
DefaultDirName={localappdata}\Programs\ZiteraLab
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes

; Output and compression settings
OutputDir=..\..\dist\installer
OutputBaseFilename=ZiteraLab-Setup-{#MyAppVersion}-x64
Compression=lzma2/ultra64
SolidCompression=yes
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

; UI Polish and silent deployment capabilities
WizardStyle=modern
DisableDirPage=no
AllowNoIcons=yes
CloseApplications=yes
RestartApplications=no

; Uninstaller configuration
UninstallDisplayIcon={app}\{#MyAppExeName}
UninstallDisplayName={#MyAppName} (Per-User)

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
; Core immutable application binaries (Tier 1)
Source: "..\..\dist\bin\zitera-engine.exe"; DestDir: "{app}\bin"; Flags: ignoreversion
Source: "..\..\catalog\*"; DestDir: "{app}\catalog"; Flags: ignoreversion recursesubdirs createallsubdirs; Tasks: ; Languages: 
Source: "..\..\dist\release_manifest.json"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\dist\README.md"; DestDir: "{app}"; Flags: ignoreversion isreadme

[Icons]
Name: "{group}\{#MyAppName} CLI"; Filename: "{app}\{#MyAppExeName}"; Parameters: "doctor"
Name: "{group}\Uninstall {#MyAppName}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Run]
; Optional post-install self-check
Filename: "{app}\{#MyAppExeName}"; Parameters: "doctor"; Description: "Run environment diagnostic"; Flags: nowait postinstall skipifsilent

[Code]
// User Data Preservation Safeguard
// Ensures %LOCALAPPDATA%\ZiteraLab (Tier 2/3: user progress & lab data) is NEVER wiped on upgrade
procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  UserDataPath: String;
begin
  if CurUninstallStep = usPostUninstall then
  begin
    UserDataPath := ExpandConstant('{localappdata}\ZiteraLab\user');
    // Log notice that student course progress remains intact in UserDataPath
  end;
end;
