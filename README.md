# Manta-API

## Starting the DB

```sh
# Start postgresql server docker image:
docker run --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=welcome  postgres:15

# (optional) To have a psql terminal on pg. 
# In another terminal (tab) run psql:
docker exec -it -u postgres pg psql

# (optional) For pg to print all sql statements.
# In psql command line started above.
ALTER DATABASE postgres SET log_statement = 'all';
```

## Dev (REPL)

```sh
# Terminal 1 - For server run.
cargo watch -q -c -w src/ -w .cargo/ -x "run"

# Terminal 2 - For quick dev.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

Unit test REPL

```sh
cargo watch -q -c -x "test -- --nocapture"

# Specific test filter.
cargo watch -q -c -x "test model::send::tests::test_create -- --nocapture"
```


## Dev

```sh
# Terminal 1 - For server run.
cargo run

# Terminal 2 - For quick dev.
cargo run --example quick_dev
```

or unit tests

```sh
# threads=1 for now, since recreate DB and could cause issue when concurrent.
cargo test -- --test-threads=1
```
