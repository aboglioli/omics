# API

## Identity
- [ ] GET /roles
- [ ] GET /roles/:id
- [ ] GET /roles/:id/users

- [ ] POST /register
- [ ] POST /login
- [ ] POST /recover-password
- [ ] GET /users?include=role
- [ ] GET,PUT,DELETE /users/:id
- [ ] PUT /users/:id/password
- [ ] GET /users/:id/validate/:code
- [ ] GET,PUT /users/:id/role

- [ ] POST /users/callback

## Publishing
- [ ] GET /authors?include=publications,collections,followers,publications.category,collections.category&name=...
- [ ] GET /authors/:id?include=publications,collections,followers,publications.category,collections.category
- [ ] POST /authors/:id/follow

- [ ] GET /readers/:id?include=following,favorites

- [ ] GET /collections?include=author,category,publications,publications.author,publications.category&author_id=...&publication_id=...
- [ ] GET /collections/:id?include=author,category,publications,publications.author,publications.category
- [ ] POST /collections
- [ ] PUT,DELETE /collections/:id
- [ ] POST,DELETE /collections/:id/publication/:publicationId

- [ ] GET /categories?include=publications,collections,publications.author,collections.author
- [ ] GET /categories/:id?include=publications,collections,publications.author,collections.author
- [ ] POST /categories
- [ ] PUT /categories/:id

- [ ] GET /publications?include=author,category&author_id=...&category_id=...&collection_id=...&status=...&name=...
- [ ] GET /publications/:id?include=author,category
- [ ] POST /publications
- [ ] PUT /publications/:id

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
