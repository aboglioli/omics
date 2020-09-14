const Request = require('./request');

describe('Author', () => {
  let req;

  beforeAll(async () => {
    req = new Request();
  });

  test('get all', async () => {
    const res = await req.get('/authors');
    expect(res.data.authors).toHaveLength(3);
  });
});

