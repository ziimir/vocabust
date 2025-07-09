interface WordSearchProps {
    formAction: string;
    hxTarget?: string;
    query?: string;
}

export function WordSearch({ formAction, hxTarget, query = '' }: WordSearchProps) {
    return (
        <form method="get" action={formAction} hx-target={`[data-id="${hxTarget}"]`}>
            <input name="q" value={query} />
            <button type="submit">Искать</button>
        </form>
    );
}
