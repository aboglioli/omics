# API

## Identity
- [ ] GET /roles
- [ ] GET /roles/:id?include=users
- [ ] GET /roles/:id/users

- [ ] POST /register
- [ ] POST /login
- [ ] POST /recover-password
- [ ] GET /users
- [ ] GET,PUT,DELETE /users/:id
- [ ] PUT /users/:id/password
- [ ] GET /users/:id/validate/:code
- [ ] GET,PUT /users/:id/role

- [ ] POST /users/callback

## Publishing
- [ ] GET /authors?include=publications,collections,followers,publications.category,collections.category
- [ ] GET /authors/:id
- [ ] GET /authors/:id/publications
- [ ] GET /authors/:id/collections
- [ ] GET /authors/:id/followers
- [ ] POST /authors/:id/follow

- [ ] GET /readers?include=following,favorites
- [ ] GET /readers/:id
- [ ] GET /readers/:id/following
- [ ] GET /readers/:id/favorites

- [ ] GET /collections?include=author,category,publications,publications.author,publications.category
- [ ] GET /collections/:id
- [ ] GET /collections/:id/publications
- [ ] POST /collections
- [ ] PUT,DELETE /collections/:id
- [ ] POST,DELETE /collections/:id/publication/:publicationId

- [ ] GET /categories?include=publications,collections
- [ ] POST /categories
- [ ] GET /categories/:id
- [ ] PUT /categories/:id
- [ ] GET /categories/:id/publications
- [ ] GET /categories/:id/collections

- [ ] GET,POST /publications
- [ ] GET /publications/:id?include=author,category
- [ ] PUT /publications/:id
- [ ] GET /publications/:id/author
- [ ] GET /publications/:id/category
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
