const dotenv = require('dotenv');
const { Client } = require('pg');
const fs = require('fs');

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


async function main() {
  const client = new Client({
    user: config.postgres_username,
    host: config.postgres_host,
    database: config.postgres_database,
    password: config.postgres_password,
    port: config.postgres_port,
  });

  console.log('[ MIGRATE ]');
  console.log('Migrating...');

  try {
    let files = fs.readdirSync(config.migrations_dir);

    client.connect();

    await client.query(`
      CREATE TABLE IF NOT EXISTS Migrations (
        file VARCHAR(255) PRIMARY KEY,
        datetime TIMESTAMP NOT NULL
      );
    `);

    const res = await client.query('SELECT * FROM Migrations');
    const existingFiles = res.rows.map(row => row.file);

    for (const file of files) {

      let existing = existingFiles.some(f => f === file);

      if (existing) {
        console.log(`${file}: EXISTING`);
        continue;
      }

      let content = fs.readFileSync(`${config.migrations_dir}/${file}`);
      content = content.toString();

      await client.query(content);
      await client.query(
        'INSERT INTO Migrations(file, datetime) VALUES($1, $2)',
        [file, new Date()],
      );

      console.log(`${file}: RUN`);
    }
  } catch(err) {
    console.log(err);
  } finally {
  console.log('READY');
    client.end();
  }
}

main();
