; =====================================================================
; ZITERA_LAB Windows Per-User Distribution Installer Script (Inno Setup)
; Contract V4 / Integrated Flutter Desktop + Rust Core Runtime
; Target: Windows 10 1809+ / Windows 11 x64 (Non-Admin Standard User)
; =====================================================================

#define MyAppName "ZITERA_LAB"
#define MyAppVersion "2.0.0"
#define MyAppPublisher "Zitera Security Research"
#define MyAppURL "https://github.com/xdaamar/zitera_lab"
#define MyAppExeName "zitera_lab.exe"

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
UninstallDisplayIcon={app}\branding\app_icon.ico
UninstallDisplayName={#MyAppName} (Per-User)

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
; Primary GUI Application Executable & Flutter Runtime
Source: "..\..\dist\ZITERA_LAB_WINDOWS_X64\zitera_lab.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\dist\ZITERA_LAB_WINDOWS_X64\flutter_windows.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\dist\ZITERA_LAB_WINDOWS_X64\*.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\dist\ZITERA_LAB_WINDOWS_X64\data\*"; DestDir: "{app}\data"; Flags: ignoreversion recursesubdirs createallsubdirs

; Rust Core Engine & Diagnostic Runtime
Source: "..\..\dist\ZITERA_LAB_WINDOWS_X64\engine\*"; DestDir: "{app}\engine"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "..\..\dist\ZITERA_LAB_WINDOWS_X64\engine\zitera-engine.exe"; DestDir: "{app}\bin"; Flags: ignoreversion

; Curriculum Catalog, Packages & Offline Trust
Source: "..\..\dist\ZITERA_LAB_WINDOWS_X64\catalog\*"; DestDir: "{app}\catalog"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "..\..\dist\ZITERA_LAB_WINDOWS_X64\packages\*"; DestDir: "{app}\packages"; Flags: ignoreversion recursesubdirs createallsubdirs

; Branding & Metadata Manifests
Source: "..\..\dist\ZITERA_LAB_WINDOWS_X64\branding\*"; DestDir: "{app}\branding"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "..\..\dist\ZITERA_LAB_WINDOWS_X64\manifest.json"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\dist\ZITERA_LAB_WINDOWS_X64\SHA256SUMS.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\dist\ZITERA_LAB_WINDOWS_X64\README_FIRST_RUN.txt"; DestDir: "{app}"; Flags: ignoreversion isreadme

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\branding\app_icon.ico"
Name: "{group}\{#MyAppName} CLI"; Filename: "{app}\engine\zitera-engine.exe"; Parameters: "doctor"
Name: "{group}\Uninstall {#MyAppName}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\branding\app_icon.ico"; Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[Code]
// User Data Preservation Safeguard
// Ensures %LOCALAPPDATA%\ZiteraLab (Tier 2/3: user progress & lab data) is NEVER wiped on upgrade or uninstallation
procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  UserDataPath: String;
begin
  if CurUninstallStep = usPostUninstall then
  begin
    UserDataPath := ExpandConstant('{localappdata}\ZiteraLab\user');
    // Learner course progress remains safely intact in UserDataPath
  end;
end;
