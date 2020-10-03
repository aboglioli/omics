const dotenv = require('dotenv');

dotenv.config({
  path: '../.env',
});

const config = {
  postgres_host: process.env.POSTGRES_HOST,
  postgres_port: process.env.POSTGRES_PORT,
  postgres_username: process.env.POSTGRES_USERNAME,
  postgres_password: process.env.POSTGRES_PASSWORD,
  postgres_database: process.env.POSTGRES_DATABASE,
  migrations_dir: './migrations',
};

module.exports = config;
