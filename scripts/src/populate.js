const assert = require('assert').strict;
const { req, sleep } = require('./request');

const rand = (min, max) => Math.floor(Math.random() * (max - min)) + min;

const image = size => `https://via.placeholder.com/${size ? size : '256'}.jpg`;
const password = 'P@asswd!';

const registerAndValidate = async cmd => {
  const { data: { id: userId } } = await req.post('/register', cmd);

  await sleep(100);
  const event = (await req.lastEvent()).payload.Registered;

  await req.get(`/users/${event.id}/validate/${event.validation_code}`);

  return userId;
};

async function categories() {
  await req.login('admin', password);

  await req.post('/categories', { id: 'action', name: 'Acción' });
  await req.post('/categories', { id: 'adventure', name: 'Aventura' });
  await req.post('/categories', { id: 'comedy', name: 'Comedia' });
  await req.post('/categories', { id: 'crime', name: 'Crimen' });
  await req.post('/categories', { id: 'drama', name: 'Drama' });
  await req.post('/categories', { id: 'fantasy', name: 'Fantasía' });
  await req.post('/categories', { id: 'historical', name: 'Histórico' });
  await req.post('/categories', { id: 'horror', name: 'Terror' });
  await req.post('/categories', { id: 'mystery', name: 'Misterio' });
  await req.post('/categories', { id: 'romance', name: 'Romance' });
  await req.post('/categories', { id: 'science-fiction', name: 'Ciencia Ficción' });
  await req.post('/categories', { id: 'thriller', name: 'Thriller' });

  req.logout();
}

async function contentManager() {
  await req.login('admin', password);

  const contentManagerId = await registerAndValidate({
    username: 'content-manager',
    email: 'content-manager@omics.com',
    password: 'P@asswd!',
  });

  await req.put(`/users/${contentManagerId}/role`, { role_id: 'content-manager' });

  req.logout();
}

async function user({
  username,
  email,
  name,
  lastname,
  birthdate,
  gender,
  biography,
}) {
  await registerAndValidate({
    username,
    email,
    password,
  });
  await req.login(username, password);
  await req.put('/users/me', {
    name,
    lastname,
    birthdate,
    gender,
    biography,
    profile_image: image(200),
  });
}

async function publication({
  name,
  synopsis,
  categoryId,
  tags,
  pagesCount,
  publish,
}) {
  const { data: { id: publicationId } } = await req.post('/publications', {
    name,
    synopsis,
    category_id: categoryId,
    tags,
    cover: image(250),
  });

  if (publish) {
    pagesCount += 1;
  }

  const pages = [];
  for (let i = 0; i < pagesCount; i++) {
    pages.push({ images: [image('663x1024')] });
  }

  await req.put(`/publications/${publicationId}/pages`, { pages });

  if (publish) {
    await req.post(`/publications/${publicationId}/publish`, {});
  }
}

async function collection({
  name,
  synopsis,
  categoryId,
  tags,
  publicationIds,
}) {
  const { data: { id: collectionId } } = await req.post('/collections', {
    name,
    synopsis,
    category_id: categoryId,
    tags,
    cover: image(250),
  });

  for (const publicationId of publicationIds) {
    await req.post(`/collections/${collectionId}/publication/${publicationId}`);
  }
}

async function main() {
  console.log('Populating...');

  try {
    await categories();
    await contentManager();

    for (let i = 0; i < 5; i++) {
      // User
      await user({
        username: `user-${i}`,
        email: `user-${i}@omics.com`,
        name: 'Name',
        lastname: 'Lastname',
        birthdate: '1994-05-06T15:30:00Z',
        gender: rand(0, 100) < 50 ? 'male' : 'female',
        biography: 'My amazing biography...',
      });

      // Publications
      const publicationsCount = rand(1, 10);
      const { data: { categories } } = await req.get('/categories');

      for (let i = 0; i < publicationsCount; i++) {
        await publication({
          name: `Publication ${i}`,
          synopsis: 'An amazing description...',
          categoryId: categories[rand(0, categories.length)].id,
          tags: ['Tag 1', 'Tag 2'],
          pagesCount: rand(0, 20),
          publish: rand(0, 100) < 80,
        });
      }

      // Collections
      const { data: { publications } } = await req.get('/authors/me/publications');
      const collectionsCount = rand(1, 4);
      for (let i = 0; i < collectionsCount; i++) {
        const publicationIds = [];
        for (const publication of publications) {
          if (rand(0, 100) < 40) {
            publicationIds.push(publication.id);
          }
        }

        await collection({
          name: `Collection ${i}`,
          synopsis: `This is a collection.`,
          categoryId: categories[rand(0, categories.length)].id,
          tags: ['Tag 2', 'Tag 3'],
          publicationIds,
        });
      }
    }

    await req.login('content-manager', password);
    const { data: { publications: waitingApprovalPublications } } = await req.get('/publications?status=waiting-approval');
    for (const publication of waitingApprovalPublications) {
      const r = rand(0, 100);
      if (r < 50) {
        await req.post(`/publications/${publication.id}/approve`, { comment: 'Todo correcto' });
      } else if (r < 70) {
        await req.post(`/publications/${publication.id}/reject`, { comment: 'La obra puede resultar ofensiva' });
      }
    }

    console.log('OK');
  } catch (err) {
    if (err.config && err.response) {
      console.log('Config:', err.config);
      console.log('Status:', err.response.status);
      console.log('Response:', err.response.data);
    } else {
      console.log(err)
    }
  }
}

main();
