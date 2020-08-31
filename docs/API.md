# API

## Identity
- [x] GET /roles ([]Role, admin)
- [x] GET /roles/:id (Role, admin)
- [x] GET /roles/:id/users ([]User, admin)

- [x] POST /register
- [x] POST /login
- [x] POST /recover-password

- [x] GET /users?include=role ([]User, admin)
- [x] GET /users/:id?include=role (User, owner|admin)
- [x] PUT /users/:id (owner|admin)
- [ ] DELETE /users/:id (owner|admin)
- [x] PUT /users/:id/password (owner|admin)
- [x] GET /users/:id/validate/:code
- [x] PUT /users/:id/role (admin)

- [ ] POST /users/callback

## Publishing
- [x] GET /authors?name=... ([]Author)
- [x] GET /authors/:id (Author)
- [x] GET /authors/:id/publications?include=category ([]Publication)
- [x] GET /authors/:id/collections?include=category ([]Collection)
- [ ] GET /authors/:id/followers ([]Reader)
- [x] POST /authors/:id/follow

- [ ] GET /readers/:id (Reader)
- [ ] GET /readers/:id/following ([]Author)
- [ ] GET /readers/:id/favorites ([]Publication)

- [x] GET /collections?name=...&include=author,category ([]Collection)
- [x] GET /collections/:id?include=author,category (Collection)
- [x] GET /collections/:id/publications?include=author,category ([]Publication)
- [x] POST /collections
- [x] PUT /collections/:id
- [x] DELETE /collections/:id
- [x] POST,DELETE /collections/:id/publication/:publicationId

- [x] GET /categories ([]Category)
- [x] GET /categories/:id (Category)
- [x] GET /categories/:id/publications?include=author ([]Publication)
- [x] GET /categories/:id/collections?include=author ([]Collection)
- [x] POST /categories
- [x] PUT /categories/:id

- [x] GET /publications?status=...&name=...&include=author,category ([]Publication)
- [x] GET /publications/:id?include=author,category (Publication)
- [x] GET /publications/:id/collections?include=author,category ([]Collection)
- [x] POST /publications
- [x] PUT /publications/:id

- [ ] GET /publications/:id/read
- [ ] POST /publications/:id/publish
- [ ] POST /publications/:id/approve
- [ ] POST /publications/:id/reject
- [ ] POST /publications/:id/like
- [ ] POST /publications/:id/unlike
- [ ] POST,DELETE /publications/:id/review
- [ ] GET /publications/:id/reviews
- [ ] POST,DELETE /publications/:id/favorite

- [ ] GET /catalogue
- [ ] GET /search?q=<search>

## Subscriptions
- [ ] GET,POST /subscriptions
- [ ] GET,PUT,DELETE /subscriptions/:id
- [ ] GET /reader/:id/subscription

- [ ] POST /subscriptions/callback

## Contract
- [ ] GET /contracts
- [ ] GET,POST /publications/:id/contracts
- [ ] POST /contracts/:id/approve
- [ ] POST /contracts/:id/reject
- [ ] POST /contracts/:id/cancel

## Notifications
- [ ] GET /notifications
- [ ] GET /users/:id/notifications

## Events
- [ ] GET /events
