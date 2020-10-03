const axios = require('axios');

const baseUrl = 'http://localhost:3000/api';

const readUrl = (url = '') =>
  url.startsWith('http://') || url.startsWith('https://') ? url : `${baseUrl}${url}`;

class Request {
  constructor() {}

  async login(username, password) {
    let res = await this.post('/login', { username, password });
    res = res.data;

    this.userId = res.user_id;
    this.authToken = res.auth_token;
  }

  logout() {
    this.userId = null;
    this.authToken = null;
  }

  async get(url = '', headers = {}) {
    if (this.authToken) {
      headers = {
        'Authorization': `Bearer ${this.authToken}`,
        ...headers,
      };
    }

    return axios.get(readUrl(url), {
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
        ...headers,
      }
    });
  }

  async post(url = '', body = {}, headers = {}) {
    if (this.authToken) {
      headers = {
        Authorization: `Bearer ${this.authToken}`,
        ...headers,
      };
    }

    return axios.post(readUrl(url), body, {
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
        ...headers,
      }
    });
  }

  async put(url = '', body = {}, headers = {}) {
    if (this.authToken) {
      headers = {
        Authorization: `Bearer ${this.authToken}`,
        ...headers,
      };
    }

    return axios.put(readUrl(url), body, {
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
        ...headers,
      }
    });
  }

  async delete(url = '', headers = {}) {
    if (this.authToken) {
      headers = {
        Authorization: `Bearer ${this.authToken}`,
        ...headers,
      };
    }

    return axios.delete(readUrl(url), {
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
        ...headers,
      }
    });
  }

  events() {
    return this.get('/events');
  }

  async lastEvent() {
    const { data: { events } } = await this.events();
    return events[events.length - 1];
  }
}

const req = new Request();

module.exports = {
  Request,
  req,
};
