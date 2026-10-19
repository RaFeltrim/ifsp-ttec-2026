# Tópico 2 — Programação Funcional com Clojure

Módulo focado no paradigma funcional moderno utilizando **Clojure**, uma linguagem dinâmica da família Lisp executada na Máquina Virtual Java (JVM), com ênfase em dados imutáveis, funções de ordem superior e concorrência simplificada.

---

## 🗂️ Estrutura do Tópico

```text
Tópico 2 - Programacao Funcional Clojure/
├── 📁 Entregas/
│   └── 📁 02_avaliacao_clojure/       # [ENTREGA T2 - NOTA 10,00] Loja Diária
│       ├── project.clj                # Gerenciador de dependências e build Leiningen
│       ├── README.md                  # Especificação e relatório de execução
│       ├── dados.json                 # Arquivo de transações da loja (entrada)
│       └── 📁 src/
│           └── 📁 loja_diaria/
│               └── core.clj           # Processamento funcional das vendas
│
└── 📁 Material de Apoio/
    └── README.md                      # Referências de sintaxe Lisp, REPL e guias
```

---

## 🧪 Prática Avaliativa: Loja Diária (`02_avaliacao_clojure`)

### O que foi solicitado
1. **Leitura e Parser de JSON:** Processar um fluxo diário de transações de vendas a partir do arquivo [`dados.json`](Entregas/02_avaliacao_clojure/dados.json).
2. **Cálculos Estratégicos:**
   - Quantidade total de transações concluídas com sucesso.
   - Quantidade total de transações canceladas.
   - Saldo financeiro final apurado do dia.
3. **Regra de Negócio Específica:**
   - Clientes classificados como especiais recebem **10% de desconto** automaticamente sobre o valor das suas compras aprovadas.
4. **Princípios de Implementação:**
   - Uso de funções puras (`filter`, `reduce`, `map`), composição e ausência de efeitos colaterais.

### Saída da Execução
```text
Total de vendas concluídas com sucesso: 2
Total de vendas canceladas: 1
Saldo do dia: 125.0
```

---

## 🚀 Como Executar

**Pré-requisitos:** Java JDK 8+ e Leiningen instalados.

```bash
# Navegar até a pasta da entrega
cd "Tópico 2 - Programacao Funcional Clojure/Entregas/02_avaliacao_clojure"

# Executar a aplicação
lein run
```
