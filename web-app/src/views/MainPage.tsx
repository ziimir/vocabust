import { routes } from '@/config/routes';

import { Layout } from './Layout';

export function MainPage(props: { query: string; results: string[] }) {
    return (
        <Layout>
            <h1>Поиск</h1>
            <form method="get" action={routes.search()}>
                <input name="q" value={props.query} />
                <button type="submit">Искать</button>
            </form>
        </Layout>
    );
}
