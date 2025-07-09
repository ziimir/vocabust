interface WordSearchResultProps {
    results: string[];
}

export function WordSearchResult({ results = [] }: WordSearchResultProps) {
    return <ul>{results.map((r) => (<li>{r}</li>))}</ul>;
}
