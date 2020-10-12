const { v4: uuidv4 } = require("uuid");

const { connectDb } = require("../core/db");
const { password } = require("../core/constants");
const { req } = require("../core/request");

describe("Contract", () => {
  let knex;
  const userIds = [];
  const publicationIds = [];

  beforeAll(async () => {
    knex = connectDb();

    for (let i = 0; i < 5; i++) {
      const userId = uuidv4();
      userIds.push(userId);
      await knex("users").insert({
        id: userId,
        provider: "local",
        username: `con-user-${i + 1}`,
        email: `con-user-${i + 1}@omics.com`,
        password,
        role_id: "user",
        created_at: new Date(),
      });

      const publicationId = uuidv4();
      publicationIds.push(publicationId);
      await knex("publications").insert({
        id: publicationId,
        author_id: userId,
        name: `Publication ${i}`,
        synopsis: "Synopsis...",
        category_id: "adventure",
        tags: JSON.stringify([{ slug: "tag-1", name: "Tag 1" }]),
        cover: "cover.jpg",
        statistics: {
          views: 0,
          unique_views: 0,
          readings: 0,
          likes: 0,
          reviews: 0,
          stars: 0.0,
        },
        pages: JSON.stringify([
          {
            number: 0,
            images: [
              {
                url: "page0.jgp",
              },
            ],
          },
        ]),
        status_history: JSON.stringify([
          {
            status: "draft",
            datetime: new Date(),
          },
          {
            status: "waiting-approval",
            datetime: new Date(),
          },
          {
            status: "published",
            admin_id: {
              id: "00000000-0000-0000-0000-000000000002",
            },
            comment: {
              comment: "Comment...",
            },
            datetime: new Date(),
          },
        ]),
        created_at: new Date(),
      });
    }
  });

  afterAll(async () => {
    await knex.destroy();
  });

  test("interactions", async () => {
    await req.login("con-user-1", "P@asswd!");
    await req.get(`/publications/${publicationIds[1]}`);
    await req.get(`/publications/${publicationIds[2]}`);
    await req.get(`/publications/${publicationIds[3]}`);

    await req.login("con-user-2", "P@asswd!");
    await req.get(`/publications/${publicationIds[0]}`);
    await req.get(`/publications/${publicationIds[1]}`);
  });
});
