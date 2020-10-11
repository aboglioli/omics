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
const { rand, randArr } = require('../core/utils');

class Populator {
  constructor(db, comicSamples) {
    this.db = db;
    this.comicSamples = comicSamples;

    this.events = [];

    this.users = {};
    this.publications = {};
    this.collections = {};

    this.publicationFavorites = [];
    this.collectionFavorites = [];
    this.views = [];
    this.readings = [];
    this.likes = [];
    this.reviews = [];
    this.follows = [];
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
    userId,
    pageCount,
    status,
  }) {
    const id = uuid();
    const sample = randArr(this.comicSamples);
    const publication = {
      id,
      author_id: userId,
      name: sample.title || faker.name.findName(),
      synopsis: sample.description || faker.lorem.paragraph(),
      category_id: randArr(categories),
      tags: randArr(tags, true),
      cover: `${sample.images[0].path}.${sample.images[0].extension}`,
      // cover: image(250)// ,
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
    this.users[userId].publications += 1;

    return publication;
  }

  createCollection({
    userId,
    publicationIds,
  }) {
    const id = uuid();
    const collection = {
      id,
      author_id: userId,
      name: faker.name.findName(),
      synopsis: faker.lorem.paragraph(),
      category_id: randArr(categories),
      tags: randArr(tags, true),
      cover: image(250),
      items: publicationIds.map(id => ({
        pulication_id: id,
        date: new Date(),
      })),
      created_at: new Date(),
    };

    return collection;
  }

  createView({ userId, publicationId, unique }) {
    this.views.push({
      reader_id: userId,
      publication_id: publicationId,
      datetime: new Date(),
      is_unique: unique,
    });

    this.publications[publicationId].statistics.views += 1;
  }

  createReading({ userId, publicationId }) {
    this.readings.push({
      reader_id: userId,
      publication_id: publicationId,
      datetime: new Date(),
    });

    this.publications[publicationId].statistics.readings += 1;
  }

  createLike({ userId, publicationId }) {
    this.likes.push({
      reader_id: userId,
      publication_id: publicationId,
      datetime: new Date(),
    });

    this.publications[publicationId].statistics.likes += 1;
  }

  createReview({ userId, publicationId }) {
    this.reviews.push({
      reader_id: userId,
      publication_id: publicationId,
      datetime: new Date(),
      stars: rand(0, 6),
      comment: faker.lorem.paragraph(),
    });

    this.publications[publicationId].statistics.reviews += 1;
    const stars =
      this.reviews.reduce((acc, r) => acc + r.stars, 0.0) / this.reviews.length;
    this.publications[publicationId].statistics.stars = +stars.toFixed(2);

  }

  createPublicationFavorite({ userId, publicationId }) {
    this.publicationFavorites.push({
      reader_id: userId,
      publication_id: publicationId,
      datetime: new Date(),
    });
  }

  createCollectionFavorite({ userId, publicationId }) {
    this.collectionFavorites.push({
      reader_id: userId,
      publication_id: publicationId,
      datetime: new Date(),
    });
  }

  createFollow({ readerId, authorId }) {
    this.follows.push({
      reader_id: readerId,
      author_id: authorId,
      datetime: new Date(),
    });

    this.users[authorId].followers += 1;
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

    await this.db('views')
      .insert(this.views);

    await this.db('readings')
      .insert(this.readings);

    await this.db('likes')
      .insert(this.likes);

    await this.db('reviews')
      .insert(this.reviews);

    await this.db('publication_favorites')
      .insert(this.publicationFavorites);

    await this.db('collection_favorites')
      .insert(this.collectionFavorites);

    await this.db('follows')
      .insert(this.follows);
  }
}

module.exports = Populator;
