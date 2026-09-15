(defproject loja-diaria "0.1.0-SNAPSHOT"
  :description "Avaliação Clojure - TTEC (Tópicos em Tecnologias Emergentes) - Transformação de dados estruturados"
  :dependencies [[org.clojure/clojure "1.11.1"]
                 [org.clojure/data.json "2.5.0"]]
  :main ^:skip-aot loja-diaria.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all}})
