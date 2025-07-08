import { Command, makeBus, CommandBus } from '@/core/commands';

import { OxfordSearchCommand } from './commands/OxfordSearchCommand';
import {SearchResult} from './types';

interface SearchUsecaseParams {
    query: string;
    searchType: 'oxford' | 'cambridge';
}
class SearchUsecase implements Command<SearchUsecaseParams, SearchResult> {
    constructor(
        private readonly bus: CommandBus,
    ) {}

    public readonly name = 'SearchUsecaseCommand';

    async execute({ query }: SearchUsecaseParams): Promise<SearchResult> {
        return await this.bus.execute(new OxfordSearchCommand(), { q: query });
    }
}

export function makeSearchUsecase(): SearchUsecase {
    return new SearchUsecase(makeBus());
}
