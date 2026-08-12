# Test suite for CRUD Livros REST API
# Runs against http://localhost:3000

$API = "http://localhost:3000/api/livros"
$pass = 0
$fail = 0
$total = 0

function Test-Assert {
    param(
        [string]$Nome,
        [bool]$Condicao
    )
    $script:total++
    if ($Condicao) {
        $script:pass++
        Write-Host "  PASS: $Nome" -ForegroundColor Green
    } else {
        $script:fail++
        Write-Host "  FAIL: $Nome" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  TESTES AUTOMATIZADOS - CRUD Livros" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# --- TESTE 1: Lista vazia no inicio ---
Write-Host "Teste 1: Lista inicial deve estar vazia" -ForegroundColor Yellow
try {
    $res = Invoke-RestMethod -Uri $API -Method GET
    Test-Assert "Lista vazia retorna array" ($res -is [Array] -or $null -eq $res)
    Test-Assert "Lista tem 0 elementos" ($res.Count -eq 0)
} catch {
    Test-Assert "Conexao com servidor" $false
    Write-Host "  Certifique-se de que o servidor esta rodando!" -ForegroundColor Red
    exit 1
}

# --- TESTE 2: Inserir primeiro livro ---
Write-Host ""
Write-Host "Teste 2: Inserir primeiro livro" -ForegroundColor Yellow
$livro1 = @{
    titulo = "O Hobbit"
    autor = "J.R.R. Tolkien"
    editora = "HarperCollins"
    paginas = 336
} | ConvertTo-Json -Depth 3

try {
    $res = Invoke-RestMethod -Uri $API -Method POST -Body $livro1 -ContentType "application/json"
    Test-Assert "Retorna titulo correto" ($res.titulo -eq "O Hobbit")
    Test-Assert "Retorna autor correto" ($res.autor -eq "J.R.R. Tolkien")
    Test-Assert "Retorna editora correta" ($res.editora -eq "HarperCollins")
    Test-Assert "Retorna paginas corretas" ($res.paginas -eq 336)
} catch {
    Test-Assert "POST inserir livro 1" $false
}

# --- TESTE 3: Inserir segundo livro ---
Write-Host ""
Write-Host "Teste 3: Inserir segundo livro" -ForegroundColor Yellow
$livro2 = @{
    titulo = "1984"
    autor = "George Orwell"
    editora = "Companhia das Letras"
    paginas = 416
} | ConvertTo-Json -Depth 3

try {
    $res = Invoke-RestMethod -Uri $API -Method POST -Body $livro2 -ContentType "application/json"
    Test-Assert "Segundo livro inserido" ($res.titulo -eq "1984")
} catch {
    Test-Assert "POST inserir livro 2" $false
}

# --- TESTE 4: Inserir terceiro livro ---
Write-Host ""
Write-Host "Teste 4: Inserir terceiro livro" -ForegroundColor Yellow
$livro3 = @{
    titulo = "Clean Code"
    autor = "Robert C. Martin"
    editora = "Alta Books"
    paginas = 456
} | ConvertTo-Json -Depth 3

try {
    $res = Invoke-RestMethod -Uri $API -Method POST -Body $livro3 -ContentType "application/json"
    Test-Assert "Terceiro livro inserido" ($res.titulo -eq "Clean Code")
} catch {
    Test-Assert "POST inserir livro 3" $false
}

# --- TESTE 5: Rejeitar titulo duplicado ---
Write-Host ""
Write-Host "Teste 5: Rejeitar titulo duplicado" -ForegroundColor Yellow
try {
    $null = Invoke-RestMethod -Uri $API -Method POST -Body $livro1 -ContentType "application/json"
    Test-Assert "Deveria rejeitar duplicata 409" $false
} catch {
    $status = $_.Exception.Response.StatusCode.value__
    Test-Assert "Retorna status 409 Conflict" ($status -eq 409)
}

# --- TESTE 6: Listar todos - 3 livros ---
Write-Host ""
Write-Host "Teste 6: Listar todos os livros" -ForegroundColor Yellow
try {
    $res = Invoke-RestMethod -Uri $API -Method GET
    Test-Assert "Lista tem 3 elementos" ($res.Count -eq 3)
    $titulos = $res | ForEach-Object { $_.titulo }
    Test-Assert "Contem O Hobbit" ($titulos -contains "O Hobbit")
    Test-Assert "Contem 1984" ($titulos -contains "1984")
    Test-Assert "Contem Clean Code" ($titulos -contains "Clean Code")
} catch {
    Test-Assert "GET listar todos" $false
}

# --- TESTE 7: Buscar um livro especifico ---
Write-Host ""
Write-Host "Teste 7: Buscar livro por titulo" -ForegroundColor Yellow
$encodedTitle = [uri]::EscapeDataString("O Hobbit")
try {
    $res = Invoke-RestMethod -Uri "$API/$encodedTitle" -Method GET
    Test-Assert "Encontrou O Hobbit" ($res.titulo -eq "O Hobbit")
    Test-Assert "Autor correto" ($res.autor -eq "J.R.R. Tolkien")
    Test-Assert "Editora correta" ($res.editora -eq "HarperCollins")
    Test-Assert "Paginas corretas" ($res.paginas -eq 336)
} catch {
    Test-Assert "GET buscar livro" $false
}

# --- TESTE 8: Buscar livro inexistente ---
Write-Host ""
Write-Host "Teste 8: Buscar livro inexistente" -ForegroundColor Yellow
try {
    $null = Invoke-RestMethod -Uri "$API/LivroQueNaoExiste" -Method GET
    Test-Assert "Deveria retornar 404" $false
} catch {
    $status = $_.Exception.Response.StatusCode.value__
    Test-Assert "Retorna status 404 Not Found" ($status -eq 404)
}

# --- TESTE 9: Alterar um livro ---
Write-Host ""
Write-Host "Teste 9: Alterar livro existente" -ForegroundColor Yellow
$livroAtualizado = @{
    titulo = "O Hobbit Edicao Especial"
    autor = "J.R.R. Tolkien"
    editora = "HarperCollins Brasil"
    paginas = 400
} | ConvertTo-Json -Depth 3

try {
    $res = Invoke-RestMethod -Uri "$API/$encodedTitle" -Method PUT -Body $livroAtualizado -ContentType "application/json"
    Test-Assert "Titulo atualizado" ($res.titulo -eq "O Hobbit Edicao Especial")
    Test-Assert "Editora atualizada" ($res.editora -eq "HarperCollins Brasil")
    Test-Assert "Paginas atualizadas" ($res.paginas -eq 400)
} catch {
    Test-Assert "PUT alterar livro" $false
}

# --- TESTE 10: Verificar que alteracao persistiu ---
Write-Host ""
Write-Host "Teste 10: Verificar alteracao persistiu" -ForegroundColor Yellow
$encodedNewTitle = [uri]::EscapeDataString("O Hobbit Edicao Especial")
try {
    $res = Invoke-RestMethod -Uri "$API/$encodedNewTitle" -Method GET
    Test-Assert "Livro encontrado com novo titulo" ($res.titulo -eq "O Hobbit Edicao Especial")
    Test-Assert "Paginas sao 400" ($res.paginas -eq 400)
} catch {
    Test-Assert "GET apos PUT persistencia" $false
}

# --- TESTE 11: Alterar livro inexistente ---
Write-Host ""
Write-Host "Teste 11: Alterar livro inexistente" -ForegroundColor Yellow
try {
    $null = Invoke-RestMethod -Uri "$API/NaoExiste" -Method PUT -Body $livroAtualizado -ContentType "application/json"
    Test-Assert "Deveria retornar 404" $false
} catch {
    $status = $_.Exception.Response.StatusCode.value__
    Test-Assert "Retorna status 404" ($status -eq 404)
}

# --- TESTE 12: Apagar um livro ---
Write-Host ""
Write-Host "Teste 12: Apagar livro 1984" -ForegroundColor Yellow
try {
    $null = Invoke-WebRequest -Uri "$API/1984" -Method DELETE -UseBasicParsing
    Test-Assert "DELETE retornou sucesso" $true
} catch {
    Test-Assert "DELETE livro 1984" $false
}

# --- TESTE 13: Confirmar que foi apagado ---
Write-Host ""
Write-Host "Teste 13: Confirmar exclusao" -ForegroundColor Yellow
try {
    $res = Invoke-RestMethod -Uri $API -Method GET
    Test-Assert "Lista tem 2 elementos agora" ($res.Count -eq 2)
    $titulos = $res | ForEach-Object { $_.titulo }
    $naoContem1984 = -not ($titulos -contains "1984")
    Test-Assert "1984 nao esta mais na lista" $naoContem1984
} catch {
    Test-Assert "GET apos DELETE" $false
}

# --- TESTE 14: Apagar livro inexistente ---
Write-Host ""
Write-Host "Teste 14: Apagar livro inexistente" -ForegroundColor Yellow
try {
    $null = Invoke-RestMethod -Uri "$API/NaoExiste" -Method DELETE
    Test-Assert "Deveria retornar 404" $false
} catch {
    $status = $_.Exception.Response.StatusCode.value__
    Test-Assert "Retorna status 404" ($status -eq 404)
}

# --- TESTE 15: Apagar todos os restantes ---
Write-Host ""
Write-Host "Teste 15: Limpar biblioteca" -ForegroundColor Yellow
try {
    $null = Invoke-WebRequest -Uri "$API/$encodedNewTitle" -Method DELETE -UseBasicParsing
    Test-Assert "Apagou O Hobbit Edicao Especial" $true
} catch {
    Test-Assert "DELETE Hobbit atualizado" $false
}
$encodedCC = [uri]::EscapeDataString("Clean Code")
try {
    $null = Invoke-WebRequest -Uri "$API/$encodedCC" -Method DELETE -UseBasicParsing
    Test-Assert "Apagou Clean Code" $true
} catch {
    Test-Assert "DELETE Clean Code" $false
}

# --- TESTE 16: Lista deve estar vazia novamente ---
Write-Host ""
Write-Host "Teste 16: Lista final deve estar vazia" -ForegroundColor Yellow
try {
    $res = Invoke-RestMethod -Uri $API -Method GET
    $count = 0
    if ($null -ne $res) { $count = $res.Count }
    Test-Assert "Lista voltou a ter 0 elementos" ($count -eq 0)
} catch {
    Test-Assert "GET final lista vazia" $false
}

# --- TESTE 17: Frontend servido corretamente ---
Write-Host ""
Write-Host "Teste 17: Frontend estatico servido" -ForegroundColor Yellow
try {
    $res = Invoke-WebRequest -Uri "http://localhost:3000/" -Method GET
    Test-Assert "Status 200 OK" ($res.StatusCode -eq 200)
    $contemCrud = $res.Content -match "CRUD de Livros"
    Test-Assert "Contem CRUD de Livros" $contemCrud
    $contemRust = $res.Content -match "Rust"
    Test-Assert "Contem Rust" $contemRust
} catch {
    Test-Assert "GET frontend" $false
}

# === RESULTADO FINAL ===
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
if ($fail -eq 0) {
    Write-Host "  TODOS OS $total TESTES PASSARAM!" -ForegroundColor Green
} else {
    Write-Host "  $pass de $total passaram, $fail falharam" -ForegroundColor Red
}
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
