const { v4: uuid } = require("uuid");
const faker = require("faker");
faker.locale = "es";

const {
  password,
  image,
  genders,
  categories,
  tags,
} = require("../core/constants");
const { rand, randArr } = require("../core/utils");

class Populator {
  constructor(db, comicSamples) {
    this.db = db;
    this.comicSamples = comicSamples;
    this.lastDate = new Date("2020-05-01T14:30:00");

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

    this.subscriptions = [];
  }

  nextDate() {
    const date = new Date(this.lastDate);
    date.setSeconds(date.getSeconds() + 10);

    this.lastDate = date;

    return date;
  }

  addEvent(topic, code, payload) {
    const id = uuid();
    this.events.push({
      id,
      topic,
      code,
      timestamp: this.nextDate(),
      payload,
    });
  }

  createUser({ username }) {
    const id = uuid();

    const user = {
      id,
      provider: "local",
      username,
      email: `${username}@omics.com`,
      password,
      name: faker.name.firstName(),
      lastname: faker.name.lastName(),
      birthdate: "1994-08-01 15:30:00Z",
      gender: randArr(genders),
      biography: faker.lorem.paragraph(),
      profile_image: image(200),
      role_id: "user",
      followers: 0,
      publications: 0,
      subscribed: false,
      created_at: this.nextDate(),
      updated_at: this.nextDate(),
    };

    this.users[id] = user;

    this.addEvent("user", "registered", {
      Registered: {
        id: user.id,
        username: user.username,
        email: user.email,
        validation_code: "#fake-code",
      },
    });
    this.addEvent("user", "validated", { Validated: { id: user.id } });
    this.addEvent("user", "updated", {
      Updated: {
        id: user.id,
        name: user.name,
        lastname: user.lastname,
        birthdate: user.birthdate,
        gender: user.gender,
        biography: user.biography,
        profile_image: user.profile_image,
      },
    });

    return user;
  }

  createPublication({ userId, pageCount, status }) {
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
      created_at: this.nextDate(),
      updated_at: this.nextDate(),
    };
    this.addEvent("publication", "created", {
      Created: {
        id: publication.id,
        author_id: publication.author_id,
        name: publication.name,
        synopsis: publication.synopsis,
        category_id: publication.category_id,
        tags: publication.tags.map((t) => t.name),
        cover: publication.cover,
      },
    });

    pageCount = pageCount || 0;
    const pages = [];
    for (let i = 0; i < pageCount; i++) {
      pages.push({
        number: i,
        images: [
          {
            url: image("663x1024"),
          },
        ],
      });
    }
    publication.pages = pages;
    this.addEvent("publication", "pages-updated", {
      PagesUpdated: {
        id: publication.id,
        pages_count: publication.pages.length,
      },
    });

    const statusHistory = [
      {
        status: "draft",
        datetime: this.nextDate(),
      },
    ];

    if (status) {
      statusHistory.push({
        status: "waiting-approval",
        datetime: this.nextDate(),
      });
      this.addEvent("publication", "approval-waited", {
        ApprovalWaited: {
          id: publication.id,
        },
      });

      if (status === "published") {
        statusHistory.push({
          status: "published",
          admin_id: {
            id: "00000000-0000-0000-0000-000000000002",
          },
          comment: {
            comment: faker.lorem.paragraph(),
          },
          datetime: this.nextDate(),
        });
        this.addEvent("publication", "published", {
          Published: {
            id: publication.id,
            author_id: publication.author_id,
            name: publication.name,
            synopsis: publication.synopsis,
            category_id: publication.category_id,
            tags: publication.tags.map((t) => t.name),
            cover: publication.cover,
            pages_count: publication.pages.length,
          },
        });
      } else if (status === "rejected") {
        statusHistory.push({
          status: "rejected",
          admin_id: {
            id: "00000000-0000-0000-0000-000000000002",
          },
          comment: {
            comment: faker.lorem.paragraph(),
          },
          datetime: this.nextDate(),
        });
        this.addEvent("publication", "rejected", {
          Rejected: {
            id: publication.id,
          },
        });
      }
    }
    publication.status_history = statusHistory;

    this.publications[id] = publication;
    this.users[userId].publications += 1;

