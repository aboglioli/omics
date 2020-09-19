const client = require('./db');

async function main() {
  console.log('[ CLEAN DATABASE ]');
  client.connect();
  process.stdout.write('Droping tables...');

  try {
    await client.query(`
      DO $$ DECLARE
          r RECORD;
      BEGIN
          FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = current_schema()) LOOP
              EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
          END LOOP;
      END $$;
    `);
  } catch(err) {
    console.log(err);
  } finally {
    console.log('READY');
    client.end();
  }
}

main();
