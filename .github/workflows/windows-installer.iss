; Script generated by the Inno Setup Script Wizard.
; SEE THE DOCUMENTATION FOR DETAILS ON CREATING INNO SETUP SCRIPT FILES!

#define MyAppName "onefetch"
;#define MyAppVersion "1.0"
#define MyAppPublisher "Ossama Hjaji"
#define MyAppURL "https://github.com/o2sh/onefetch"
#define MyAppExeName "onefetch.exe"

[Setup]
; NOTE: The value of AppId uniquely identifies this application. Do not use the same AppId value in installers for other applications.
; (To generate a new GUID, click Tools | Generate GUID inside the IDE.)
AppId={{BB44DE71-B34D-4707-AE9D-5FF3FA632283}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
;AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DisableDirPage=yes
DisableProgramGroupPage=yes
; Uncomment the following line to run in non administrative install mode (install for current user only.)
;PrivilegesRequired=lowest
OutputDir=..\..
OutputBaseFilename=onefetch-setup
SetupIconFile=..\..\assets\onefetch.ico
Compression=lzma
SolidCompression=yes
WizardStyle=modern
ChangesEnvironment=true

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
Source: "..\..\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Icons]
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"

[Code]
{ https://stackoverflow.com/a/46609047/149111 }
const EnvironmentKey = 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment';
procedure EnvAddPath(instlPath: string);
var
  Paths: string;
begin
  { Retrieve current path (use empty string if entry not exists) }
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', Paths) then
    Paths := '';
  if Paths = '' then
    Paths := instlPath + ';'
  else
  begin
    { Skip if string already found in path }
    if Pos(';' + Uppercase(instlPath) + ';',  ';' + Uppercase(Paths) + ';') > 0 then exit;
    if Pos(';' + Uppercase(instlPath) + '\;', ';' + Uppercase(Paths) + ';') > 0 then exit;
    { Append App Install Path to the end of the path variable }
    if Paths[length(Paths)] <> ';' then
      Paths := Paths + ';';
    Paths := Paths + instlPath + ';';
  end;
  { Overwrite (or create if missing) path environment variable }
  if RegWriteStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', Paths)
  then Log(Format('The [%s] added to PATH: [%s]', [instlPath, Paths]))
  else Log(Format('Error while adding the [%s] to PATH: [%s]', [instlPath, Paths]));
end;
procedure EnvRemovePath(instlPath: string);
var
  Paths: string;
  P, Offset, DelimLen: Integer;
begin
  { Skip if registry entry not exists }
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', Paths) then
    exit;
  { Skip if string not found in path }
  DelimLen := 1;     { Length(';') }
  P := Pos(';' + Uppercase(instlPath) + ';', ';' + Uppercase(Paths) + ';');
  if P = 0 then
  begin
    { perhaps instlPath lives in Paths, but terminated by '\;' }
    DelimLen := 2; { Length('\;') }
    P := Pos(';' + Uppercase(instlPath) + '\;', ';' + Uppercase(Paths) + ';');
    if P = 0 then exit;
  end;
  { Decide where to start string subset in Delete() operation. }
  if P = 1 then
    Offset := 0
  else
    Offset := 1;
  { Update path variable }
  Delete(Paths, P - Offset, Length(instlPath) + DelimLen);
  { Overwrite path environment variable }
  if RegWriteStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', Paths)
  then Log(Format('The [%s] removed from PATH: [%s]', [instlPath, Paths]))
  else Log(Format('Error while removing the [%s] from PATH: [%s]', [instlPath, Paths]));
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then
    EnvAddPath(ExpandConstant('{app}'));
end;
procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usPostUninstall then
    EnvRemovePath(ExpandConstant('{app}'));
end;
