const { connectDb } = require("../core/db");
const { rand, randArr } = require("../core/utils");
const samples = require("../comic-samples.json");

const Populator = require("../core/populator");

async function main() {
  console.log("[ POPULATE ]");
  const knex = connectDb();
  console.log("Populating DB...");

  const populator = new Populator(knex, samples.comics);

  try {
    // User
    for (let i = 0; i < 1200; i++) {
      const user = populator.createUser({ username: `user-${i}` });

      if (rand(0, 100) < 5) {
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

          populator.createPublication({
            userId: user.id,
            pageCount: rand(1, 20),
            status,
          });
        }

        // Collections
        for (let i = 0; i < rand(0, 5); i++) {
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
    }

    // Follows
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
    for (let i = 0; i < 10000; i++) {
      const user = randArr(Object.values(populator.users));
      const publication = randArr(Object.values(populator.publications));
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

      // if (!unique) {
      //   continue;
      // }

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

    await populator.save();
  } catch (err) {
    console.log(err);
  } finally {
    console.log("READY");
    await knex.destroy();
  }
}

main();
