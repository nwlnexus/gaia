import { Hono } from 'hono';
import { cors } from 'hono/cors';
import { Env } from '~/types';
import { D1Orm } from 'd1-orm';

const h = new Hono<{ Bindings: Env }>();
h.use("*", cors());

/// This endpoint is used by devices to first check to see if they are registered
/// and if not the program should register them and associated them with an account.
/// It should look up the serial in the DB and see what account is associated with
/// the device.
h.get('/s/:serial?', async (c) => {
  try{
    const s = c.req.param('serial');
    if (typeof s !== 'undefined' || s !== '') {
      const orm = new D1Orm(c.env.GAIA_DB);
      return c.json({ message: `Hello! You are coming from a device with serial: ${s}` });
    } else {
      return c.json({ message: 'Error occured' }, 400);
    }
  } catch (e) {
    console.log(e);
    return new Response(null);
  }
});

/// This endpoint should be the fall through return should no other route match.
h.notFound(c => {
  return c.text('Oops! There seems to be an issue here. We are having trouble finding what you are looking for.');
});
export default h;
