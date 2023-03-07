import { D1Database, KVNamespace, Queue } from '@cloudflare/workers-types';

export type Env = {
  GAIA_KV: KVNamespace;
  GAIA_DB: D1Database;
  GAIA_QUEUE: Queue;
};
