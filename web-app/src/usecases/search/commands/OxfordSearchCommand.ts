import { Command } from '@/core/commands';

import { SearchQuery, SearchResult } from '../types';

export class OxfordSearchCommand implements Command<SearchQuery, SearchResult> {
    constructor() {}

    public readonly name = 'OxfordSearchCommand';

    async execute(input: SearchQuery): Promise<SearchResult> {
        return {
            input: input.q,
            definitions: ['one', 'two', 'go'],
        };
    }
}
