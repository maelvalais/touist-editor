const VERSION = '0.1';
const isDev = process.env.NODE_ENV !== 'production';
const apiUrl = process.env.API_URL || '/';

export default {
  isDev,
  version: `v${VERSION}${isDev ? '-dev' : ''}`,
  solvers: ['sat', 'qbf'],
  apiUrl: apiUrl.replace(/\/$/, ''),
};
