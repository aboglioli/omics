CREATE TABLE IF NOT EXISTS roles (
  id VARCHAR(255) PRIMARY KEY,
  name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS permissions (
  module VARCHAR(255) NOT NULL,
  permissions VARCHAR(16) NOT NULL,
  role_id VARCHAR(255) NOT NULL,
  PRIMARY KEY(role_id, module),
  FOREIGN KEY(role_id) REFERENCES roles(id)
);

CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY,

  provider VARCHAR(16) NOT NULL,
  username VARCHAR(64) UNIQUE NOT NULL,
  email VARCHAR(64) UNIQUE NOT NULL,
  password VARCHAR(128),

  name VARCHAR(64),
  lastname VARCHAR(64),
  birthdate TIMESTAMP,
  gender VARCHAR(16),
  biography TEXT,
  profile_image VARCHAR(1024),

  role_id VARCHAR(255) NOT NULL,

  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP,

  FOREIGN KEY(role_id) REFERENCES roles(id)
);

CREATE TABLE IF NOT EXISTS authors (
  id UUID PRIMARY KEY,

  followers INTEGER DEFAULT 0,

  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP,

  FOREIGN KEY(id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS readers (
  id UUID PRIMARY KEY,

  subscribed BOOLEAN DEFAULT FALSE,

  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP,

  FOREIGN KEY(id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS categories (
  id VARCHAR(255) PRIMARY KEY,

  name VARCHAR(255) NOT NULL,

  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP
);


CREATE TABLE IF NOT EXISTS collections (
  id UUID PRIMARY KEY,

  author_id UUID NOT NULL,

  name VARCHAR(255) NOT NULL,
  synopsis TEXT NOT NULL,
  category_id VARCHAR(255) NOT NULL,
  tags TEXT[],
  cover VARCHAR(1024) NOT NULL,

  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP,

  FOREIGN KEY(author_id) REFERENCES authors(id),
  FOREIGN KEY(category_id) REFERENCES categories(id)
);

CREATE TABLE IF NOT EXISTS publications (
  id UUID PRIMARY KEY,

  author_id UUID NOT NULL,

  name VARCHAR(255) NOT NULL,
  synopsis TEXT NOT NULL,
  category_id VARCHAR(255) NOT NULL,
  tags TEXT[],
  cover VARCHAR(1024) NOT NULL,

  contract BOOLEAN DEFAULT FALSE,

  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP,

  FOREIGN KEY(author_id) REFERENCES authors(id),
  FOREIGN KEY(category_id) REFERENCES categories(id)
);

CREATE TABLE IF NOT EXISTS collection_items (
  collection_id UUID NOT NULL,
  publication_id UUID NOT NULL,
  datetime TIMESTAMP NOT NULL,

  PRIMARY KEY(collection_id, publication_id),
  FOREIGN KEY(collection_id) REFERENCES collections(id),
  FOREIGN KEY(publication_id) REFERENCES publications(id)
);

CREATE TABLE IF NOT EXISTS pages (
  publication_id UUID NOT NULL,
  number INTEGER NOT NULL,

  PRIMARY KEY(publication_id, number),
  FOREIGN KEY(publication_id) REFERENCES publications(id)
);

CREATE TABLE IF NOT EXISTS page_items (
  id SERIAL PRIMARY KEY,
  image_url VARCHAR(1024) NOT NULL,

  publication_id UUID NOT NULL,
  number INTEGER NOT NULL,

  FOREIGN KEY(publication_id, number) REFERENCES pages(publication_id, number)
);

CREATE TABLE IF NOT EXISTS statistics (
  publication_id UUID PRIMARY KEY,

  views INTEGER DEFAULT 0,
  unique_views INTEGER DEFAULT 0,
  readings INTEGER DEFAULT 0,
  likes INTEGER DEFAULT 0,
  reviews INTEGER DEFAULT 0,
  stars REAL DEFAULT 0.0,

  FOREIGN KEY(publication_id) REFERENCES publications(id)
);

CREATE TABLE IF NOT EXISTS publication_status (
  id VARCHAR(255) PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS publication_status_history (
  id SERIAL PRIMARY KEY,
  publication_id UUID NOT NULL,
  status_id VARCHAR(255) NOT NULL,

  admin_id UUID,
  comment TEXT,
  datetime TIMESTAMP NOT NULL,

  FOREIGN KEY(publication_id) REFERENCES publications(id),
  FOREIGN KEY(status_id) REFERENCES publication_status(id),
  FOREIGN KEY(admin_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS views (
  id SERIAL PRIMARY KEY,

  reader_id UUID NOT NULL,
  publication_id UUID NOT NULL,
  datetime TIMESTAMP NOT NULL,

  is_unique BOOLEAN DEFAULT FALSE,

  FOREIGN KEY(reader_id) REFERENCES readers(id),
  FOREIGN KEY(publication_id) REFERENCES publications(id)
);

CREATE TABLE IF NOT EXISTS readings (
  id SERIAL PRIMARY KEY,

  reader_id UUID NOT NULL,
  publication_id UUID NOT NULL,
  datetime TIMESTAMP NOT NULL,

  FOREIGN KEY(reader_id) REFERENCES readers(id),
  FOREIGN KEY(publication_id) REFERENCES publications(id)
);

CREATE TABLE IF NOT EXISTS likes (
  id SERIAL PRIMARY KEY,

  reader_id UUID NOT NULL,
  publication_id UUID NOT NULL,
  datetime TIMESTAMP NOT NULL,

  FOREIGN KEY(reader_id) REFERENCES readers(id),
  FOREIGN KEY(publication_id) REFERENCES publications(id)
);

CREATE TABLE IF NOT EXISTS reviews (
  id SERIAL PRIMARY KEY,

  reader_id UUID NOT NULL,
  publication_id UUID NOT NULL,
  datetime TIMESTAMP NOT NULL,

  stars SMALLINT DEFAULT 0,
  comment TEXT NOT NULL,

  FOREIGN KEY(reader_id) REFERENCES readers(id),
  FOREIGN KEY(publication_id) REFERENCES publications(id)
);
