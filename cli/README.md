# forge-api-cli

This crate contains a CLI for managing the database and creating project files like controllers, entities, middleware, or tests.

_You should not need to make any changes to this crate._

## Managing the database

Creating the database (make sure the user configured in `.env` has sufficient permissions):

```
cargo db create
```

Dropping the database (make sure the user configured in `.env` has sufficient permissions):

```
cargo db create
```

Running all pending migrations:

```
cargo db migrate
```

Resetting the database – this will drop the database and re-create the database, then run all migrations:

```
cargo db reset
```

Seeding the database – this will execute any statements in `db/seeds.sql`

```
cargo db seed
```

Seeds can be used for essentially static data like currencies or countries.

### Environments

By default, the database tasks run with the development environment. That means the `.env` is used to set the `APP_DATABASE__URL` environment variable. To run the tasks against the test database in which case the `.env.test` file is read instead of the `.env.file`, run e.g.:

```
cargo db reset -e test
```

When running against the production database, neither the `.env` or `.env.test` files are read and the `APP_DATABASE__URL` environment variable is expected to point to the production database:

```
cargo db reset -e production
```

## Generating project files

Project files are generated with the

```
cargo generate
```

command. The CLI comes with commands for generating middlewares, controllers, controller tests, CRUD controllers and tests for those, migrations, and entities. To get help for each of the controllers, use the `-h` flag, e.g.:

```
cargo generate controller -h
```
