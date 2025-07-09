import { Request, Response } from 'express';

import { appCommandBus } from '@/core/commands';
import {RenderHtmlCommand} from '@/core/commands';
import { makeSearchUsecase } from '@/usecases/search';
import {MainPage} from '@/fsd/pages/MainPage';
import {WordSearchPartial} from '@/fsd/widgets/WordSearch';

export default async (req: Request, res: Response) => {
    try {
        const usecase = makeSearchUsecase();
        const {input, definitions} = await appCommandBus.execute(usecase, { query: 'word', searchType: 'oxford' });

        if (req.headers['hx-request']) {
            const renderPage = new RenderHtmlCommand(WordSearchPartial);
            const html = await appCommandBus.execute(renderPage, {results: definitions});
            res.send(html);
            return;
        }

        const renderPage = new RenderHtmlCommand(MainPage);
        const html = await appCommandBus.execute(renderPage, {query: input, results: definitions});

        res.send(html);
    } catch (error) {
        console.error(error)
        res.status(500).send('Internal Error');
    }
};
