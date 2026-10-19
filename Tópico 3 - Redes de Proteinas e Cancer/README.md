# Tópico 3 — Redes de Proteínas e Cancro: Análise Topológica e Resiliência (Palestra Ramos)

Módulo dedicado à **Ciência de Redes (Network Science)** aplicada à bioinformática e oncologia de precisão, com foco na modelagem de redes de interação proteína-proteína (**PPIN** - *Protein-Protein Interaction Networks*), caracterização de centralidades topológicas e simulação computacional de estabilidade estrutural sob ataques direcionados.

---

## 🗂️ Estrutura do Tópico

```text
Tópico 3 - Redes de Proteinas e Cancer/
├── 📁 Entregas/
│   └── 📁 03_avaliacao_redes_proteinas/              # [ENTREGA T3 - NOTA 10,00]
│       ├── README.md                                 # Relatório analítico completo
│       ├── TTEC_Avaliacao_Ramos_corrigido.pdf       # Relatório Técnico Oficial Corrigido (PDF)
│       ├── TTEC_Avaliacao_Ramos.pdf                 # Relatório Técnico Original (PDF)
│       ├── avaliacao_ramos.ipynb                     # Notebook Jupyter resolvido (Google Colab)
│       ├── avaliacao_ramos.py                       # Script Python standalone para execução local
│       ├── 📁 dados/                                 # Vias e drivers utilizados
│       └── 📁 img/                                   # Gráficos de boxplot e impacto no LCC
│
└── 📁 Material de Apoio/
    ├── Redes de Proteínas e Cancer - Ataque Resiliencia e Alvos.pdf # Slides oficiais da aula/palestra
    ├── Atividade.ipynb                               # Notebook base disponibilizado pelo docente
    ├── input.zip                                     # Pacote compactado de pathways e drivers
    └── 📁 input/                                     # Arquivos brutos de dados
        ├── canonical_drivers.txt                     # 591 genes drivers associados a tumores
        ├── DNA Repair                                # Rede de reparo de DNA (332 nós, 8.353 arestas)
        ├── Chromatin organization                    # Rede de organização de cromatina
        └── Programmed Cell Death                     # Rede de morte celular programada
```

---

## 🧬 Fundamentação Teórica & Metodológica

### 1. Interactoma Tumoral & Hipótese de Centralidade-Letalidade
A progressão tumorigênica resulta do desbalanceamento de vias de sinalização celular e de reparo de DNA. Na Ciência de Redes, as proteínas codificadas por **genes drivers** (genes mutados recorrentemente com vantagem seletiva em câncer) tendem a ocupar posições centrais (*hubs* e *bottlenecks*).

### 2. Atividade 1: Métricas de Centralidade (Drivers vs. Não-Drivers)
Comparação estatística via diagramas de caixa (*Box Plots*):
- **Grau (Degree):** Mediana de drivers (>60) significativamente maior que não-drivers (~40) $\rightarrow$ são **hubs** estruturais.
- **Intermediação (Betweenness):** Concentrada na cauda superior dos drivers $\rightarrow$ funcionam como **pontes e gargalos críticos** de fluxo de informação.
- **Coeficiente de Agrupamento (Clustering):** Ligeiramente menor em drivers (0.62 vs 0.72) $\rightarrow$ ligam módulos funcionais distintos em vez de agregados locais fechados.
- **Proximidade (Closeness):** Maior em drivers (~0.53 vs ~0.49) $\rightarrow$ localizam-se no núcleo topológico da via biológica.

### 3. Atividade 2: Ataque Sistemático e Impacto no LCC
Simulação da remoção dirigida de cada um dos 43 genes drivers identificados na via **DNA Repair** e mensuração da variação da média de intermediação no **Maior Componente Conexo (LCC - Largest Connected Component)**:

$$\Delta B = \overline{B}_{\text{orig}} - \overline{B}_{\text{ataque}}$$

#### 🏆 Top 3 Alvos Terapêuticos Prioritários:
1. **1º Lugar — TP53 ($\Delta B = -0.000030$):** O "guardião do genoma" — orquestra o reparo do DNA, parada de ciclo celular e apoptose.
2. **2º Lugar — BRCA1 ($\Delta B = -0.000025$):** Essencial na recombinação homóloga de fita dupla; fortemente ligado ao câncer de mama e ovário.
3. **3º Lugar — ATM ($\Delta B = -0.000021$):** Quinase sensora de danos na fita de DNA que ativa cascata com TP53 e BRCA1.

---

## 🚀 Como Executar

```bash
# Navegar até a pasta da entrega
cd "Tópico 3 - Redes de Proteinas e Cancer/Entregas/03_avaliacao_redes_proteinas"

# Executar o pipeline analítico
python avaliacao_ramos.py
```
