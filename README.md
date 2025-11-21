📡 Rust HTTP Client

Um cliente HTTP simples e moderno feito em Rust, utilizando Reqwest, Tokio e Iced para fornecer uma interface gráfica minimalista e funcional.

✨ Sobre o projeto

Este projeto é um HTTP Client desenvolvido em Rust com suporte a todos os métodos disponíveis na biblioteca Reqwest, incluindo:

GET

POST

PUT

DELETE

PATCH

HEAD

OPTIONS <- não tem ainda

Além disso, o cliente suporta envio de body (JSON, texto ou qualquer payload fornecido pelo usuário).

A interface gráfica foi construída usando Iced, permitindo interação simples para testar APIs diretamente do desktop.

🚀 Tecnologias utilizadas
Tecnologia	Função
Rust	Linguagem principal do projeto
Reqwest	Biblioteca HTTP assíncrona
Tokio	Runtime assíncrono
Iced	Interface gráfica moderna e reativa
🧩 Funcionalidades principais

✔️ Suporte a todos os métodos HTTP do Reqwest
✔️ Envio de body no request
✔️ Exibição da resposta diretamente na UI
✔️ Erros tratados (ex.: URL inexistente, timeout, erro de parse etc.)
✔️ Operações totalmente assíncronas usando Tokio
✔️ Interface clean feita com Iced
