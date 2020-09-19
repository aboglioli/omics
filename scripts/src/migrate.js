const fs = require('fs');
const client = require('./db');
const config = require('./config');

async function main() {
  console.log('[ MIGRATE ]');
  client.connect();
  console.log('Migrating...');

  try {
    let files = fs.readdirSync(config.migrations_dir);

    await client.query(`
      CREATE TABLE IF NOT EXISTS Migrations (
        file VARCHAR(255) PRIMARY KEY,
        datetime TIMESTAMP NOT NULL
      );
    `);

    const res = await client.query('SELECT * FROM Migrations');
    const existingFiles = res.rows.map(row => row.file);

    for (const file of files) {
      process.stdout.write(`${file}: `);

      let existing = existingFiles.some(f => f === file);

      if (existing) {
        console.log('EXISTING');
        continue;
      }

      let content = fs.readFileSync(`${config.migrations_dir}/${file}`);
      content = content.toString();

      await client.query(content);
      await client.query(
        'INSERT INTO Migrations(file, datetime) VALUES($1, $2)',
        [file, new Date()],
      );

      console.log('RUN');
    }
  } catch(err) {
    console.log(err);
  } finally {
    console.log('READY');
    client.end();
  }
}

main();
