const fs = require('fs');
const { connectDb } = require('../core/db');
const config = require('../core/config');

async function main() {
  console.log('[ MIGRATE ]');
  const knex = connectDb();
  console.log('Migrating...');

  try {
    let files = fs.readdirSync(config.migrations_dir);

    await knex.raw(`
      CREATE TABLE IF NOT EXISTS migrations (
        file VARCHAR(255) PRIMARY KEY,
        datetime TIMESTAMP NOT NULL
      );
    `);

    const migrations = await knex.select('*').from('migrations');
    const existingFiles = migrations.map(m => m.file);

    for (const file of files) {
      process.stdout.write(`${file}: `);

      if (existingFiles.some(f => f === file)) {
        console.log('EXISTING');
        continue;
      }

      let content = fs.readFileSync(`${config.migrations_dir}/${file}`);
      content = content.toString();

      await knex.raw(content);
      await knex('migrations')
        .insert({
          file,
          datetime: new Date(),
        });

      console.log('RUN');
    }
  } catch(err) {
    console.log(err);
  } finally {
    console.log('READY');
    await knex.destroy();
  }
}

main();
