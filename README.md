# backend-project
Um acompanhamento do livro "Zero2Prod" sobre backends escalaveis na núvem escritos em RUST.

A ideia é programar um servidor HTTP para um newsletter.

Utilizado todo o CRUD de SQL através dos metodos HTTP, utilizando PostgreSQL.

## --->Nuvem<---
O Deploy é realizado no GoogleCloud, que recebe um gatilho através de um push na branch main, criando então um imagem docker que é rodada em seus sevidores