# Zero To Production In Rust

## Axum Version

This repo follows the [zero2prod](https://www.zero2prod.com/index.html) book 
except the code is converted to using Axum instead of Actix.

Check out the branches for step by step code through the book.

## Script commands
Initialize DB
```
./scripts/init_db.sh
```

Skip docker container creation
```
SKIP_DOCKER=true ./scripts/init_db.sh
```