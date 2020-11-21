INSERT INTO cache(key, value)
VALUES (
  'business_rules',
  '{
    "days_to_generate_summaries": 10,
    "donation_percentage_retention": 0.3,
    "minimum_charge_amount": 200.0,
    "minimum_donation_amount": 50.0,
    "minimum_views_percentage_to_require_contract": 0.01,
    "subscription_percentage_retention": 0.7
  }'
);

INSERT INTO roles(id, name, created_at)
VALUES
  ('admin', 'Administrador', NOW()),
  ('content-manager', 'Gestor de contenido', NOW()),
  ('user', 'Usuario', NOW());

INSERT INTO permissions(id, name)
VALUES
  -- Roles
  ('create_role', 'Crear rol'),
  ('delete_role', 'Eliminar rol'),
  ('get_all_roles', 'Obtener todos los roles'),
  ('get_own_role', 'Obtener rol propio'),
  ('get_permissions', 'Obtener permisos'),
  ('make_role_default', 'Cambiar rol por defecto'),
  ('update_role', 'Editar rol'),
  -- Users
  ('change_user_password', 'Cambiar contraseña'),
  ('change_user_payment_email', 'Cambiar dirección de correo para pagos'),
  ('change_user_role', 'Cambiar rol de usuario'),
  ('delete_any_user', 'Eliminar cualquier usuario'),
  ('delete_own_user', 'Eliminar usuario propio'),
  ('get_any_user', 'Obtener cualquier usuario'),
  ('get_own_user', 'Obtener usuario propio'),
  ('login', 'Login'),
  ('recover_user_password', 'Recuperar contraseña'),
  ('update_any_user', 'Editar cualquier usuario'),
  ('update_own_user', 'Editar usuario propio'),
  ('validate_user_account', 'Validar cuenta de usuario'),
  -- Authors
  ('follow_unfollow_author', 'Seguir/Dejar de seguir autor'),
  ('get_any_author', 'Obtener cualquier autor'),
  ('get_own_author', 'Obtener autor propio'),
  -- Categories
  ('create_category', 'Crear categoría'),
  ('delete_category', 'Eliminar categoría'),
  ('get_categories', 'Obtener categorías'),
  ('update_category', 'Editar categoría'),
  -- Collections
  ('add_remove_collection_from_favorites', 'Agregar/Quitar colección de favoritos'),
  ('add_remove_publication_from_collection', 'Agregar/Quitar publicación de colección'),
  ('create_collection', 'Crear colección'),
  ('delete_collection', 'Eliminar colección'),
  ('get_any_collection', 'Obtener cualquier colección'),
  ('get_own_collection', 'Obtener colección propia'),
  ('get_publications_from_collection', 'Obtener publicaciones de una colección'),
  ('update_collection', 'Editar colección'),
  -- Publications
  ('add_remove_publication_from_favorites', 'Agregar/Quitar publicación de favoritos'),
  ('approve_reject_publication', 'Aprobar/Rechazar publicación'),
  ('create_publication', 'Crear publicación'),
  ('delete_publication', 'Eliminar publicación'),
  ('get_any_publication', 'Obtener cualquier publicación'),
  ('get_own_publication', 'Obtener publicación propia'),
  ('get_publication_reviews', 'Obtener calificaciones de publicación'),
  ('get_unpublished_publications', 'Obtener publicaciones no publicadas'),
  ('like_unlike_publication', 'Like/Dislike publicación'),
  ('publish_publication', 'Publicar publicación'),
  ('read_publication', 'Leer publicación'),
  ('review_publication', 'Calificar publicación'),
  ('update_publication', 'Editar publicación'),
  -- Readers
  ('get_reader', 'Obtener lector'),
  ('get_reader_favorites', 'Obtener favoritos'),
  ('get_reader_following', 'Obtener autores seguidos'),
  -- Contracts
  ('approve_reject_contract', 'Aprobar/Rechazar contrato'),
  ('cancel_contract', 'Rescindir contrato'),
  ('charge_for_contract', 'Cobrar por contrato'),
  ('generate_summaries_for_contract', 'Generar resúmenes para contrato'),
  ('get_any_contract', 'Obtener cualquier contrato'),
  ('get_own_contract', 'Obtener contrato propio'),
  ('request_contract', 'Requerir contrato'),
  -- Donations
  ('charge_donations', 'Cobrar donaciones'),
  ('donate', 'Donar'),
  ('get_any_donation', 'Obtener cualquier donación'),
  ('get_own_donation', 'Obtener donación propia'),
  -- Plans
  ('create_plan', 'Crear plan'),
  ('delete_plan', 'Eliminar plan'),
  ('update_plan', 'Editar plan'),
  -- Subscriptions
  ('get_any_subscription', 'Obtener cualquier suscripción'),
  ('get_own_subscription', 'Obtener suscripción propia'),
  ('subscribe', 'Subscribirse'),
  -- Notifications
  ('get_notifications', 'Obtener notificaciones'),
  -- Reports
  ('generate_report', 'Generar reports'),
  -- Configuration
  ('change_business_rules', 'Editar reglas de negocio');


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
