import { Request, Response } from 'express';

import { appCommandBus } from '@/core/commands';
import {RenderHtmlCommand} from '@/core/commands';

import { makeSearchUsecase } from '@/usecases/search';
import {Search} from '@/views/Search';

export default async (_req: Request, res: Response) => {
    try {
        const usecase = makeSearchUsecase();
        const renderPage = new RenderHtmlCommand(Search);

        const {input, definitions} = await appCommandBus.execute(usecase, { query: 'word', searchType: 'oxford' });
        const html = await appCommandBus.execute(renderPage, {query: input, results: definitions});

        res.send(html);
    } catch (error) {
        console.error(error)
        res.status(500).send('Internal Error');
    }
};
