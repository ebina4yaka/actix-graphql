# actix-graphql
## development
run
```bash
docker-compose up -d
```
```bash
diesel migration run
```
add .env example
```.env
DATABASE_URL=postgres://postgres:postgres@localhost/graphql_sample
RUST_LOG=debug,actix_web=debug
```
