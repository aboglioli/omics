const { objToInsert, objToUpdate } = require('./sql');

describe('SQL', () => {
  test('objToInsert', () => {
    const res = objToInsert(
      'publications',
      {
        id: '#pub01',
        name: 'Publication 123',
        synopsis: 'Synopsis...',
        likes: 32,
        stars: 4.6,
        status: {
          code: 'published',
          datetime: '2020-05-01T15:30:00Z',
        },
      },
    );

    expect(res).toBe(
      'INSERT INTO publications(id, name, synopsis, likes, stars, status) VALUES($1, $2, $3, $4, $5, $6)'
    );
  });

  test('objToUpdate', () => {
    const res = objToUpdate(
      'publications',
      {
        name: 'Publication 123',
        synopsis: 'Synopsis...',
        status: {
          code: 'published',
          datetime: '2020-05-01T15:30:00Z',
        },
      },
      { pub_id: '#pub01'},
    );

    expect(res).toBe(
      'UPDATE publications SET name = $2, synopsis = $3, status = $4 WHERE pub_id = $1',
    );
  })
});
