import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

import express from 'express';
import { initFileRouter } from 'node-file-router';
import { v4 as uuid } from 'uuid';

import { withRequestContext, RequestContext } from '@/core/context';

const __dirname = dirname(fileURLToPath(import.meta.url));

const app = express();
const port = 3000;


app.use((_req, _res, next) => {
    const requestId = uuid();
    const ctx: RequestContext = { requestId };

    withRequestContext(ctx, () => next());
});

const fileRouter = await initFileRouter({ baseDir: resolve(__dirname, './routes') });
app.use(fileRouter);

app.use((err, req, res, next) => {
    console.log(err);
    res.status(500).send('Something broke!');
});

app.listen(port, () => {
    console.log(`Example app listening on port ${port}`);
});
