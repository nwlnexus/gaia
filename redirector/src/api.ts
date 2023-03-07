import { Hono } from "hono";

const h = new Hono();

h.get("/", (c) => {
    return c.text("Hello!");
});

export default h;