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

INSERT INTO permissions(id, name)
VALUES
  -- Roles
  ('create_role', 'Crear rol'),
  ('delete_role', 'Eliminar rol'),
  ('get_any_role', 'Obtener todos los roles'),
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
  -- Categories
  ('create_category', 'Crear categoría'),
  ('delete_category', 'Eliminar categoría'),
  ('update_category', 'Editar categoría'),
  -- Collections
  ('add_remove_collection_from_favorites', 'Agregar/Quitar colección de favoritos'),
  ('add_remove_publication_from_collection', 'Agregar/Quitar publicación de colección'),
  ('create_collection', 'Crear colección'),
  ('delete_collection', 'Eliminar colección'),
  ('get_publications_from_collection', 'Obtener publicaciones de una colección'),
  ('update_collection', 'Editar colección'),
  -- Publications
  ('add_remove_publication_from_favorites', 'Agregar/Quitar publicación de favoritos'),
  ('approve_reject_publication', 'Aprobar/Rechazar publicación'),
  ('create_publication', 'Crear publicación'),
  ('delete_publication', 'Eliminar publicación'),
  ('get_any_publication', 'Obtener cualquier publicación'),
  ('get_publication_reviews', 'Obtener calificaciones de publicación'),
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

INSERT INTO roles(id, name, permissions, "default", created_at)
VALUES
  (
    'admin',
    'Administrador',
    '[
      { "id": "create_role", "name": "Crear rol" },
      { "id": "delete_role", "name": "Eliminar rol" },
      { "id": "get_any_role", "name": "Obtener todos los roles" },
      { "id": "get_own_role", "name": "Obtener rol propio" },
      { "id": "get_permissions", "name": "Obtener permisos" },
      { "id": "make_role_default", "name": "Cambiar rol por defecto" },
      { "id": "update_role", "name": "Editar rol" },
      { "id": "change_user_password", "name": "Cambiar contraseña" },
      { "id": "change_user_payment_email", "name": "Cambiar dirección de correo para pagos" },
      { "id": "change_user_role", "name": "Cambiar rol de usuario" },
      { "id": "delete_any_user", "name": "Eliminar cualquier usuario" },
      { "id": "delete_own_user", "name": "Eliminar usuario propio" },
      { "id": "get_any_user", "name": "Obtener cualquier usuario" },
      { "id": "get_own_user", "name": "Obtener usuario propio" },
      { "id": "login", "name": "Login" },
      { "id": "recover_user_password", "name": "Recuperar contraseña" },
      { "id": "update_any_user", "name": "Editar cualquier usuario" },
      { "id": "update_own_user", "name": "Editar usuario propio" },
      { "id": "validate_user_account", "name": "Validar cuenta de usuario" },
      { "id": "follow_unfollow_author", "name": "Seguir/Dejar de seguir autor" },
      { "id": "create_category", "name": "Crear categoría" },
      { "id": "delete_category", "name": "Eliminar categoría" },
      { "id": "update_category", "name": "Editar categoría" },
      { "id": "add_remove_collection_from_favorites", "name": "Agregar/Quitar colección de favoritos" },
      { "id": "add_remove_publication_from_collection", "name": "Agregar/Quitar publicación de colección" },
      { "id": "create_collection", "name": "Crear colección" },
      { "id": "delete_collection", "name": "Eliminar colección" },
      { "id": "get_publications_from_collection", "name": "Obtener publicaciones de una colección" },
      { "id": "update_collection", "name": "Editar colección" },
      { "id": "add_remove_publication_from_favorites", "name": "Agregar/Quitar publicación de favoritos" },
      { "id": "approve_reject_publication", "name": "Aprobar/Rechazar publicación" },
      { "id": "create_publication", "name": "Crear publicación" },
      { "id": "delete_publication", "name": "Eliminar publicación" },
      { "id": "get_any_publication", "name": "Obtener cualquier publicación" },
      { "id": "get_publication_reviews", "name": "Obtener calificaciones de publicación" },
      { "id": "like_unlike_publication", "name": "Like/Dislike publicación" },
      { "id": "publish_publication", "name": "Publicar publicación" },
      { "id": "read_publication", "name": "Leer publicación" },
      { "id": "review_publication", "name": "Calificar publicación" },
      { "id": "update_publication", "name": "Editar publicación" },
      { "id": "get_reader", "name": "Obtener lector" },
      { "id": "get_reader_favorites", "name": "Obtener favoritos" },
      { "id": "get_reader_following", "name": "Obtener autores seguidos" },
      { "id": "approve_reject_contract", "name": "Aprobar/Rechazar contrato" },
      { "id": "cancel_contract", "name": "Rescindir contrato" },
      { "id": "charge_for_contract", "name": "Cobrar por contrato" },
      { "id": "generate_summaries_for_contract", "name": "Generar resúmenes para contrato" },
      { "id": "get_any_contract", "name": "Obtener cualquier contrato" },
      { "id": "get_own_contract", "name": "Obtener contrato propio" },
      { "id": "request_contract", "name": "Requerir contrato" },
      { "id": "charge_donations", "name": "Cobrar donaciones" },
      { "id": "donate", "name": "Donar" },
      { "id": "get_any_donation", "name": "Obtener cualquier donación" },
      { "id": "get_own_donation", "name": "Obtener donación propia" },
      { "id": "create_plan", "name": "Crear plan" },
      { "id": "delete_plan", "name": "Eliminar plan" },
      { "id": "update_plan", "name": "Editar plan" },
      { "id": "get_any_subscription", "name": "Obtener cualquier suscripción" },
      { "id": "get_own_subscription", "name": "Obtener suscripción propia" },
      { "id": "subscribe", "name": "Subscribirse" },
      { "id": "get_notifications", "name": "Obtener notificaciones" },
      { "id": "generate_report", "name": "Generar reports" },
      { "id": "change_business_rules", "name": "Editar reglas de negocio" }
    ]',
    FALSE,
    NOW()
  ),
  (
    'content-manager',
    'Gestor de contenido',
    '[
      { "id": "get_any_role", "name": "Obtener todos los roles" },
      { "id": "get_own_role", "name": "Obtener rol propio" },
      { "id": "get_permissions", "name": "Obtener permisos" },
      { "id": "change_user_password", "name": "Cambiar contraseña" },
      { "id": "delete_own_user", "name": "Eliminar usuario propio" },
      { "id": "get_any_user", "name": "Obtener cualquier usuario" },
      { "id": "get_own_user", "name": "Obtener usuario propio" },
      { "id": "login", "name": "Login" },
      { "id": "recover_user_password", "name": "Recuperar contraseña" },
      { "id": "update_own_user", "name": "Editar usuario propio" },
      { "id": "validate_user_account", "name": "Validar cuenta de usuario" },
      { "id": "get_publications_from_collection", "name": "Obtener publicaciones de una colección" },
      { "id": "approve_reject_publication", "name": "Aprobar/Rechazar publicación" },
      { "id": "get_any_publication", "name": "Obtener cualquier publicación" },
      { "id": "get_publication_reviews", "name": "Obtener calificaciones de publicación" },
      { "id": "read_publication", "name": "Leer publicación" },
      { "id": "approve_reject_contract", "name": "Aprobar/Rechazar contrato" },
      { "id": "get_any_contract", "name": "Obtener cualquier contrato" },
      { "id": "get_any_donation", "name": "Obtener cualquier donación" },
      { "id": "get_any_subscription", "name": "Obtener cualquier suscripción" },
      { "id": "get_notifications", "name": "Obtener notificaciones" },
      { "id": "generate_report", "name": "Generar reports" }
    ]',
    FALSE,
    NOW()
  ),
  (
    'user',
    'Usuario',
    '[
      { "id": "get_own_role", "name": "Obtener rol propio" },
      { "id": "get_permissions", "name": "Obtener permisos" },
      { "id": "change_user_password", "name": "Cambiar contraseña" },
      { "id": "change_user_payment_email", "name": "Cambiar dirección de correo para pagos" },
      { "id": "delete_own_user", "name": "Eliminar usuario propio" },
      { "id": "get_own_user", "name": "Obtener usuario propio" },
      { "id": "login", "name": "Login" },
      { "id": "recover_user_password", "name": "Recuperar contraseña" },
      { "id": "update_own_user", "name": "Editar usuario propio" },
      { "id": "validate_user_account", "name": "Validar cuenta de usuario" },
      { "id": "follow_unfollow_author", "name": "Seguir/Dejar de seguir autor" },
      { "id": "add_remove_collection_from_favorites", "name": "Agregar/Quitar colección de favoritos" },
      { "id": "add_remove_publication_from_collection", "name": "Agregar/Quitar publicación de colección" },
      { "id": "create_collection", "name": "Crear colección" },
      { "id": "delete_collection", "name": "Eliminar colección" },
      { "id": "get_publications_from_collection", "name": "Obtener publicaciones de una colección" },
      { "id": "update_collection", "name": "Editar colección" },
      { "id": "add_remove_publication_from_favorites", "name": "Agregar/Quitar publicación de favoritos" },
      { "id": "create_publication", "name": "Crear publicación" },
      { "id": "delete_publication", "name": "Eliminar publicación" },
      { "id": "get_publication_reviews", "name": "Obtener calificaciones de publicación" },
      { "id": "like_unlike_publication", "name": "Like/Dislike publicación" },
      { "id": "publish_publication", "name": "Publicar publicación" },
      { "id": "read_publication", "name": "Leer publicación" },
      { "id": "review_publication", "name": "Calificar publicación" },
      { "id": "update_publication", "name": "Editar publicación" },
      { "id": "get_reader", "name": "Obtener lector" },
      { "id": "get_reader_favorites", "name": "Obtener favoritos" },
      { "id": "get_reader_following", "name": "Obtener autores seguidos" },
      { "id": "cancel_contract", "name": "Rescindir contrato" },
      { "id": "charge_for_contract", "name": "Cobrar por contrato" },
      { "id": "generate_summaries_for_contract", "name": "Generar resúmenes para contrato" },
      { "id": "get_own_contract", "name": "Obtener contrato propio" },
      { "id": "request_contract", "name": "Requerir contrato" },
      { "id": "charge_donations", "name": "Cobrar donaciones" },
      { "id": "donate", "name": "Donar" },
      { "id": "get_own_donation", "name": "Obtener donación propia" },
      { "id": "get_own_subscription", "name": "Obtener suscripción propia" },
      { "id": "subscribe", "name": "Subscribirse" },
      { "id": "get_notifications", "name": "Obtener notificaciones" }
    ]',
    TRUE,
    NOW()
  );

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
