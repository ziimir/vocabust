export function Layout(props: { children: any }) {
    return (
        <html>
            <head>
                <title>Мой сайт</title>
            </head>
            <body>
                <main>{props.children}</main>
            </body>
        </html>
    );
}
