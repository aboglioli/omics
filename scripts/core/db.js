const slugify = require('slugify');
const { Client } = require('pg');

const config = require('./config');

// PostgreSQL client
const client = new Client({
  user: config.postgres_username,
  host: config.postgres_host,
  database: config.postgres_database,
  password: config.postgres_password,
  port: config.postgres_port,
});

const knex = require('knex')({
  client: 'pg',
  connection: {
    host: config.postgres_host,
    user: config.postgres_username,
    password: config.postgres_password,
    database: config.postgres_database,
  },
});

// Utils
const image = size => `https://via.placeholder.com/${size ? size : '256'}.jpg`;
const password = '$2y$12$nPMNHiYhXb90lZTu0CX.2eY5RIQ/Uek28lCua23OIfkLhcjZtnIIO';

// Constants
const genders = ['male', 'female', 'other'];
const categories = [
  'action',
  'adventure',
  'comedy',
  'crime',
  'drama',
  'fantasy',
  'historical',
  'horror',
  'mystery',
  'romance',
  'science-fiction',
  'thriller',
];
const tags = [
  'Increíble',
  'Suspenso',
  'Emocionante',
  'Etiqueta genérica',
  'Hola',
  'Chau',
].map(tag => ({
  slug: slugify(tag, { lower: true }),
  name: tag,
}));

// Clean db
function clean() {
  return client.query(`
    DO $$ DECLARE
        r RECORD;
    BEGIN
        FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = current_schema()) LOOP
            EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
        END LOOP;
    END $$;
  `);
}

// Inserts
async function createUser({
  userId,
  username,
  email,
  name,
  lastname,
  birthdate,
  gender,
  biography,
  profileImage,
  createdAt,
}) {
  await client.query(
    `INSERT INTO users(
      id,
      provider,
      username,
      email,
      password,
      name,
      lastname,
      birthdate,
      gender,
      biography,
      profile_image,
      role_id,
      created_at
    ) VALUES (
      $1,
      $2,
      $3,
      $4,
      $5,
      $6,
      $7,
      $8,
      $9,
      $10,
      $11,
      $12,
      $13
    )`, [
      userId,
      'local',
      username,
      email,
      password,
      name,
      lastname,
      birthdate,
      gender,
      biography,
      profileImage,
      'user',
      createdAt,
    ]);
}

async function createPublication({
  publicationId,
  authorId,
  name,
  synopsis,
  categoryId,
  tags,
  cover,
  statistics,
  statusHistory,
  pages,
  createdAt,
}) {
  await client.query(
    `INSERT INTO publications(
      id,
      author_id,
      name,
      synopsis,
      category_id,
      tags,
      cover,
      statistics,
      status_history,
      pages,
      created_at
    ) VALUES (
      $1,
      $2,
      $3,
      $4,
      $5,
      $6,
      $7,
      $8,
      $9,
      $10,
      $11
    )`, [
      publicationId,
      authorId,
      name,
      synopsis,
      categoryId,
      JSON.stringify(tags),
      cover,
      statistics,
      JSON.stringify(statusHistory),
      JSON.stringify(pages),
      createdAt,
    ]);
}

async function createCollection({
  collectionId,
  authorId,
  name,
  synopsis,
  categoryId,
  tags,
  cover,
  items,
  createdAt,
}) {
  await client.query(
    `INSERT INTO collections(
      id,
      author_id,
      name,
      synopsis,
      category_id,
      tags,
      cover,
      items,
      created_at
    ) VALUES (
      $1,
      $2,
      $3,
      $4,
      $5,
      $6,
      $7,
      $8,
      $9
    )`, [
      collectionId,
      authorId,
      name,
      synopsis,
      categoryId,
      JSON.stringify(tags),
      cover,
      JSON.stringify(items),
      createdAt,
    ]);
}

module.exports = {
  client,
  knex,

  image,
  password,

  genders,
  categories,
  tags,

  clean,

  createUser,
  createPublication,
  createCollection,
};
