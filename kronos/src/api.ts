import { Hono } from 'hono';
import { Env } from '~/types';

const h = new Hono<{ Bindings: Env }>();
h.get('/s/:serial?', c => {
  const s = c.req.param('serial');
  if (typeof s !== 'undefined' || s !== '') {
    return c.json({ message: `Hello! You are coming from a device with serial: ${s}` });
  } else {
    return c.json({ message: 'Error occured' }, 400);
  }
});

h.notFound(c => {
  return c.text('Oops! There seems to be an issue here. We are having trouble finding what you are looking for.');
});
export default h;
