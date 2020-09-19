INSERT INTO roles(id, name, created_at)
VALUES
  ('admin', 'Administrador', NOW()),
  ('content-manager', 'Gestor de contenido', NOW()),
  ('user', 'Usuario', NOW());

INSERT INTO users(
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
  validation_code,
  created_at,
  updated_at,
  deleted_at
) VALUES (
  '00000000-0000-0000-0000-000000000001',
  'local',
  'admin',
  'admin@omics.com',
  '$2y$12$nPMNHiYhXb90lZTu0CX.2eY5RIQ/Uek28lCua23OIfkLhcjZtnIIO',
  'Admin',
  'Omics',
  '1994-08-01 15:30:00',
  'other',
  'I am the administrator.',
  'https://via.placeholder.com/200.jpg',
  'admin',
  NULL,
  '2020-09-18 23:40:10',
  NULL,
  NULL
);

INSERT INTO categories(id, name, created_at)
VALUES
  ('action','Acción', NOW()),
  ('adventure','Aventura', NOW()),
  ('comedy','Comedia', NOW()),
  ('crime','Crimen', NOW()),
  ('drama','Drama', NOW()),
  ('fantasy','Fantasía', NOW()),
  ('historical','Histórico', NOW()),
  ('horror','Terror', NOW()),
  ('mystery','Misterio', NOW()),
  ('romance','Romance', NOW()),
  ('science-fiction','Ciencia Ficción', NOW()),
  ('thriller','Thriller', NOW());
