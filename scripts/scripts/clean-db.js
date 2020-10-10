const { cleanDb } = require('../core/db');

async function main() {
  console.log('[ CLEAN DATABASE ]');
  process.stdout.write('Droping tables...');

  try {
    await cleanDb();
  } catch(err) {
    console.log(err);
  } finally {
    console.log('READY');
  }
}

main();
