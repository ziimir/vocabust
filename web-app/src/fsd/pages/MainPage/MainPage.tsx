import { routes } from '@/config/routes';

import { Layout } from '@/fsd/shared/ui/Layout';
import {WordSearchFull} from '@/fsd/widgets/WordSearch';

interface MainPageProps {
    query?: string;
    results?: string[];
}

export function MainPage({query, results}: MainPageProps) {
    return (
        <Layout>
            <h1>Поиск</h1>
            <WordSearchFull formAction={routes.search()} query={query} results={results} />
        </Layout>
    );
}
