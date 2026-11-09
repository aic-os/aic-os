# tools/scripts/update-tree.ps1 -- ASCII-safe
$ErrorActionPreference = 'Stop'
$root    = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$outFile = Join-Path $root '.ai/PROJECT_TREE.md'

$excludeDirs = @('.git','node_modules','target','dist','build','.next','.dart_tool','__pycache__','.venv','.idea','.vscode')

$script:lines = New-Object System.Collections.Generic.List[string]
$script:lines.Add('# PROJECT_TREE (auto-generated)')
$script:lines.Add('')
$script:lines.Add('> Generated: ' + (Get-Date -Format 'yyyy-MM-dd HH:mm:ss'))
$script:lines.Add('> Regenerate: powershell -File tools/scripts/update-tree.ps1')
$script:lines.Add('')
$script:lines.Add('```')

function Add-Tree {
    param([string]$Path, [string]$Prefix, [int]$MaxDepth, [int]$Depth = 0)
    if ($Depth -gt $MaxDepth) { return }
    $items = Get-ChildItem -LiteralPath $Path -Force -ErrorAction SilentlyContinue | Where-Object {
        -not ($_.PSIsContainer -and ($excludeDirs -contains $_.Name))
    } | Sort-Object @{Expression={-not $_.PSIsContainer}}, Name
    for ($i = 0; $i -lt $items.Count; $i++) {
        $item = $items[$i]
        $isLast = ($i -eq $items.Count - 1)
        if ($isLast) { $connector = '\-- ' } else { $connector = '|-- ' }
        $script:lines.Add("$Prefix$connector$($item.Name)")
        if ($item.PSIsContainer) {
            if ($isLast) { $newPrefix = $Prefix + '    ' } else { $newPrefix = $Prefix + '|   ' }
            Add-Tree -Path $item.FullName -Prefix $newPrefix -MaxDepth $MaxDepth -Depth ($Depth + 1)
        }
    }
}

$script:lines.Add('aic-os/')
Add-Tree -Path $root -Prefix '' -MaxDepth 5
$script:lines.Add('```')
$script:lines.Add('')

Set-Content -Path $outFile -Value $script:lines -Encoding UTF8
Write-Host ('OK: PROJECT_TREE.md updated -> ' + $outFile)
