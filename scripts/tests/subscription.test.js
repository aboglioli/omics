const { v4: uuidv4 } = require('uuid');

const { connectDb } = require('../core/db');
const { password } = require('../core/constants');
const { req } = require('../core/request');

describe('Subscription', () => {
  let knex;
  const user_id = uuidv4();
  const publication_id = uuidv4();

  beforeAll(async () => {
    knex = connectDb();

    await knex('users')
      .insert({
        id: user_id,
        provider: 'local',
        username: 'sub-user-1',
        email: 'sub-user-1@omics.com',
        password,
        name: 'Name',
        lastname: 'Lastname',
        birthdate: '1994-05-01T14:30:00Z',
        gender: 'male',
        role_id: 'user',
        created_at: new Date(),
      });

    await knex('publications')
      .insert({
        id: publication_id,
        author_id: user_id,
        name: 'Name',
        synopsis: 'Synopsis...',
        category_id: 'adventure',
        tags: JSON.stringify([{ slug: 'tag-1', name: 'Tag 1' }]),
        cover: 'cover.jpg',
        statistics: {
          views: 0,
          unique_views: 0,
          readings: 0,
          likes: 0,
          reviews: 0,
          stars: 0.0,
        },
        pages: JSON.stringify([{
          number: 0,
          images: [{
            url: 'page0.jgp',
          }]
        }]),
        status_history: JSON.stringify([{
          status: 'draft',
          datetime: new Date(),
        }, {
          status: 'waiting-approval',
          datetime: new Date(),
        }, {
          status: 'published',
          admin_id: {
            id: '00000000-0000-0000-0000-000000000002',
          },
          comment: {
            comment: 'Comment...',
          },
          datetime: new Date(),
        }]),
        created_at: new Date(),
      });

    await req.login('admin-1', 'P@asswd!');
  });

  afterAll(async () => {
    await knex.destroy();
  });

  test('example', async () => {
    let { data: user } = await req.get(`/users/${user_id}`);
    expect(user.username).toBe('sub-user-1');

    let { data: { publication } } = await req.get(`/publications/${publication_id}`);
    expect(publication.id).toBe(publication_id);
  });
});
