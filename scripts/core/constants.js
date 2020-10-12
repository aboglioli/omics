const slugify = require("slugify");

// Utils
const image = (size) =>
  `https://via.placeholder.com/${size ? size : "256"}.jpg`;
const password = "$2y$12$nPMNHiYhXb90lZTu0CX.2eY5RIQ/Uek28lCua23OIfkLhcjZtnIIO";

// Constants
const genders = ["male", "female", "other"];
const categories = [
  "action",
  "adventure",
  "comedy",
  "crime",
  "drama",
  "fantasy",
  "historical",
  "horror",
  "mystery",
  "romance",
  "science-fiction",
  "thriller",
];
const tags = [
  "Increíble",
  "Suspenso",
  "Emocionante",
  "Etiqueta genérica",
  "Hola",
  "Chau",
].map((tag) => ({
  slug: slugify(tag, { lower: true }),
  name: tag,
}));

module.exports = {
  image,
  password,
  genders,
  categories,
  tags,
};
