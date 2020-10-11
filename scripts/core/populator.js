const { v4: uuid } = require('uuid');
const faker = require('faker');
faker.locale = 'es';

const {
  password,
  image,
  genders,
  categories,
  tags,
} = require('../core/constants');
const { randArr } = require('../core/utils');

class Populator {
  constructor(db) {
    this.db = db;

    this.events = [];

    this.users = {};
    this.publications = {};
    this.publicationFavorites = {};
    this.collections = {};
  }

  createUser({
    username,
  }) {
    const id = uuid();

    const user = {
      id,
      provider: 'local',
      username,
      email: `${username}@omics.com`,
      password,
      name: faker.name.firstName(),
      lastname: faker.name.lastName(),
      birthdate: '1994-08-01 15:30:00Z',
      gender: randArr(genders),
      biography: faker.lorem.paragraph(),
      profile_image: image(200),
      role_id: 'user',
      followers: 0,
      publications: 0,
      subscribed: false,
      created_at: new Date(),
    };

    this.users[id] = user;

    return user;
  }

  createPublication({
    user,
    pageCount,
    status,
  }) {
    const id = uuid();
    const publication = {
      id,
      author_id: user.id,
      name: faker.name.findName(),
      synopsis: faker.lorem.paragraph(),
      category_id: randArr(categories),
      tags: randArr(tags, true),
      cover: image(250),
      statistics: {
        views: 0,
        unique_views: 0,
        readings: 0,
        likes: 0,
        reviews: 0,
        stars: 0.0,
      },
      pages: [],
      status_history: [],
      created_at: new Date(),
    };

    pageCount = pageCount || 0;
    const pages = [];
    for (let i = 0; i < pageCount; i++) {
      pages.push({
        number: i,
        images: [{
          url: image('663x1024'),
        }],
      });
    }
    publication.pages = pages;

    const statusHistory = [{
      status: 'draft',
      datetime: new Date(),
    }];

    if (status) {
      statusHistory.push({
        status: 'waiting-approval',
        datetime: new Date(),
      });

      if (status === 'published') {
        statusHistory.push({
          status: 'published',
          admin_id: {
            id: '00000000-0000-0000-0000-000000000002',
          },
          comment: {
            comment: faker.lorem.paragraph(),
          },
          datetime: new Date(),
        });
      } else if (status === 'rejected') {
        statusHistory.push({
          status: 'rejected',
          admin_id: {
            id: '00000000-0000-0000-0000-000000000002'
          },
          comment: {
            comment: faker.lorem.paragraph()
          },
          datetime: new Date(),
        });
      }
    }
    publication.status_history = statusHistory;

    this.publications[id] = publication;

    user.publications += 1;

    return publication;
  }

  createCollection({
    user,
    publications,
  }) {
    const id = uuid();
    const collection = {
      id,
      author_id: user,
      name: faker.name.findName(),
      synopsis: faker.lorem.paragraph(),
      category_id: randArr(categories),
      tags: randArr(tags, true),
      cover: image(250),
      items: publications.map(({ id }) => ({
        pulication_id: id,
        date: new Date(),
      })),
      created_at: new Date(),
    };

    return collection;
  }

  async save() {
    await this.db('users')
      .insert(Object.values(this.users));

    await this.db('publications')
      .insert(
        Object.values(this.publications)
          .map(p => ({
            ...p,
            tags: JSON.stringify(p.tags),
            pages: JSON.stringify(p.pages),
            status_history: JSON.stringify(p.status_history),
          })),
      );

    await this.db('collections')
      .insert(
        Object.values(this.collections)
          .map(c => ({
            ...c,
            tags: JSON.stringify(c.tags),
            items: JSON.stringify(c.items),
          })),
      );

    // await this.db('publication_favorites')
    //   .insert(Object.values(this.publicationFavorites));
  }
}

module.exports = Populator;
