import { unstable_dev } from 'wrangler';
import type { UnstableDevWorker } from 'wrangler';
import { describe, expect, it, beforeAll, afterAll } from 'vitest';

describe('Worker', () => {
  let worker: UnstableDevWorker;

  beforeAll(async () => {
    worker = await unstable_dev('src/index.ts', {
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      //@ts-ignore
      experimental: { disableExperimentalWarning: true }
    });
  });

  afterAll(async () => {
    await worker.stop();
  });

  it('should return HTML on the root', async () => {
    const resp = await worker.fetch('/');
    if (resp) {
      const contentType = resp.headers.get('content-type');
      expect(contentType).toBe('text/plain;charset=UTF-8');
    }
  });
});