const { v4: uuidv4 } = require('uuid');
const client = require('./db');

const rand = (min, max) => Math.floor(Math.random() * (max - min)) + min;
const image = size => `https://via.placeholder.com/${size ? size : '256'}.jpg`;
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
    )`,
    [
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
    ],
  );
}

async function createPublication({
  publicationId,
  authorId,
  name,
  synopsis,
  categoryId,
  tags,
  cover,
  createdAt,
}) {
  let pages = [];
  for (let k = 0; k < 15; k++) {
    pages.push({
      number: k,
      images: [{
        url: image('663x1024'),
      }],
    });
  }

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
    )`,
    [
      publicationId,
      authorId,
      name,
      synopsis,
      categoryId,
      tags,
      cover,
      {
        views: rand(0, 1000),
        unique_views: rand(0, 850),
        readings: rand(0, 560),
        likes: rand(0, 175),
        reviews: rand(0, 85),
        stars: rand(0, 51) / 10.0,
      },
      JSON.stringify([{
        status: 'draft',
        datetime: new Date(),
      }, {
        status: 'published',
        admin_id: '00000000-0000-0000-0000-000000000002',
        comment: 'Comment...',
        datetime: new Date(),
      }]),
      JSON.stringify(pages),
      createdAt,
    ],
  );
}

async function main() {
  console.log('[ POPULATE ]');
  client.connect();
  console.log('Populating DB...');

  try {
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

      for (let j = 0; j < 20; j++) {
        const publicationId = uuidv4();

        await createPublication({
          publicationId,
          authorId: userId,
          name: `Publication ${i}-${j}`,
          synopsis: 'Synopsis...',
          categoryId: categories[rand(0, categories.length)],
          tags: ['Tag 1' ,'Tag 2'],
          cover: image(250),
          createdAt: new Date(),
        });
      }
    }
  } catch (err) {
    console.log(err);
  } finally {
    console.log('READY');
    client.end();
  }
}

main();
