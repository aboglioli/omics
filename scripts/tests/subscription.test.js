const { v4: uuidv4 } = require('uuid');

const { knex } = require('../core/db');

describe('Subscription', () => {
  before(async () => {
    await knex('users')
      .insert({
        id: uuidv4(),
        provider: 'local',
        username: 'user-1',
        email: 'user-1@omics.com',
      });
  });

  after(async () => {
    await knex('users').where('username', 'user-1').del();
  });
});
