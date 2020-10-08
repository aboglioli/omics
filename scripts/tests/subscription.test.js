const { v4: uuidv4 } = require('uuid');

const { knex, password } = require('../core/db');
const { req } = require('../core/request');

describe('Subscription', () => {
  const user_id = uuidv4();

  beforeAll(async () => {
    await knex('users')
      .insert({
        id: user_id,
        provider: 'local',
        username: 'user-1',
        email: 'user-1@omics.com',
        password,
        name: 'Name',
        lastname: 'Lastname',
        birthdate: '1994-05-01T14:30:00Z',
        gender: 'male',
        role_id: 'user',
        created_at: new Date(),
      });

    await req.login('admin-1', 'P@asswd!');
  });

  afterAll(async () => {
    await knex('users').where('username', 'user-1').del();
    await knex.destroy();
  });

  test('example', async () => {
    let { data: user } = await req.get(`/users/${user_id}`);
    expect(user.username).toBe('user-1');
  });
});
