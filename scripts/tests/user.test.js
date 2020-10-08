const { connectDb } = require('../core/db');
const { req } = require('../core/request');

describe('User', () => {
  let knex;
  let userId;

  beforeAll(async () => {
    knex = connectDb();
  });

  afterAll(async () => {
    await knex.destroy();
  });

  test('register', async () => {
    const { data: registerRes } = await req.post('/register', {
      username: 'user-1',
      email: 'user-1@omics.com',
      password: 'P@asswd!',
    });

    userId = registerRes.id;
  });

  test('validate', async () => {
    const user = await knex('users').where('id', userId).first();
    expect(user).not.toBeNull();
    expect(user.validation_code.length).toBeGreaterThan(5);

    try {
      await req.post('/login', {
        username: 'user-1',
        password: 'P@asswd!',
      });
    } catch (err) {
      expect(err).not.toBeNull();
    }

    await req.get(`/users/${user.id}/validate/${user.validation_code}`);
  });

  test('login', async () => {
    const { data: loginRes } = await req.post('/login', {
      username: 'user-1',
      password: 'P@asswd!',
    });
    expect(loginRes.auth_token.length).toBeGreaterThan(5);

    await req.login('user-1', 'P@asswd!');
  });

  test('update', async () => {
    const { data: { ok } } = await req.put(`/users/${userId}`, {
      name: 'Name',
      lastname: 'Lastname',
      birthdate: '1994-08-01T15:30:00Z',
      gender: 'male',
      biography: 'Hello World',
    });
    expect(ok).toBeTruthy();
  });

  test('get by id', async () => {
    const { data: user } = await req.get(`/users/${userId}`);
    expect(user.name).toBe('Name');
    expect(user.lastname).toBe('Lastname');
    expect(user.birthdate).toBe('1994-08-01T15:30:00+00:00');
    expect(user.gender).toBe('male');
    expect(user.biography).toBe('Hello World');
  });
});
