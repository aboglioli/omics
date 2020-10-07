# All

# Catalog
Catálogo endpoint:
- Más vistas
- Nuevas (últimas)
- Más likeadas
- Mejor calificadas
- Recomendadas

Filtros:
- Desde
- Hast

## Latest
- [ ] Add filters to publication Search.
- [ ] Returns ID after requesting contract.

## Urgent
- [x] Publication status should be an object in DTO to show comments.
- [x] Get collections that a publication is in.
- [x] Add 'me' as ID for user endpoints.
- [x] Check auth permissions for existing use cases.
- [x] AuthorizationService should return user id only. Not load it.
- [x] Persist events in EventRepository.
- [x] Add API to get events if user is admin.
- [x] Split publicaton view and publication read.
- [x] Add Reader preferences.
- [x] Add Reader interaction for a viewed publication. DTO.
- [x] Allow client to specify the 'fields' to get from a request
- [x] Add DateTime and Date support and examples.
- [x] Use Include in use cases returning nested data.
- [x] User should not include Role. It must include RoleId.
- [x] Add Search use case when necessary.
- [x] ChangePassword should not validate logged in user to allow change a
  temporal password (RecoverPassword).
- [x] Add methods to get events from date in EventRepository.
- [x] Add Option<String> for auth_id in use cases where user does not need to be logged in.
- [x] Make User birthday as value object. Check date ranges.
- [x] API for add new categories.
- [x] Add 'biography' to Author.
- [x] Add favorite interactions for publications and collections.
- [x] Refactor interactions and events.
- [x] Add datetimes in DTOs.
- [x] Add Author followers.
- [x] Call base.update() and base.delete() in each AggregateRoot.
- [x] Add next_id() implementation in trait of repositories.
- [x] Separate events from AggregateRoot.
- [x] Move publishing::UserService to shared crate.
- [x] Add entpoint for /publications/:id/collections (collections having the given publication).
- [x] Redirect after user validation.
- [x] Recover password: send email.

- [ ] Create Author from UserService when a new publication is created.
- [ ] Make /events accessible only in development env.
- [x] If request is a GET and an error occurs, a 404 error should be returned.
- [ ] Use username as user id too.
- [ ] Add 'username' as ID of user.
- [ ] Search by tags for publications and collections.
- [ ] Consider deleted_at in repositories to not return deleted entities.
- [ ] Allow change 'username' and 'email' in IdentityService.
- [ ] Merge 'code' with 'topic' in Event.
- [ ] Add Uuid as base id (StringId for string id only).
- [x] Add timestamps to events.
- [x] Improve enum serializatiion/deserialization.
- [ ] Replace rename of each enum for serde by rename_all = "kebab-case".
- [ ] Serialize directly (without util) in shared::event.

# Progress

## Módulo de Registro y Actividad de Usuario 
- [x] CU 01 - Registrar Cuenta de Usuario 
- [x] CU 02 - Iniciar Sesión 
- [x] CU 03 - Recuperar Contraseña 
- [x] CU 04 - Cambiar Contraseña 
- [x] CU 05 - Editar Cuenta de Usuario 
- [x] CU 06 - Ver Perfil de Usuario 
- [x] CU 07 - Desactivar Cuenta de Usuario  

## Módulo de Catálogo y Búsqueda 
- [x] CU 08 - Buscar y Filtrar Contenido 
- [x] CU 09 - Ver Detalle de Contenido 
- [x] CU 10 - Ver Favoritos  

## Módulo de Visualización y Seguimiento 
- [x] CU 11 - Leer Publicación 
- [x] CU 12 - Añadir y Eliminar Contenido de Favoritos 
- [x] CU 13 - Seguir/Dejar de Seguir a Usuario 
 
## Módulo de Reseñas 
- [x] CU 14 - Calificar Publicación 
- [x] CU 15 - Dar Like a Publicación  

## Módulo de Suscripción y Donación 
- [ ] CU 16 - Donar a Autor 
- [ ] CU 17 - Realizar Suscripción 
- [ ] CU 18 - Cancelar Suscripción 
- [ ] CU 19 - Pagar Suscripción  

## Módulo de Publicación de Obras 
- [x] CU 20 - Publicar Obra 
- [x] CU 21 - Modificar Publicación 
- [x] CU 22 - Gestionar Colecciones 
- [x] CU 23 - Aceptar/Rechazar Publicación 
- [x] CU 24 - Ver Estadísticas de Publicaciones  

## Módulo de Notificaciones 
- [ ] CU 25 - Notificar Actividad de Autores y Contenido 
- [ ] CU 26 - Notificar Resumen de Pago 

## Módulo de Contrato 
- [ ] CU 27 - Requerir Contrato 
- [ ] CU 28 - Rescindir Contrato 
- [ ] CU 29 - Cobrar por Resumen de Publicación 
- [ ] CU 30 - Generar Resumen de Publicación 
- [ ] CU 31 - Aceptar/Rechazar Contrato  

## Módulo de Reportes 
- [ ] CU 32 - Generar y Visualizar Reportes 

## Módulo de Configuración
- [ ] CU 33 - Configurar Reglas de Negocio  
- [ ] CU 34 - Gestionar Categorías y Etiquetas 
- [ ] CU 35 - Gestionar Estados 
