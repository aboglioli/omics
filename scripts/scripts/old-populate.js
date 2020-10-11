const { v4: uuidv4 } = require('uuid');
const { connectDb } = require('../core/db');
const {
  password,
  image,
  genders,
  categories,
  tags,
} = require('../core/constants');
const { rand, randArr } = require('../core/utils');

async function main() {
  console.log('[ POPULATE ]');
  const knex = connectDb();
  console.log('Populating DB...');

  try {
    // Create user, publications and collections
    for (let i = 0; i < 10; i++) {
      const userId = uuidv4();

      await knex('users')
        .insert({
          id: userId,
          provider: 'local',
          username: `user-${i}`,
          email: `user-${i}@omics.com`,
          password,
          name: 'Name',
          lastname: 'Lastname',
          birthdate: '1994-08-01 15:30:00Z',
          gender: randArr(genders),
          biography: 'My biography...',
          profile_image: image(200),
          role_id: 'user',
          created_at: new Date(),
        });

      let publicationIds = [];
      for (let j = 0; j < rand(1, 15); j++) {
        const publicationId = uuidv4();

        let pages = [];
        for (let k = 0; k < rand(1,15); k++) {
          pages.push({
            number: k,
            images: [{
              url: image('663x1024'),
            }],
          });
        }

        const statusHistory = [{
          status: 'draft',
          datetime: new Date(),
        }];

        if (rand(0, 100) < 80) {
          statusHistory.push({
            status: 'waiting-approval',
            datetime: new Date(),
          });

          if (rand(0, 100) < 60) {
            statusHistory.push({
              status: 'published',
              admin_id: {
                id: '00000000-0000-0000-0000-000000000002',
              },
              comment: {
                comment: 'Todo está perfecto.',
              },
              datetime: new Date(),
            });
          } else if (rand(0, 100) < 40) {
            statusHistory.push({
              status: 'rejected',
              admin_id: {
                id: '00000000-0000-0000-0000-000000000002'
              },
              comment: {
                comment: 'Tiene contenido que puede resultar ofensivo a los pandas.'
              },
              datetime: new Date(),
            });
          }
        }

        await knex('publications')
          .insert({
            id: publicationId,
            author_id: userId,
            name: `Publication ${i}-${j}`,
            synopsis: 'Synopsis...',
            category_id: randArr(categories),
            tags: JSON.stringify(randArr(tags, true)),
            cover: image(250),
            statistics: {
              views: rand(0, 1000),
              unique_views: rand(0, 850),
              readings: rand(0, 560),
              likes: rand(0, 175),
              reviews: rand(0, 85),
              stars: rand(0, 51) / 10.0,
            },
            status_history: JSON.stringify(statusHistory),
            pages: JSON.stringify(pages),
            created_at: new Date(),
          });

        publicationIds.push(publicationId);
      }

      for (let j = 0; j < rand(1, 6); j++) {
        const collectionId = uuidv4();

        const items = publicationIds
          .filter(() => rand(1, 100) < 20)
          .map(pId => ({
            publication_id: { id: pId },
            date: new Date(),
          }));

        await knex('collections')
          .insert({
            id: collectionId,
            author_id: userId,
            name: `Collection ${i}-${j}`,
            synopsis: 'I am a collection...',
            category_id: randArr(categories),
            tags: JSON.stringify(randArr(tags, true)),
            cover: image(250),
            items: JSON.stringify(items),
            created_at: new Date(),
          });
      }
    }

    // Add interactions
    const users = await knex('users');
    const publications = await knex('publications');

    for (const user of users) {
      for (const publication of publications) {
        if (user.id != publication.author_id) {
          if (rand(0, 100) < 30) {
            await knex('publication_favorites')
              .insert({
                reader_id: user.id,
                publication_id: publication.id,
                datetime: new Date(),
              });
          }
        }
      }
    }
  } catch (err) {
    console.log(err);
  } finally {
    console.log('READY');
    await knex.destroy();
  }
}

main();
