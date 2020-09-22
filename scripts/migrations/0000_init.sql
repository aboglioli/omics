CREATE TABLE IF NOT EXISTS roles (
  id VARCHAR(255) PRIMARY KEY,
  name VARCHAR(255) NOT NULL,

  permissions JSONB,

  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE,
  deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY,

  provider VARCHAR(16) NOT NULL,
  username VARCHAR(64) UNIQUE NOT NULL,
  email VARCHAR(64) UNIQUE NOT NULL,
  password VARCHAR(128),

  name VARCHAR(64),
  lastname VARCHAR(64),
  birthdate TIMESTAMP WITH TIME ZONE,
  gender VARCHAR(16),
  biography TEXT,
  profile_image VARCHAR(1024),

  role_id VARCHAR(255) NOT NULL,

  validation_code VARCHAR(255),

  followers INTEGER DEFAULT 0,
  subscribed BOOLEAN DEFAULT FALSE,

  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE,
  deleted_at TIMESTAMP WITH TIME ZONE,

  FOREIGN KEY(role_id) REFERENCES roles(id)
);

CREATE TABLE IF NOT EXISTS categories (
  id VARCHAR(255) PRIMARY KEY,

  name VARCHAR(255) NOT NULL,

  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE,
  deleted_at TIMESTAMP WITH TIME ZONE
);


CREATE TABLE IF NOT EXISTS collections (
  id UUID PRIMARY KEY,

  author_id UUID NOT NULL,

  name VARCHAR(255) NOT NULL,
  synopsis TEXT NOT NULL,
  category_id VARCHAR(255) NOT NULL,
  tags TEXT[] NOT NULL,
  cover VARCHAR(1024) NOT NULL,

  items JSONB NOT NULL,

  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE,
  deleted_at TIMESTAMP WITH TIME ZONE,

  FOREIGN KEY(author_id) REFERENCES users(id),
  FOREIGN KEY(category_id) REFERENCES categories(id)
);

CREATE TABLE IF NOT EXISTS publications (
  id UUID PRIMARY KEY,

  author_id UUID NOT NULL,

  name VARCHAR(255) NOT NULL,
  synopsis TEXT NOT NULL,
  category_id VARCHAR(255) NOT NULL,
  tags TEXT[] NOT NULL,
  cover VARCHAR(1024) NOT NULL,

  contract BOOLEAN DEFAULT FALSE,

  statistics JSONB NOT NULL,
  pages JSONB NOT NULL,

  status_history JSONB NOT NULL,

  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE,
  deleted_at TIMESTAMP WITH TIME ZONE,

  FOREIGN KEY(author_id) REFERENCES users(id),
  FOREIGN KEY(category_id) REFERENCES categories(id)
);

CREATE TABLE IF NOT EXISTS publication_status (
  id VARCHAR(255) PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS views (
  id SERIAL PRIMARY KEY,

  reader_id UUID NOT NULL,
  publication_id UUID NOT NULL,
  datetime TIMESTAMP WITH TIME ZONE NOT NULL,

  is_unique BOOLEAN DEFAULT FALSE,

  FOREIGN KEY(reader_id) REFERENCES users(id),
  FOREIGN KEY(publication_id) REFERENCES publications(id)
);

CREATE TABLE IF NOT EXISTS readings (
  id SERIAL PRIMARY KEY,

  reader_id UUID NOT NULL,
  publication_id UUID NOT NULL,
  datetime TIMESTAMP WITH TIME ZONE NOT NULL,

  FOREIGN KEY(reader_id) REFERENCES users(id),
  FOREIGN KEY(publication_id) REFERENCES publications(id)
);

CREATE TABLE IF NOT EXISTS likes (
  id SERIAL PRIMARY KEY,

  reader_id UUID NOT NULL,
  publication_id UUID NOT NULL,
  datetime TIMESTAMP WITH TIME ZONE NOT NULL,

  FOREIGN KEY(reader_id) REFERENCES users(id),
  FOREIGN KEY(publication_id) REFERENCES publications(id)
);

CREATE TABLE IF NOT EXISTS reviews (
  id SERIAL PRIMARY KEY,

  reader_id UUID NOT NULL,
  publication_id UUID NOT NULL,
  datetime TIMESTAMP WITH TIME ZONE NOT NULL,

  stars SMALLINT DEFAULT 0,
  comment TEXT NOT NULL,

  FOREIGN KEY(reader_id) REFERENCES users(id),
  FOREIGN KEY(publication_id) REFERENCES publications(id)
);

CREATE TABLE IF NOT EXISTS publication_favorites (
  id SERIAL PRIMARY KEY,

  reader_id UUID NOT NULL,
  publication_id UUID NOT NULL,
  datetime TIMESTAMP WITH TIME ZONE NOT NULL,

  FOREIGN KEY(reader_id) REFERENCES users(id),
  FOREIGN KEY(publication_id) REFERENCES publications(id)
);

CREATE TABLE IF NOT EXISTS collection_favorites (
  id SERIAL PRIMARY KEY,

  reader_id UUID NOT NULL,
  collection_id UUID NOT NULL,
  datetime TIMESTAMP WITH TIME ZONE NOT NULL,

  FOREIGN KEY(reader_id) REFERENCES users(id),
  FOREIGN KEY(collection_id) REFERENCES collections(id)
);

CREATE TABLE IF NOT EXISTS follows (
  id SERIAL PRIMARY KEY,

  reader_id UUID NOT NULL,
  author_id UUID NOT NULL,
  datetime TIMESTAMP WITH TIME ZONE NOT NULL,

  FOREIGN KEY(reader_id) REFERENCES users(id),
  FOREIGN KEY(author_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS notifications (

)
