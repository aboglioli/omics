const { v4: uuidv4 } = require('uuid');
const client = require('./db');

const rand = (min, max) => Math.floor(Math.random() * (max - min)) + min;
const image = size => `https://via.placeholder.com/${size ? size : '256'}.jpg`;
// const image = size => `http://lorempixel.com/${size ? `${size}/${size}` : '256/256'}/`;
const password = '$2y$12$nPMNHiYhXb90lZTu0CX.2eY5RIQ/Uek28lCua23OIfkLhcjZtnIIO';
const categories = [
  'action',
  'adventure',
  'comedy',
  'crime',
  'drama',
  'fantasy',
  'historical',
  'horror',
  'mystery',
  'romance',
  'science-fiction',
  'thriller',
];
const tags = () => {
  return [
    'Increíble',
    'Suspenso',
    'Emocionante',
    'Etiqueta genérica',
    'Hola',
    'Chau',
  ].reduce((acc, tag) => {
    if (rand(1, 100) < 50) {
      return [...acc, tag];
    }

    return acc;
  }, []);
};

async function createUser({
  userId,
  username,
  email,
  name,
  lastname,
  birthdate,
  gender,
  biography,
  profileImage,
  createdAt,
}) {
  await client.query(
    `INSERT INTO users(
      id,
      provider,
      username,
      email,
      password,
      name,
      lastname,
      birthdate,
      gender,
      biography,
      profile_image,
      role_id,
      created_at
    ) VALUES (
      $1,
      $2,
      $3,
      $4,
      $5,
      $6,
      $7,
      $8,
      $9,
      $10,
      $11,
      $12,
      $13
    )`, [
      userId,
      'local',
      username,
      email,
      password,
      name,
      lastname,
      birthdate,
      gender,
      biography,
      profileImage,
      'user',
      createdAt,
    ]);
}

async function createPublication({
  publicationId,
  authorId,
  name,
  synopsis,
  categoryId,
  tags,
  cover,
  statistics,
  statusHistory,
  pages,
  createdAt,
}) {
  await client.query(
    `INSERT INTO publications(
      id,
      author_id,
      name,
      synopsis,
      category_id,
      tags,
      cover,
      statistics,
      status_history,
      pages,
      created_at
    ) VALUES (
      $1,
      $2,
      $3,
      $4,
      $5,
      $6,
      $7,
      $8,
      $9,
      $10,
      $11
    )`, [
      publicationId,
      authorId,
      name,
      synopsis,
      categoryId,
      tags,
      cover,
      statistics,
      JSON.stringify(statusHistory),
      JSON.stringify(pages),
      createdAt,
    ]);
}

async function createCollection({
  collectionId,
  authorId,
  name,
  synopsis,
  categoryId,
  tags,
  cover,
  items,
  createdAt,
}) {
  await client.query(
    `INSERT INTO collections(
      id,
      author_id,
      name,
      synopsis,
      category_id,
      tags,
      cover,
      items,
      created_at
    ) VALUES (
      $1,
      $2,
      $3,
      $4,
      $5,
      $6,
      $7,
      $8,
      $9
    )`, [
      collectionId,
      authorId,
      name,
      synopsis,
      categoryId,
      tags,
      cover,
      JSON.stringify(items),
      createdAt,
    ]);
}

async function main() {
  console.log('[ POPULATE ]');
  client.connect();
  console.log('Populating DB...');

  try {
    // Create user, publications and collections
    for (let i = 0; i < 10; i++) {
      const userId = uuidv4();
      await createUser({
        userId,
        username: `user-${i}`,
        email: `user-${i}@omics.com`,
        name: 'Name',
        lastname: 'Lastname',
        birthdate: '1994-08-01 15:30:00Z',
        gender: 'male',
        biography: 'My biography...',
        profileImage: image(200),
        createdAt: new Date(),
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
              admin_id: '00000000-0000-0000-0000-000000000002',
              comment: 'Todo está perfecto.',
              datetime: new Date(),
            });
          } else if (rand(0, 100) < 40) {
            statusHistory.push({
              status: 'published',
              admin_id: '00000000-0000-0000-0000-000000000002',
              comment: 'Tiene contenido que puede resultar ofensivo a los pandas.',
              datetime: new Date(),
            });
          }
        }

        await createPublication({
          publicationId,
          authorId: userId,
          name: `Publication ${i}-${j}`,
          synopsis: 'Synopsis...',
          categoryId: categories[rand(0, categories.length)],
          tags: tags(),
          cover: image(250),
          statistics: {
            views: rand(0, 1000),
            unique_views: rand(0, 850),
            readings: rand(0, 560),
            likes: rand(0, 175),
            reviews: rand(0, 85),
            stars: rand(0, 51) / 10.0,
          },
          statusHistory,
          pages,
          createdAt: new Date(),
        });

        publicationIds.push(publicationId);
      }

      for (let j = 0; j < rand(1, 6); j++) {
        const collectionId = uuidv4();

        const items = publicationIds
          .filter(_ => rand(1, 100) < 20)
          .map(pId => ({
            publication_id: { id: pId },
            date: new Date(),
          }));

        await createCollection({
          collectionId,
          authorId: userId,
          name: `Collection ${i}-${j}`,
          synopsis: 'I am a collection...',
          categoryId: categories[rand(0, categories.length)],
          tags: tags(),
          cover: image(250),
          items,
          createdAt: new Date(),
        });
      }
    }

    // Add interactions
    const { rows: users } = await client.query('SELECT * FROM users');
    const { rows: publications } = await client.query('SELECT * FROM publications');

    for (const user of users) {
      for (const publication of publications) {
        if (user.id != publication.author_id) {
          // if (rand(0, 100) < 50) {
          //   await client.query(
          //     `INSERT INTO views(reader_id, publication_id, datetime, is_unique)
          //     VALUES ($1, $2, $3, $4)`,
          //     [user.id, publication.id, new Date(), rand(0, 100) < 50 ? true : false],
          //   );
          // }
          //
          // if (rand(0, 100) < 30) {
          //   await client.query(
          //     `INSERT INTO readings(reader_id, publication_id, datetime)
          //     VALUES ($1, $2, $3)`,
          //     [user.id, publication.id, new Date()],
          //   );
          // }
          //
          // if (rand(0, 100) < 20) {
          //   await client.query(
          //     `INSERT INTO reviews(reader_id, publication_id, datetime, stars, comment)
          //     VALUES ($1, $2, $3, $4, $5)`,
          //     [user.id, publication.id, new Date(), rand(0, 6), 'Genial.'],
          //   );
          // }

          if (rand(0, 100) < 30) {
            await client.query(
              `INSERT INTO publication_favorites(reader_id, publication_id, datetime)
              VALUES ($1, $2, $3)`,
              [user.id, publication.id, new Date()],
            );
          }
        }
      }
    }

    // for (const user1 of users) {
    //   for (const user2 of users) {
    //     if (user1.id != user2.id && rand(0, 100) < 10) {
    //       await client.query(
    //         `INSERT INTO follows(reader_id, author_id, datetime)
    //         VALUES($1, $2, $3)`,
    //         [user1.id, user2.id, new Date()],
    //       )
    //     }
    //   }
    // }
  } catch (err) {
    console.log(err);
  } finally {
    console.log('READY');
    client.end();
  }
}

main();