    return publication;
  }

  createCollection({ userId, publicationIds }) {
    const id = uuid();
    const collection = {
      id,
      author_id: userId,
      name: faker.name.findName(),
      synopsis: faker.lorem.paragraph(),
      category_id: randArr(categories),
      tags: randArr(tags, true),
      cover: image(250),
      items: publicationIds.map((id) => ({
        pulication_id: id,
        date: this.nextDate(),
      })),
      created_at: this.nextDate(),
      updated_at: this.nextDate(),
    };

    this.addEvent("collection", "created", {
      Created: {
        id: collection.id,
        author_id: collection.author_id,
        name: collection.name,
        synopsis: collection.synopsis,
        category_id: collection.category_id,
        tags: collection.tags.map((t) => t.name),
        cover: collection.cover,
      },
    });
    collection.items.forEach((item) => {
      this.addEvent("collection", "publication-added", {
        PublicationAdded: {
          id: collection.id,
          publication_id: item.publication_id,
        },
      });
    });

    return collection;
  }

  createView({ userId, publicationId, unique }) {
    this.views.push({
      reader_id: userId,
      publication_id: publicationId,
      datetime: this.nextDate(),
      is_unique: unique,
    });
    this.addEvent("publication", "viewed", {
      Viewed: {
        reader_id: userId,
        publication_id: publicationId,
        unique,
      },
    });

    this.publications[publicationId].statistics.views += 1;
  }

  createReading({ userId, publicationId }) {
    this.readings.push({
      reader_id: userId,
      publication_id: publicationId,
      datetime: this.nextDate(),
    });
    this.addEvent("publication", "read", {
      Read: {
        reader_id: userId,
        publication_id: publicationId,
      },
    });

    this.publications[publicationId].statistics.readings += 1;
  }

  createLike({ userId, publicationId }) {
    this.likes.push({
      reader_id: userId,
      publication_id: publicationId,
      datetime: this.nextDate(),
    });
    this.addEvent("publication", "liked", {
      Liked: {
        reader_id: userId,
        publication_id: publicationId,
      },
    });

    this.publications[publicationId].statistics.likes += 1;
  }

  createReview({ userId, publicationId }) {
    const stars = rand(0, 6);
    const comment = faker.lorem.paragraph();

    this.reviews.push({
      reader_id: userId,
      publication_id: publicationId,
      datetime: this.nextDate(),
      stars,
      comment,
    });

    this.publications[publicationId].statistics.reviews += 1;
    const avgStars =
      this.reviews
        .filter((r) => r.publication_id === publicationId)
        .reduce((acc, r) => acc + r.stars, 0.0) / this.reviews.length;
    this.publications[publicationId].statistics.stars = +avgStars.toFixed(2);

    this.addEvent("publication", "reviewed", {
      Reviewed: {
        reader_id: userId,
        publication_id: publicationId,
        stars,
        comment,
      },
    });
  }

  createPublicationFavorite({ userId, publicationId }) {
    this.publicationFavorites.push({
      reader_id: userId,
      publication_id: publicationId,
      datetime: this.nextDate(),
    });
    this.addEvent("reader", "publication-added-to-favorites", {
      PublicationAddedToFavorites: {
        reader_id: userId,
        publication_id: publicationId,
      },
    });
  }

  createCollectionFavorite({ userId, publicationId }) {
    this.collectionFavorites.push({
      reader_id: userId,
      publication_id: publicationId,
      datetime: this.nextDate(),
    });
    this.addEvent("reader", "collection-added-to-favorites", {
      CollectionAddedToFavorites: {
        reader_id: userId,
        publication_id: publicationId,
      },
    });
  }

  createFollow({ readerId, authorId }) {
    this.follows.push({
      reader_id: readerId,
      author_id: authorId,
      datetime: this.nextDate(),
    });
    this.addEvent("author", "followed", {
      Followed: {
        author_id: authorId,
        reader_id: readerId,
      },
    });

    this.users[authorId].followers += 1;
  }

  createSubscription({ userId, status }) {
    const id = uuid();
    const subscription = {
      id,
      user_id: userId,
      plan: {
        plan_id: {
          id: "basic",
        },
        price: 75.0,
        assigned_at: this.nextDate(),
      },
      created_at: this.nextDate(),
      updated_at: this.nextDate(),
    };
    this.addEvent("subscription", "created", {
      Created: {
        id,
        user_id: userId,
        plan_id: "basic",
      },
    });

    const statusHistory = [
      {
        status: "waiting-for-payment",
        datetime: this.nextDate(),
      },
    ];
    const payments = [];
    if (status === "active") {
      statusHistory.push([
        {
          status: "active",
          datetime: this.nextDate(),
        },
      ]);
      status.push({
        kind: "income",
        amount: subscription.plan.price,
        datetime: this.nextDate(),
      });
      this.addEvent("subscription", "payment-added", {
        PaymentAdded: {
          id: subscription.id,
          user_id: subscription.user_id,
          amount: subscription.plan.price,
        },
      });
    } else if (status === "inactive") {
      statusHistory.push([
        {
          status: "inactive",
          datetime: this.nextDate(),
        },
      ]);
    }
    subscription.status_history = statusHistory;
    subscription.payments = payments;

    this.subscriptions.push(subscription);

    return subscription;
  }

  async save() {
    await this.db("users").insert(Object.values(this.users));

    await this.db("publications").insert(
      Object.values(this.publications).map((p) => ({
        ...p,
        tags: JSON.stringify(p.tags),
        pages: JSON.stringify(p.pages),
        status_history: JSON.stringify(p.status_history),
      }))
    );

    await this.db("collections").insert(
      Object.values(this.collections).map((c) => ({
        ...c,
        tags: JSON.stringify(c.tags),
        items: JSON.stringify(c.items),
      }))
    );

    await this.db("views").insert(this.views);

    await this.db("readings").insert(this.readings);

    await this.db("likes").insert(this.likes);

    await this.db("reviews").insert(this.reviews);

    await this.db("publication_favorites").insert(this.publicationFavorites);

    await this.db("collection_favorites").insert(this.collectionFavorites);

    await this.db("follows").insert(this.follows);

    // Events
    for (; this.events.length > 0; ) {
      const events = this.events.splice(0, 10000);
      await this.db("events").insert(events);
    }
  }
}

module.exports = Populator;
