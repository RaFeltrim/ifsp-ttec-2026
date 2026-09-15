(ns loja-diaria.core
  (:require [clojure.data.json :as json]
            [clojure.java.io :as io]))

(defn carregar-dados [caminho]
  (with-open [reader (io/reader caminho)]
    (json/read reader :key-fn keyword)))

(defn total-de-operacoes [dados operacao]
  (count (filter #(= (:operacao %) operacao) dados)))

(defn total-vendas-sucesso [dados]
  (- (total-de-operacoes dados "venda")
     (total-de-operacoes dados "cancelamento")))

(defn valor-liquido [{:keys [cliente valor]}]
  (if (= cliente "especial")
    (* valor 0.9)
    valor))

(defn saldo-dia [dados]
  (reduce (fn [acc {:keys [operacao] :as transacao}]
            (let [liquido (valor-liquido transacao)]
              (cond
                (= operacao "venda")        (+ acc liquido)
                (= operacao "cancelamento") (- acc liquido)
                :else acc)))
          0
          dados))

(defn -main [& args]
  (let [dados (carregar-dados "dados.json")]
    (println "Total de vendas concluídas com sucesso:" (total-vendas-sucesso dados))
    (println "Total de vendas canceladas:" (total-de-operacoes dados "cancelamento"))
    (println "Saldo do dia:" (saldo-dia dados))))
