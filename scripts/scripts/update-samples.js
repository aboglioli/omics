const fs = require("fs");
const crypto = require("crypto");
const axios = require("axios").default;

const config = require("../core/config");

async function main() {
  console.log("[ SAMPLES ]");
  console.log("Updating...");

  try {
    let comicSamples = [];

    const marvelTs = 1;
    const hash = crypto
      .createHash("md5")
      .update(`${marvelTs}${config.marvelPrivateKey}${config.marvelPublicKey}`)
      .digest("hex");
    const params = `ts=1&apikey=${config.marvelPublicKey}&hash=${hash}`;
    const baseUrl = "http://gateway.marvel.com/v1/public/comics";
    const limit = 100;
    let offset = 0;

    for (let i = 0; i < 3; i++) {
      const { data } = await axios.get(
        `${baseUrl}?${params}&limit=${limit}&offset=${offset}`
      );
      comicSamples.push(...data.data.results);
      offset += limit;
    }

    comicSamples = comicSamples.filter((s) => s.images.length > 0);

    fs.writeFileSync(
      "./comic-samples.json",
      JSON.stringify({
        comics: comicSamples,
      })
    );

    console.log(comicSamples[0]);
    console.log(comicSamples.length);
  } catch (err) {
    console.log(err);
  } finally {
    console.log("READY");
  }
}

main();
