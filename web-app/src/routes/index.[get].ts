import { Request, Response } from 'express';

import {renderHtml} from '@/core/renderers/renderHtml';
import {MainPage} from '@/views/MainPage';

export default async (_req: Request, res: Response) => {
    try {
        res.send(renderHtml(MainPage));
    } catch (error) {
        res.status(500).send('Internal Error');
    }
};
