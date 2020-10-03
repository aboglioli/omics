const { client } = require('./db')

function keyValue(obj) {
  return Object
    .keys(obj)
    .map((key, i) => `${key} = \$${i + 1}`);
}

function objToInsert(table, obj) {
  const props = Object.keys(obj).join(', ');
  const params = Object.keys(obj).map((_, i) => `\$${i + 1}`).join(', ');

  return `INSERT INTO ${table}(${props}) VALUES(${params})`;
}

function objToUpdate(table, obj, id) {
  const whereParams = keyValue(id);
  const props = Object
    .keys(obj)
    .map((key, i) => `${key} = \$${i + 1 + whereParams.length}`)
    .join(', ');

  return `UPDATE ${table} SET ${props} WHERE ${whereParams.join(' AND ')}`;
}

function insert(table, obj) {
  const sql = objToInsert(table, obj);

  const values = Object.values(obj).map(value => {
    if (typeof value === 'object' || Array.isArray(value)) {
      return JSON.stringify(value);
    }

    return value;
  });

  return client.query(sql, values);
}

module.exports = {
  keyValue,
  objToInsert,
  objToUpdate,
};
