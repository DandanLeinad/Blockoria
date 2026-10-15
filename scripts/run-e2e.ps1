<#
.SYNOPSIS
    Blockoria E2E Test Runner - PowerShell script for running Tauri app with Playwright E2E tests.

.DESCRIPTION
    Provides interactive menu to run Tauri app with remote debugging for E2E testing.
    Supports two modes: App Only (manual) or Combined (auto-run full-flow test).

.NOTES
    Requires: cargo tauri build already run, fixtures generated.
#>

param(
    [Parameter()]
    [ValidateSet('AppOnly', 'Combined')]
    [string]$Mode
)

# --- Config ---
$APP_EXE = ".\target\release\blockoria-tauri.exe"
$CDP_PORT = 9222
$FIXTURES_DIR = "$env:TEMP\blockoria-test-fixtures"
$FIXTURES_USERS = "$FIXTURES_DIR\Users"
$APP_ARGS = @("--remote-debugging-port=$CDP_PORT")

# --- Helpers ---
function Write-Header {
    Write-Host "╔═══════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║      Blockoria E2E Test Runner       ║" -ForegroundColor Cyan
    Write-Host "╚═══════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""
}

function Write-Info { param($msg) Write-Host "  $msg" -ForegroundColor Gray }
function Write-Success { param($msg) Write-Host "  ✓ $msg" -ForegroundColor Green }
function Write-Warn { param($msg) Write-Host "  ⚠ $msg" -ForegroundColor Yellow }
function Write-ErrorMsg { param($msg) Write-Host "  ✗ $msg" -ForegroundColor Red }

function Clean-OrphanProcesses {
    $procs = Get-Process -Name "blockoria-tauri" -ErrorAction SilentlyContinue
    if ($procs) {
        Write-Warn "Encontrados $($procs.Count) processo(s) órfão(s) - matando..."
        $procs | ForEach-Object { Stop-Process -Id $_.Id -Force -ErrorAction SilentlyContinue }
        Write-Success "Processos órfãos limpos"
    }
}

function Test-AppExe {
    if (-not (Test-Path $APP_EXE)) {
        Write-ErrorMsg "Executável não encontrado: $APP_EXE"
        Write-Info "Execute primeiro: cargo tauri build"
        return $false
    }
    return $true
}

function Test-Fixtures {
    if (-not (Test-Path $FIXTURES_USERS)) {
        Write-Warn "Fixtures não encontradas em: $FIXTURES_USERS"
        Write-Info "Gere com: cargo run --example gen_test_worlds"
        return $false
    }
    return $true
}

function Test-CdpEndpoint {
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:$CDP_PORT/json/version" -TimeoutSec 2 -ErrorAction Stop
        return $true
    }
    catch {
        return $false
    }
}

function Wait-ForAppReady {
    Write-Info "Aguardando app subir (CDP port $CDP_PORT)..."
    $maxAttempts = 30  # 30s max
    for ($i = 1; $i -le $maxAttempts; $i++) {
        if (Test-CdpEndpoint) {
            Write-Success "App pronto! (CDP respondendo)"
            return $true
        }
        Start-Sleep 1
        Write-Host "." -NoNewline -ForegroundColor Gray
    }
    Write-ErrorMsg "Timeout: CDP não respondeu após ${maxAttempts}s"
    return $false
}

function Set-EnvVars {
    $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = "--remote-debugging-port=$CDP_PORT"
    $env:BLOCKORIA_TEST_WORLDS_DIR = $FIXTURES_DIR
}

function Run-AppOnly {
    Write-Info "Modo: App Only (foreground)"
    Write-Info "Pressione Ctrl+C para parar"
    Set-EnvVars
    & $APP_EXE @APP_ARGS
}

function Run-Combined {
    Write-Info "Modo: Combined (app + full-flow test)"
    Set-EnvVars

    # Start app in background
    Write-Info "Iniciando app em background..."
    $appProcess = Start-Process -FilePath $APP_EXE -ArgumentList $APP_ARGS -PassThru -WindowStyle Normal

    # Trap Ctrl+C to cleanup
    $cleanup = {
        if ($appProcess -and !$appProcess.HasExited) {
            Write-Warn "Parando app (PID $($appProcess.Id))..."
            Stop-Process -Id $appProcess.Id -Force -ErrorAction SilentlyContinue
        }
    }
    trap { & $cleanup; exit 1 }

    try {
        if (-not (Wait-ForAppReady)) {
            throw "App não ficou pronto a tempo"
        }

        Write-Info "Rodando testes E2E (full-flow.spec.ts)..."
        Push-Location app
        $testResult = & bun run test:e2e -- full-flow.spec.ts
        $exitCode = $LASTEXITCODE
        Pop-Location

        if ($exitCode -eq 0) {
            Write-Success "Testes passaram!"
        }
        else {
            Write-ErrorMsg "Testes falharam (exit code: $exitCode)"
        }
        return $exitCode
    }
    finally {
        & $cleanup
    }
}

function Show-Menu {
    Write-Host "  1) Apenas app (manual)" -ForegroundColor White
    Write-Host "     → Sobe app com remote debugging" -ForegroundColor Gray
    Write-Host "     → Você roda testes em outro terminal" -ForegroundColor Gray
    Write-Host ""
    Write-Host "  2) App + Testes (automático)" -ForegroundColor White
    Write-Host "     → Sobe app em background" -ForegroundColor Gray
    Write-Host "     → Roda full-flow.spec.ts" -ForegroundColor Gray
    Write-Host "     → Mata app no final" -ForegroundColor Gray
    Write-Host ""
    Write-Host "  3) Sair" -ForegroundColor White
    Write-Host ""
}

# --- Main ---
Write-Header

Clean-OrphanProcesses

if (-not (Test-AppExe)) { exit 1 }
Test-Fixtures | Out-Null  # warn only

Write-Info "App: $APP_EXE"
Write-Info "Fixtures: $FIXTURES_DIR"
Write-Info "CDP Port: $CDP_PORT"
Write-Host ""

# Determine mode
if ($Mode) {
    $choice = if ($Mode -eq 'AppOnly') { '1' } else { '2' }
}
else {
    Show-Menu
    $choice = Read-Host "Escolha [1-3]"
}

switch ($choice) {
    '1' { Run-AppOnly }
    '2' { exit (Run-Combined) }
    '3' { Write-Host "Saindo..."; exit 0 }
    default { Write-ErrorMsg "Opção inválida"; exit 1 }
}
