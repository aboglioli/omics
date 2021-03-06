const { connectDb } = require("../core/db");
const { rand, randArr } = require("../core/utils");
const samples = require("../comic-samples.json");

const Populator = require("../core/populator");

function createCompleteUser(populator) {
  const user = populator.createUser({
    username: "the-user",
  });

  const publication = populator.createPublication({
    userId: user.id,
    pageCount: 5,
    status: "published",
  });

  for (const reader of Object.values(populator.users)) {
    if (user.id === reader.id) {
      continue;
    }

    populator.createView({
      userId: reader.id,
      publicationId: publication.id,
      unique: true,
    });
    populator.createReading({
      userId: reader.id,
      publicationId: publication.id,
    });
    populator.createLike({
      userId: reader.id,
      publicationId: publication.id,
    });
  }
}

async function main() {
  console.log("[ POPULATE ]");
  const knex = connectDb();
  console.log("Populating DB...");

  const date = new Date();
  date.setHours(date.getHours() - 24 * 60);

  const populator = new Populator(knex, samples.comics, date);

  try {
    // User
    for (let i = 0; i < 10000; i++) {
      const user = populator.createUser({ username: `user-${i}` });

      if (rand(0, 100) < 8) {
        // Publications
        for (let i = 0; i < rand(0, 10); i++) {
          let status = null;
          if (rand(0, 100) < 80) {
            status = "waiting-approval";

            const r = rand(0, 100);
            if (r < 40) {
              status = "published";
            } else if (r < 60) {
              status = "rejected";
            }
          }

          const publication = populator.createPublication({
            userId: user.id,
            pageCount: rand(1, 20),
            status,
          });

          const is_published = status === "published";
          if (is_published && rand(0, 100) < 20) {
            populator.createContract({
              publicationId: publication.id,
              userId: user.id,
              summaryCount: rand(3, 15),
            });
          }
        }

        // Collections
        if (rand(0, 100) < 10) {
          populator.createCollection({
            userId: user.id,
            publicationIds: randArr(
              Object.values(populator.publications)
                .filter((p) => p.author_id === user.id)
                .map((p) => p.id),
              true
            ),
          });
        }
      }

      const oldLastDate = new Date(populator.lastDate);
      const newDate = new Date();
      newDate.setHours(newDate.getHours() - 24 * 10);
      populator.lastDate = newDate;
      // if (rand(0, 100) < 5) {
      if (i < 1200) {
        populator.createSubscription({
          userId: user.id,
          planPrice: 75.0,
        });
      }
      populator.lastDate = oldLastDate;
    }

    // Follows
    const now = new Date();
    now.setHours(now.getHours() - 24 * 8);
    populator.lastDate = now;
    for (let i = 0; i < 200; i++) {
      const reader = randArr(Object.values(populator.users));
      const author = randArr(
        Object.values(populator.users).filter((u) => u.publications > 0)
      );

      if (reader.id != author.id) {
        populator.createFollow({ readerId: reader.id, authorId: author.id });
      }
    }

    // Views
    const interactions = [];
    const users = Object.values(populator.users);
    const publications = Object.values(populator.publications).filter(
      (p) =>
        p.status_history[p.status_history.length - 1].status === "published"
    );
    for (let i = 0; i < 10000; i++) {
      const user = randArr(users);
      const publication = randArr(publications);

      const unique = !interactions.some(
        (i) => i[0] === user.id && i[1] === publication.id
      );

      if (user.id == publication.author_id) {
        continue;
      }

      populator.createView({
        userId: user.id,
        publicationId: publication.id,
        unique,
      });

      if (rand(0, 100) < 70) {
        populator.createReading({
          userId: user.id,
          publicationId: publication.id,
        });

        if (rand(0, 100) < 40) {
          populator.createLike({
            userId: user.id,
            publicationId: publication.id,
          });
        }

        if (rand(0, 100) < 20) {
          populator.createReview({
            userId: user.id,
            publicationId: publication.id,
          });
        }

        if (rand(0, 100) < 20) {
          populator.createPublicationFavorite({
            userId: user.id,
            publicationId: publication.id,
          });
        }
      }

      interactions.push([user.id, publication.id]);
    }

    // Donations
    for (let i = 0; i < 500; i++) {
      const reader = randArr(Object.values(populator.users));
      const author = randArr(
        Object.values(populator.users).filter((u) => u.publications > 0)
      );

      if (reader.id != author.id) {
        populator.createDonation({ readerId: reader.id, authorId: author.id });
      }
    }

    populator.generateSummariesForContracts();

    createCompleteUser(populator);

    await populator.save();
  } catch (err) {
    console.log(err);
  } finally {
    console.log("READY");
    await knex.destroy();
  }
}

main();
