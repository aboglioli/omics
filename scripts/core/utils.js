const sleep = ms => new Promise((resolve, _) => {
  setTimeout(() => {
    resolve();
  }, ms);
});

const rand = (min, max) => Math.floor(Math.random() * (max - min)) + min;
const randArr = (arr, multiple = false) => {
  if (!multiple) {
    return arr[rand(0, arr.length)];
  }

  return arr.reduce((acc, el) => {
    if (rand(0, 100) < 50) {
      return [...acc, el];
    }

    return acc;
  }, []);
}

module.exports = {
    sleep,
    rand,
    randArr,
};
