import {WordSearchResult} from '@/fsd/features/WordSearch';

interface WordSearchFullProps {
    results?: string[];
}

export function WordSearchPartial({ results }: WordSearchFullProps) {
    if (!results || results.length === 0) {
        return null;
    }

    return (
        <WordSearchResult results={results} />
    );
}
