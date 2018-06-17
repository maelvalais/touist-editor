'use strict';
module.exports = {
  NODE_ENV: '"production"',
  // API_URL won't be set during compile time. Instead, we will have
  // to sed 's/%API_URL%/${API_URL}/' in the webpacked js in dist/ after any
  // build.
};
