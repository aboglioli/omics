# ---
# Build stage
# ---
FROM node:latest AS build

WORKDIR /src
COPY . .

RUN npm install
RUN npm run build

# ---
# Final stage
# ---
FROM nginx:alpine

EXPOSE 4000

COPY nginx.conf /etc/nginx/nginx.conf
COPY --from=build /src/dist/web /usr/share/nginx/html
