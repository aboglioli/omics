const { connectDb } = require('../core/db');
const { rand, randArr } = require('../core/utils');

const Populator = require('../core/populator');

async function main() {
  console.log('[ POPULATE ]');
  const knex = connectDb();
  console.log('Populating DB...');

  const populator = new Populator(knex);

  try {
    for (let i = 0; i < 1200; i++) {
      const user = populator.createUser({ username: `user-${i}` });

      if (rand(0, 100) < 5) {
        for (let i = 0; i < rand(0, 10); i++) {
          let status = null;
          if (rand(0, 100) < 80) {
            status = 'waiting-approval';

            const r = rand(0, 100);
            if (r < 40) {
              status = 'published';
            } else if (r < 60) {
              status = 'rejected';
            }
          }

          populator.createPublication({
            user,
            pageCount: rand(1, 20),
            status,
          });
        }

        for (let i = 0; i < rand(0, 5); i++) {
          populator.createCollection({
            user,
            publications: randArr(
              Object.values(populator.publications)
                .filter(p => p.author_id === user.id),
              true,
            ),
          });
        }
      }
    }

    await populator.save();
  } catch (err) {
    console.log(err);
  } finally {
    console.log('READY');
    await knex.destroy();
  }
}

main();
