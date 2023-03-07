import { ExecutionContext } from '@cloudflare/workers-types';
import { Env } from '~/types';
import h from '~/api';

export default {
  async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
    return h.fetch(request, env, ctx);
  }
};
