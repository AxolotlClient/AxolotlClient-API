# Contributing
Using [devenv](https://devenv.sh/) is recommended as it provides a semi-standardised environment, including a convenient method for running a local PostgreSQL instance for testing. Please refer to [devenv's documentation](https://devenv.sh/getting-started/) for installation instructions.

Once devenv is properly set up, you should do the following:
- Enter the devenv shell with `devenv shell`
- Start the PostgreSQL process with `devenv up -d`
- Open a PostgreSQL shell to verify the database is running with `psql`
- Exit with Ctrl + D
- Initialize the database with `sqlx migrate run`

You can reset the database by deleting the `.devenv/state/postgres` directory.

You can shut down the database with `devenv processes down`, you should make sure to do this when you no longer need the database, as this is not done automaticaly, so the process will remain open in the background.
