export function Layout(props: { children: any }) {
    return (
        <html>
            <head>
                <title>Мой сайт</title>
                <script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.6/dist/htmx.min.js"></script>
            </head>
            <body hx-boost="true">
                <main>{props.children}</main>
            </body>
        </html>
    );
}
