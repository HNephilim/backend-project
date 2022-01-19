# backend-project
Um acompanhamento do livro "Zero2Prod" sobre backends escalaveis na núvem escritos em RUST.

A ideia é programar um servidor HTTP para um newsletter.

Utilizado todo o CRUD de SQL através dos metodos HTTP e banco de dados PostgreSQL.

## --->Nuvem<---
O Deploy é realizado no GoogleCloud, que recebe um gatilho através de um push na branch main, criando então um imagem 
docker que é rodada em seus sevidores.

O app reconhece através de variáveis de ambiente se está rodando local ou produção, escolhendo assim a instancia SQL que
irá rodar. Os dados da instância local é dado através de arquivos de configuração, mas a instancia de produção
é passada através de "Secrets" criptografados pelo google, que levam a uma instância PostgreSQL do CloudSQL.