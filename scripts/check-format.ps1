$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    $manifests = @(& rg --files scm -g Cargo.toml | Where-Object {
        $_ -notmatch '^scm[\\/]Cargo\.toml$'
    })
    if ($LASTEXITCODE -ne 0) {
        throw 'Could not enumerate workspace package manifests.'
    }

    foreach ($manifest in $manifests) {
        & cargo fmt --manifest-path $manifest -- --check
        if ($LASTEXITCODE -ne 0) {
            throw "Formatting check failed for $manifest."
        }
    }

    Write-Output "Formatting passed for $($manifests.Count) workspace packages."
}
finally {
    Pop-Location
}
