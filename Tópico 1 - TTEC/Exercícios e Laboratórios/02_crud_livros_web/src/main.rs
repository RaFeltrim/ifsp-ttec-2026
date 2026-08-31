use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

// ══════════════════════════════════════════════════════════════
// 1. MODELO DE DADOS (mesma struct do CLI, agora com Serialize)
// ══════════════════════════════════════════════════════════════
//
// #[derive(Serialize, Deserialize)] → macros do Serde que permitem
// converter a struct automaticamente para JSON e vice-versa.
// Clone é necessário porque compartilhamos dados entre requests.

#[derive(Serialize, Deserialize, Clone)]
struct Livro {
    titulo: String,
    autor: String,
    editora: String,
    paginas: u32,
}

// ══════════════════════════════════════════════════════════════
// 2. ESTADO COMPARTILHADO (o "banco de dados" em memória)
// ══════════════════════════════════════════════════════════════
//
// Arc  → Permite que múltiplas threads acessem o mesmo dado (Atomic Reference Count)
// Mutex → Garante que apenas UMA thread por vez modifica o Vec (Mutual Exclusion)
//
// Isso é o equivalente web do nosso `let mut biblioteca: Vec<Livro>` do CLI,
// mas seguro para acesso concorrente (múltiplos usuários ao mesmo tempo).

type Biblioteca = Arc<Mutex<Vec<Livro>>>;

// ══════════════════════════════════════════════════════════════
// 3. FUNÇÃO PRINCIPAL — Configura o servidor HTTP
// ══════════════════════════════════════════════════════════════
//
// #[tokio::main] → Transforma main() em uma função assíncrona.
// Tokio é o runtime async usado pela Discord em produção.

#[tokio::main]
async fn main() {
    // Cria o "banco de dados" em memória (Vec vazio protegido por Arc+Mutex)
    let biblioteca: Biblioteca = Arc::new(Mutex::new(Vec::new()));

    // Define as rotas da API REST
    // Cada rota mapeia para um verbo HTTP + uma função handler
    let api_routes = Router::new()
        .route("/livros", get(listar_todos))       // GET    /api/livros
        .route("/livros", post(inserir))            // POST   /api/livros
        .route("/livros/{titulo}", get(buscar))     // GET    /api/livros/{titulo}
        .route("/livros/{titulo}", put(alterar))    // PUT    /api/livros/{titulo}
        .route("/livros/{titulo}", delete(apagar)); // DELETE /api/livros/{titulo}

    // Monta a aplicação completa
    let app = Router::new()
        .nest("/api", api_routes)                         // API sob /api/*
        .fallback_service(ServeDir::new("frontend"))      // Frontend estático
        .layer(CorsLayer::permissive())                   // Permite requisições cross-origin
        .with_state(biblioteca);                          // Injeta o "banco" em todos os handlers

    // Inicia o servidor na porta 3000
    let endereco = "0.0.0.0:3000";
    println!("══════════════════════════════════════════");
    println!("  🦀 CRUD de Livros — API REST com Axum");
    println!("══════════════════════════════════════════");
    println!("  🌐 Frontend: http://localhost:3000");
    println!("  📡 API:      http://localhost:3000/api/livros");
    println!("══════════════════════════════════════════");

    let listener = tokio::net::TcpListener::bind(endereco).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ══════════════════════════════════════════════════════════════
// 4. HANDLERS — As funções que respondem cada rota
// ══════════════════════════════════════════════════════════════

/// POST /api/livros — Inserir um novo livro
/// Recebe JSON no body, retorna 201 Created ou 409 Conflict
async fn inserir(
    State(db): State<Biblioteca>,
    Json(novo_livro): Json<Livro>,
) -> Result<(StatusCode, Json<Livro>), (StatusCode, String)> {
    let mut biblioteca = db.lock().unwrap();

    // Verifica duplicata pelo título
    let ja_existe = biblioteca.iter().any(|l| l.titulo == novo_livro.titulo);

    if ja_existe {
        Err((
            StatusCode::CONFLICT,
            format!("Já existe um livro com o título '{}'", novo_livro.titulo),
        ))
    } else {
        let livro_clone = novo_livro.clone();
        biblioteca.push(novo_livro);
        Ok((StatusCode::CREATED, Json(livro_clone)))
    }
}

/// GET /api/livros — Listar todos os livros
/// Retorna um array JSON com todos os livros
async fn listar_todos(State(db): State<Biblioteca>) -> Json<Vec<Livro>> {
    let biblioteca = db.lock().unwrap();
    Json(biblioteca.clone())
}

/// GET /api/livros/{titulo} — Buscar um livro pelo título
/// Retorna o livro ou 404 Not Found
async fn buscar(
    State(db): State<Biblioteca>,
    Path(titulo): Path<String>,
) -> Result<Json<Livro>, (StatusCode, String)> {
    let biblioteca = db.lock().unwrap();

    biblioteca
        .iter()
        .find(|l| l.titulo == titulo)
        .cloned()
        .map(Json)
        .ok_or((
            StatusCode::NOT_FOUND,
            format!("Livro '{}' não encontrado", titulo),
        ))
}

/// PUT /api/livros/{titulo} — Alterar um livro existente
/// Recebe JSON com novos dados, retorna o livro atualizado ou 404
async fn alterar(
    State(db): State<Biblioteca>,
    Path(titulo): Path<String>,
    Json(novos_dados): Json<Livro>,
) -> Result<Json<Livro>, (StatusCode, String)> {
    let mut biblioteca = db.lock().unwrap();

    let encontrado = biblioteca.iter_mut().find(|l| l.titulo == titulo);

    match encontrado {
        Some(livro) => {
            livro.titulo = novos_dados.titulo;
            livro.autor = novos_dados.autor;
            livro.editora = novos_dados.editora;
            livro.paginas = novos_dados.paginas;
            Ok(Json(livro.clone()))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            format!("Livro '{}' não encontrado", titulo),
        )),
    }
}

/// DELETE /api/livros/{titulo} — Apagar um livro
/// Retorna 204 No Content ou 404 Not Found
async fn apagar(
    State(db): State<Biblioteca>,
    Path(titulo): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut biblioteca = db.lock().unwrap();
    let tamanho_antes = biblioteca.len();

    biblioteca.retain(|l| l.titulo != titulo);

    if biblioteca.len() < tamanho_antes {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            format!("Livro '{}' não encontrado", titulo),
        ))
    }
}
