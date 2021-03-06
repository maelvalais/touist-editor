'use strict';
const merge = require('webpack-merge');
const prodEnv = require('./prod.env');

module.exports = {
  NODE_ENV: '"development"',
  SERVER_HOST: '"' + process.env.SERVER_HOST + '"',
  API_URL: '"http://localhost:8000"',
};
