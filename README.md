# Cockroach Portal

## Pre-reqs:
- Task
- docker
- rust (cargo)

## Reproducing Portal Issue

### Clone this repo
```
git clone https://github.com/bragaigor/cockroach_portal.git
cd cockroach_portal
```

### Spawn local CockroachDB docker container and start database (required)
```
docker-compose up -d
task initroach
```

### Test your code
```
cd portal
cargo test
```

### To reproduce the error uncomment the transaction lines in `portal/src/transformer_test.rs` and run `cargo test` again. They are
```
// sqlx::query("BEGIN;").execute(&mut dbc).await?;
// sqlx::query("COMMIT;").execute(&mut dbc).await?;
```

### You should expect an error 
```
"unimplemented: multiple active portals not supported"
```

## Shuting down container
```
docker-compose down
```