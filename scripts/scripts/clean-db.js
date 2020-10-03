const { client, clean } = require('../core/db');

async function main() {
  console.log('[ CLEAN DATABASE ]');
  client.connect();
  process.stdout.write('Droping tables...');

  try {
    await clean();
  } catch(err) {
    console.log(err);
  } finally {
    console.log('READY');
    client.end();
  }
}

main();
