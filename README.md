# toydb2

Toy in-memory RDB

## Usage

### REPL

```
$ cd toydb2
$ cargo run -- repl
```

### Web application

```
$ cd toydb2
$ cargo run -- server
```

```
$ cd web_server
$ cargo run
```

Access to `localhost:2222` from your browser

## Support SQL commands

- `SELECT` - extracts data from a database
- `INSERT INTO` - inserts new data into a database
- `CREATE TABLE` - creates a new table

## Support Types

- `INT`
- `TEXT`

## e.g.

```
$ cargo run
Welcome to toydb2
Use `exit` to exit

>> create table users (id INT, name TEXT);
ok
>> insert into users values (1, 'Alice');
ok
>> insert into users values (2, 'Bob');
ok
>> select id,name from users;
| id | name |
=============
| 1 | Alice |
| 2 | Bob |
```

## LICENSE

MIT

