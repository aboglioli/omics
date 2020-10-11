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
};

// const randDate = (start, end, startHour, endHour) => {
//   const date = new Date(+start + Math.random() * (end - start));
//   const hour = startHour + Math.random() * (endHour - startHour) | 0;
//   date.setHours(hour);
//   return date;
// }
//
// const randDate = () => {
//   const month = rand(1, 13).toString().padStart(2, '0'); // 01-12
//   const day = rand(1, 29).toString().padStart(2, '0'); // 01-28
//   const hour = rand(0, 24).toString().padStart(2, '0'); // 00-23
//   const minute = rand(0, 60).toString().padStart(2, '0'); // 00-59
//
//   // return `2020-${month}-${day}T${hour}:${minute}:00Z`;
//   return `2020-${month}-${day} ${hour}:${minute}:00Z`;
// };

module.exports = {
  sleep,
  rand,
  randArr,
};
