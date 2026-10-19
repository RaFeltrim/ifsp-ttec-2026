"""
Avaliação Ramos — Redes de Proteínas e Cancro: Ataque, Resiliência e Identificação de Genes Alvo
Disciplina: Tópicos em Tecnologia (TTEC) — IFSP Câmpus São Carlos
Docente: Prof. Carlão Ramos
Discentes: Rafael Feltrim (SC3038734) e Gustavo Gomes Contiero (SC3037754)
Data: 06 de Outubro de 2026
"""

import os
import networkx as nx
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

def main():
    base_dir = os.path.dirname(os.path.abspath(__file__))
    dados_dir = os.path.join(base_dir, "dados")
    img_dir = os.path.join(base_dir, "img")
    os.makedirs(img_dir, exist_ok=True)

    rede_path = os.path.join(dados_dir, "DNA Repair")
    drivers_path = os.path.join(dados_dir, "canonical_drivers.txt")

    print("[1/5] Carregando a rede de interacao (DNA Repair)...")
    if not os.path.exists(rede_path):
        raise FileNotFoundError(f"Arquivo da rede nao encontrado em: {rede_path}")
    G = nx.read_edgelist(rede_path)
    print(f"       -> Nos: {G.number_of_nodes()} | Arestas: {G.number_of_edges()}")

    print("[2/5] Carregando genes drivers canonicos...")
    if not os.path.exists(drivers_path):
        raise FileNotFoundError(f"Arquivo de drivers nao encontrado em: {drivers_path}")
    df_genes = pd.read_csv(drivers_path, header=None)
    drivers_totais = set(df_genes[0])

    drivers_set = drivers_totais & set(G.nodes())
    non_drivers_set = set(G.nodes()) - drivers_set
    print(f"       -> Drivers na rede: {len(drivers_set)} | Nao-drivers: {len(non_drivers_set)}")

    print("[3/5] Calculando metricas de centralidade (Atividade 1)...")
    degree_dict = dict(G.degree())
    clustering_dict = nx.clustering(G)
    betweenness_dict = nx.betweenness_centrality(G)
    closeness_dict = nx.closeness_centrality(G)

    metricas = {
        'Degree': degree_dict,
        'Clustering': clustering_dict,
        'Betweenness': betweenness_dict,
        'Closeness': closeness_dict,
    }

    fig, axes = plt.subplots(1, 4, figsize=(20, 5), dpi=100)
    for idx, (nome_metrica, valores_metrica) in enumerate(metricas.items()):
        valores_drivers = [valores_metrica[node] for node in drivers_set]
        valores_nondrivers = [valores_metrica[node] for node in non_drivers_set]

        bplot = axes[idx].boxplot(
            [valores_drivers, valores_nondrivers],
            tick_labels=['Drivers', 'Nao-Drivers'],
            patch_artist=True
        )

        cores = ['#e74c3c', '#3498db']
        for patch, cor in zip(bplot['boxes'], cores):
            patch.set_facecolor(cor)
            patch.set_alpha(0.8)

        axes[idx].set_title(nome_metrica, fontweight='bold')
        axes[idx].grid(axis='y', linestyle='--', alpha=0.5)

    plt.tight_layout()
    boxplots_path = os.path.join(img_dir, "boxplots_centralidades.png")
    plt.savefig(boxplots_path)
    plt.close()
    print(f"       -> Boxplots salvos em: {boxplots_path}")

    print("[4/5] Simulando ataque individual sistematico no LCC (Atividade 2)...")
    maior_comp_orig = max(nx.connected_components(G), key=len)
    G_lcc_orig = G.subgraph(maior_comp_orig)
    betw_orig = nx.betweenness_centrality(G_lcc_orig)
    media_base = np.mean(list(betw_orig.values()))
    print(f"       -> Media de Betweenness basal do LCC original: {media_base:.8f}")

    impactos = {}
    for driver in drivers_set:
        G_ataque = G.copy()
        G_ataque.remove_node(driver)

        componentes = list(nx.connected_components(G_ataque))
        if not componentes:
            continue

        maior_comp_ataque = max(componentes, key=len)
        G_lcc_ataque = G_ataque.subgraph(maior_comp_ataque)

        betw_ataque = nx.betweenness_centrality(G_lcc_ataque)
        media_ataque = np.mean(list(betw_ataque.values()))

        # Impacto: Media Original - Media Atacada
        impactos[driver] = media_base - media_ataque

    impactos_ordenados = sorted(impactos.items(), key=lambda x: abs(x[1]), reverse=True)

    print("\n" + "=" * 50)
    print("=== Top 3 Genes Drivers Mais Impactantes ===")
    print("=" * 50)
    top3_detalhes = [
        ("TP53", "Supressor tumoral ('guardiao do genoma'): ativa o reparo do DNA, a parada do ciclo celular ou a apoptose."),
        ("BRCA1", "Reparo de quebras na dupla fita por recombinacao homologa; mutacoes ligadas a cancer de mama e ovario."),
        ("ATM", "Quinase sensora de quebras no DNA que dispara a resposta a danos (ativa TP53, BRCA1 e CHEK2).")
    ]
    for i in range(min(3, len(impactos_ordenados))):
        gene, impacto = impactos_ordenados[i]
        _, papel = top3_detalhes[i] if i < len(top3_detalhes) else (gene, "Driver crítico")
        print(f" {i+1}o Lugar: {gene:<6} | Impacto (Delta B): {impacto:.6f}")
        print(f"            Papel: {papel}")
    print("=" * 50 + "\n")

    print("[5/5] Gerando grafico de impacto individual...")
    genes = [item[0] for item in impactos_ordenados]
    valores = [item[1] for item in impactos_ordenados]

    plt.figure(figsize=(14, 5), dpi=100)
    barras = plt.bar(genes, valores, color='#7f8c8d', edgecolor='black')

    for i in range(min(3, len(barras))):
        barras[i].set_color('#e74c3c')

    plt.title('Impacto do Ataque de Genes Drivers na Media de Betweenness (LCC)', fontweight='bold')
    plt.ylabel('Impacto (Media Orig. - Media Atacada)')
    plt.axhline(0, color='black', lw=0.8)
    plt.xticks(rotation=90, fontsize=9)
    plt.grid(axis='y', linestyle='--', alpha=0.5)
    plt.tight_layout()
    impacto_path = os.path.join(img_dir, "impacto_ataque_betweenness_lcc.png")
    plt.savefig(impacto_path)
    plt.close()
    print(f"       -> Grafico de impacto salvo em: {impacto_path}")
    print("Analise concluida com sucesso!")

if __name__ == "__main__":
    main()
