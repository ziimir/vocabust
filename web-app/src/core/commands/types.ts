export interface Command<I, O> {
    name: string;

    execute(input: I): Promise<O>;
}

export type CommandMiddleware = <I, O>(
    command: Command<I, O>,
    input: I,
    next: (replaceWith?: Command<I, O>) => Promise<O>,
) => Promise<O>;
