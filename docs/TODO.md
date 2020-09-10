# All

- [x] Add 'me' as ID for user endpoints.
- [x] Check auth permissions for existing use cases.
- [x] AuthorizationService should return user id only. Not load it.
- [ ] Create Author from UserService when a new publication is created.
- [x] Persist events in EventRepository.
- [x] Add API to get events if user is admin.
- [ ] Make /events accessible only in development env.
- [ ] If request is a GET and an error occurs, a 404 error should be returned.
- [x] Split publicaton view and publication read.
- [x] Add Reader preferences.
- [x] Add Reader interaction for a viewed publication. DTO.
- [x] Allow client to specify the 'fields' to get from a request
- [x] Add DateTime and Date support and examples.
- [x] Use Include in use cases returning nested data.
- [ ] User should not include Role. It must include RoleId.
- [x] Add Search use case when necessary.
- [x] ChangePassword should not validate logged in user to allow change a
  temporal password (RecoverPassword).
- [x] Add methods to get events from date in EventRepository.
- [x] Add Option<String> for auth_id in use cases where user does not need to be logged in.
- [x] Make User birthday as value object. Check date ranges.
- [ ] API for add new categories.
- [ ] Use username as user id too.
- [ ] Add 'biography' to Author.
- [ ] Add favorite interactions for publications and collections.
- [ ] Refactor interactions and events.
- [ ] Search by tags for publications and collections.
- [ ] Add datetimes in DTOs.
- [ ] Add Author followers.
- [ ] Call base.update() and base.delete() in each AggregateRoot.
- [ ] Consider deleted_at in repositories to not return deleted entities.
- [ ] Add next_id() implementation in trait of repositories.
- [ ] Add 'username' as ID of user.
- [ ] Separate events from AggregateRoot.
- [ ] Move publishing::UserService to shared crate.

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
