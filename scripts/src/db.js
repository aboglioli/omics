const { Client } = require('pg');
const config = require('./config');

const client = new Client({
  user: config.postgres_username,
  host: config.postgres_host,
  database: config.postgres_database,
  password: config.postgres_password,
  port: config.postgres_port,
});

module.exports = client;
