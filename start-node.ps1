# start-node.ps1 -- Запуск AIC Node одной командой
# Использование: .\start-node.ps1

$ErrorActionPreference = "Stop"

# Убиваем старую ноду
Get-Process -Name "aic-node" -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Seconds 2

# Сброс RUSTFLAGS (могут остаться от WASM)
$env:RUSTFLAGS = ""

$nodePath = "target\debug\aic-node.exe"
if (-not (Test-Path $nodePath)) {
    Write-Host "ERROR: $nodePath не найден. Собери ноду: cargo build -p aic-node" -ForegroundColor Red
    exit 1
}

$alice = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"

Write-Host "Starting AIC Node..." -ForegroundColor Cyan
Write-Host "  RPC:     http://127.0.0.1:8545" -ForegroundColor Gray
Write-Host "  P2P:     port 30333" -ForegroundColor Gray
Write-Host "  Validator: Alice (dev)" -ForegroundColor Gray

$proc = Start-Process -FilePath $nodePath `
  -ArgumentList "--p2p-port","30333","--rpc-port","8545","--validators",$alice,"--my-address",$alice `
  -RedirectStandardOutput "node.log" -RedirectStandardError "node.err" `
  -PassThru -WindowStyle Hidden

Start-Sleep -Seconds 4

# Проверка
try {
    $body = '{"jsonrpc":"2.0","id":1,"method":"aic_blockNumber","params":[]}'
    $r = Invoke-RestMethod -Uri "http://127.0.0.1:8545/" -Method Post -ContentType "application/json" -Body $body
    Write-Host "OK: node is running (PID $($proc.Id)), block $($r.result)" -ForegroundColor Green
    Write-Host "F5 in browser: http://localhost:8080" -ForegroundColor Cyan
} catch {
    Write-Host "ERROR: node started but RPC not responding: $($_.Exception.Message)" -ForegroundColor Red
    Get-Content "node.err" -Tail 10
}
