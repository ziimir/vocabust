import { Request, Response } from 'express';

import { appCommandBus } from '@/core/commands';
import {RenderHtmlCommand} from '@/core/commands';
import {MainPage} from '@/fsd/pages/MainPage';

export default async (req: Request, res: Response) => {
    try {
        const renderPage = new RenderHtmlCommand(MainPage);
        const html = await appCommandBus.execute(renderPage, {});

        res.send(html);
    } catch (error) {
        res.status(500).send('Internal Error');
    }
};
