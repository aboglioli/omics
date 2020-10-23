INSERT INTO configuration (key, value)
VALUES
  ('days_to_generate_summaries', '10'),
  ('donation_percentage_retention', '0.3'),
  ('minimum_charge_amount', '200.0'),
  ('minimum_donation_amount', '50.0'),
  ('minimum_views_percentage_to_require_contract', '0.01'),
  ('subscription_percentage_retention', '0.7');

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
)
VALUES
  (
    '00000000-0000-0000-0000-000000000001',
    'local',
    'admin-1',
    'admin-1@omics.com',
    '$2y$12$nPMNHiYhXb90lZTu0CX.2eY5RIQ/Uek28lCua23OIfkLhcjZtnIIO',
    'Admin',
    'Omics',
    '1994-08-01 15:30:00',
    'other',
    'I am a administrator.',
    'https://via.placeholder.com/200.jpg',
    'admin',
    NULL,
    NOW(),
    NULL,
    NULL
  ),
  (
    '00000000-0000-0000-0000-000000000002',
    'local',
    'content-manager-1',
    'content-manager-1@omics.com',
    '$2y$12$nPMNHiYhXb90lZTu0CX.2eY5RIQ/Uek28lCua23OIfkLhcjZtnIIO',
    'Content Manager',
    'Omics',
    '1994-08-01 15:30:00',
    'other',
    'I am a content manager.',
    'https://via.placeholder.com/200.jpg',
    'content-manager',
    NULL,
    NOW(),
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

INSERT INTO publication_status(id)
VALUES
  ('draft'),
  ('waiting-approval'),
  ('published'),
  ('rejected');

INSERT INTO plans(id, name, description, price, created_at) VALUES (
  'basic',
  'Plan Básico',
  'Accedé a todo el catálogo de Omics.',
  75.0,
  NOW()
);
