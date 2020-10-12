const dotenv = require("dotenv");

dotenv.config({
  path: "../.env",
});

const config = {
  postgresHost: process.env.POSTGRES_HOST,
  postgresPort: process.env.POSTGRES_PORT,
  postgresUsername: process.env.POSTGRES_USERNAME,
  postgresPassword: process.env.POSTGRES_PASSWORD,
  postgresDatabase: process.env.POSTGRES_DATABASE,
  marvelPublicKey: process.env.MARVEL_PUBLIC_KEY,
  marvelPrivateKey: process.env.MARVEL_PRIVATE_KEY,
  migrationsDir: "./migrations",
};

module.exports = config;
