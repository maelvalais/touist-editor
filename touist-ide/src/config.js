const VERSION = '0.1';
const isDev = process.env.NODE_ENV !== 'production';
const api_url = process.env.API_URL || '/';

export default {
  isDev,
  version: `v${VERSION}${isDev ? '-dev' : ''}`,
  solvers: ['sat', 'qbf'],
  api_url: api_url.replace(/\/$/, ''),
};
