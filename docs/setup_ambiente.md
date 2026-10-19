# Setup do Ambiente - Guia para Rodar as PoCs

## Pré-requisitos

Certifique-se de ter instalado:

- **Git** (>= 2.x)
- **Node.js** (>= 18.x) ou **Python** (>= 3.10), conforme a PoC
- Um editor de código (recomendado: VS Code)

## Clonando o Repositório

```bash
git clone https://github.com/<seu-usuario>/ifsp-ttec-2026.git
cd ifsp-ttec-2026
```

## Executando uma PoC

Cada PoC possui sua própria estrutura dentro de `pocs/`. Siga os passos:

1. Navegue até o diretório da PoC:
   ```bash
   cd pocs/<nome_da_poc>
   ```

2. Instale as dependências (se aplicável):
   ```bash
   # Node.js
   npm install

   # Python
   python -m venv venv
   source venv/bin/activate   # Linux/macOS
   venv\Scripts\activate      # Windows
   pip install -r requirements.txt
   ```

3. Execute o projeto:
   ```bash
   # Consulte o README.md específico de cada PoC para comandos de execução.
   ```

## Executando Testes

```bash
# Node.js
npm test

# Python
pytest tests/
```

> ⚠️ **Importante:** Toda PoC deve manter cobertura de testes na pasta `tests/`.
