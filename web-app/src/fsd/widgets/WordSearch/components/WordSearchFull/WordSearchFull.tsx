import {WordSearch} from '@/fsd/features/WordSearch';

import {WordSearchPartial} from '../WordSearchPartial/WordSearchPartial';

interface WordSearchFullProps {
    dataName?: string;
    formAction: string;
    query?: string;
    results?: string[];
}

export function WordSearchFull({
    dataName = 'WordSearchFull',
    formAction,
    query,
    results,
}: WordSearchFullProps) {
    const resultDataName = `${dataName}.Result`;
    return (
        <>
            <WordSearch formAction={formAction} query={query} hxTarget={resultDataName} />
            <div data-id={resultDataName}>
                <WordSearchPartial results={results} />
            </div>
        </>
    );
}
