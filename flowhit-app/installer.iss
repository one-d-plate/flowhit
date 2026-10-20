; FLOWHIT PIXEL installer
#define MyAppName "FLOWHIT PIXEL"
#define MyAppVersion "2.4.0"
#define MyAppPublisher "one-d-plate"
#define MyAppURL "https://one-d-plate.github.io/flowhit-pixel/"
#define MyAppExeName "flowhit-app.exe"

[Setup]
AppId={{3F4E1A2B-7C9D-4E5F-8A6B-1D2C3E4F5A6B}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
DefaultDirName={autopf}\FLOWHIT PIXEL
DefaultGroupName=FLOWHIT PIXEL
OutputDir=installer-out
OutputBaseFilename=FLOWHIT-PIXEL-Setup-2.4.0
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=lowest
ArchitecturesInstallIn64BitMode=x64

[Tasks]
Name: "desktopicon"; Description: "Buat ikon di Desktop"; Flags: unchecked
Name: "quicklaunchicon"; Description: "Buat ikon Quick Launch"; Flags: unchecked; OnlyBelowVersion: 0,6.1

[Files]
Source: "dist\flowhit-app.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "dist\flowhit-server.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "dist\web\*"; DestDir: "{app}\web"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "dist\workspace\*"; DestDir: "{app}\workspace"; Flags: ignoreversion recursesubdirs createallsubdirs onlyifdoesntexist uninsneveruninstall

[Icons]
Name: "{group}\FLOWHIT PIXEL"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\Hapus FLOWHIT PIXEL"; Filename: "{uninstallexe}"
Name: "{autodesktop}\FLOWHIT PIXEL"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "Jalankan FLOWHIT PIXEL"; Flags: nowait postinstall skipifsilent
